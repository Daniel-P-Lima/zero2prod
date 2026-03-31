use actix_web::{HttpResponse, web::Data, web::Form};
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use tracing::Instrument;
use uuid::Uuid;

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

pub async fn subscribe(form: Form<FormData>, pool: Data<PgPool>) -> HttpResponse {
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
            HttpResponse::Ok().json(values)
        }
        Err(e) => {
            println!("Error {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
