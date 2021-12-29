use actix_web::{web, HttpResponse};

#[derive(serde::Deserialize)]
pub struct Subscription {
    name: String,
    email: String,
}

pub async fn subscribe(_form: web::Form<Subscription>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
