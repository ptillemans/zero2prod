use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct Subscription {
    name: String,
    email: String,
}

#[tracing::instrument(
    name = "Adding a new subscriber",
    skip(form, pool),
    fields(
        subscriber_name = %form.name,
        subscriber_email = %form.email
    )
)]
pub async fn subscribe(form: web::Form<Subscription>, pool: web::Data<PgPool>) -> HttpResponse {
    match insert_subscriber(&pool, &form).await {
        Ok(_) => {
            tracing::info!("Saving new subscriber details in the database.");
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            tracing::error!("Failed to execute query : {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[tracing::instrument(
    name = "Saving new subscriber details in the database",
    skip(subscription, pool)
)]
pub async fn insert_subscriber(
    pool: &PgPool,
    subscription: &Subscription,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
                INSERT INTO subscriptions (id, email, name, subscribed_at)
                VALUES( $1 , $2, $3, $4);
                "#,
        Uuid::new_v4(),
        subscription.email,
        subscription.name,
        Utc::now()
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failing to insert subscriber: {:?}", e);
        e
    })?;
    Ok(())
}
