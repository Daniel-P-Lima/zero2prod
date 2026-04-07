use actix_web::{HttpResponse, web::Data, web::Form};
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use tracing::Instrument;
use uuid::Uuid;
use unicode_segmentation::UnicodeSegmentation;

#[derive(serde::Deserialize)]
pub struct FormData {
    name: String,
    email: String,
}

#[derive(serde::Serialize)]
pub struct Subscriber {
    id: Uuid,
    email: String,
    name: String,
    subscribed_at: DateTime<Utc>,
}

#[derive(serde::Serialize)]
pub struct SubscribersResponse {
    count: usize,
    subscribers: Vec<Subscriber>,
}

pub async fn subscribe(form: Form<FormData>, pool: Data<PgPool>) -> HttpResponse {
    if !is_valid_name(&form.name) {
        return HttpResponse::BadRequest().finish();
    }
    let request_id = Uuid::new_v4();
    let request_span = tracing::info_span!("Adding a new subscriber",
        %request_id,
        subscriber_email = %form.email ,
        subscriber_name = %form.name
    );
    let _request_span_guard = request_span.enter();

    let query_span = tracing::info_span!("Saving new subscriber details in the database");

    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(pool.get_ref())
    .instrument(query_span)
    .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            tracing::error!(
                "request_id {} - Failed to save new subscriber details! Query error: {:?}",
                &request_id,
                e
            );
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn get_all_subscribers(pool: Data<PgPool>) -> HttpResponse {
    match sqlx::query_as!(Subscriber, "SELECT * FROM subscriptions")
        .fetch_all(pool.get_ref())
        .await
    {
        Ok(values) => {
            let response = SubscribersResponse {
                count: values.len(),
                subscribers: values,
            };
            HttpResponse::Ok().json(response)
        }
        Err(e) => {
            println!("Error {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
pub fn is_valid_name(s: &str) -> bool {
    // `.trim()` returns a view over the input `s` without trailing
    // whitespace-like characters.
    // `.is_empty` checks if the view contains any character.
    let is_empty_or_whitespace = s.trim().is_empty();
    // A grapheme is defined by the Unicode standard as a "user-perceived"
    // character: `å` is a single grapheme, but it is composed of two characters
    // (`a` and `̊`).
    //
    // `graphemes` returns an iterator over the graphemes in the input `s`.
    // `true` specifies that we want to use the extended grapheme definition set,
    // the recommended one.
    let is_too_long = s.graphemes(true).count() > 256;
    // Iterate over all characters in the input `s` to check if any of them matches
    // one of the characters in the forbidden array.
    let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
    let contains_forbidden_characters = s.chars().any(|g| forbidden_characters.contains(&g));
    // Return `false` if any of our conditions have been violated
    !(is_empty_or_whitespace || is_too_long || contains_forbidden_characters)
}