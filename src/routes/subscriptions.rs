use actix_web::{HttpResponse, web::Form};


#[derive(serde::Deserialize)]
pub struct FormData {
    _email: String,
    _name: String,
}


pub async fn subscribe(_form: Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
