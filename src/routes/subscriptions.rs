use crate::{
    domain::{NewSubscriber, SubscriberEmail, SubscriberName},
    email_client::EmailClient,
};
use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct Subscription {
    name: String,
    email: String,
}

impl TryFrom<Subscription> for NewSubscriber {
    type Error = String;

    fn try_from(form: Subscription) -> Result<Self, Self::Error> {
        let name = SubscriberName::parse(form.name)?;
        let email = SubscriberEmail::parse(form.email)?;
        Ok(NewSubscriber { email, name })
    }
}

#[tracing::instrument(
    name = "Adding a new subscriber",
    skip(form, pool, email_client),
    fields(
        subscriber_name = %form.name,
        subscriber_email = %form.email
    )
)]
pub async fn subscribe(
    form: web::Form<Subscription>,
    pool: web::Data<PgPool>,
    email_client: web::Data<EmailClient>,
) -> HttpResponse {
    let new_subscriber = match form.0.try_into() {
        Ok(subscriber) => subscriber,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };
    if insert_subscriber(&pool, &new_subscriber).await.is_err() {
        return HttpResponse::InternalServerError().finish();
    };
    if send_confirmation_mail(&email_client, new_subscriber)
        .await
        .is_err()
    {
        return HttpResponse::InternalServerError().finish();
    }
    HttpResponse::Ok().finish()
}

#[tracing::instrument(
    "Send a confirmation mail to a new subscriber",
    skip(email_client, new_subscriber)
)]
pub async fn send_confirmation_mail(
    email_client: &EmailClient,
    new_subscriber: NewSubscriber,
) -> Result<(), reqwest::Error> {
    let confirmation_link = "https://myapi.com/subscriptions/confirm";
    email_client
        .send_email(
            new_subscriber.email,
            "Welcome!",
            &format!("Welcome to our newsletter!<br/>Click <a href=\"{}\">here</a> to confirm your subscription.",confirmation_link),
            &format!("Welcome to our newsletter!\n Visit {} to confirm your subscription.", confirmation_link),
        )
        .await
}

#[tracing::instrument(
    name = "Saving new subscriber details in the database",
    skip(subscription, pool)
)]
pub async fn insert_subscriber(
    pool: &PgPool,
    subscription: &NewSubscriber,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at, status)
        VALUES( $1 , $2, $3, $4, 'pending_confirmation');
        "#,
        Uuid::new_v4(),
        subscription.email.as_ref(),
        subscription.name.as_ref(),
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
