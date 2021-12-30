use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use tracing_futures::Instrument;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct Subscription {
    name: String,
    email: String,
}

pub async fn subscribe(form: web::Form<Subscription>, pool: web::Data<PgPool>) -> HttpResponse {
    let request_id = Uuid::new_v4();
    let request_span = tracing::info_span!(
        "Adding a new subcriber",
        %request_id,
        subscriber_name = %form.name,
        subscriber_email = %form.email
    );

    // !!! do not do this in prod in async functions !!!
    // when the _request_span_guard goes out of scope it is dropped
    // closing the span.
    let _request_span_guard = request_span.enter();

    let query_span = tracing::info_span!("Saving new subscriber details in the database.");
    match sqlx::query!(
        r#"
            INSERT INTO subscriptions (id, email, name, subscribed_at)
            VALUES( $1 , $2, $3, $4);
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
        Ok(_) => {
            tracing::info!("Saving new subscriber details in the database.");
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            tracing::error!(
                "requestId {} - Failed to execute query : {:?}",
                request_id,
                e
            );
            HttpResponse::InternalServerError().finish()
        }
    }
}
