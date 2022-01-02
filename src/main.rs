use sqlx::PgPool;
use std::net::TcpListener;
use tokio::time::timeout;
use zero2prod::configuration;
use zero2prod::email_client::EmailClient;
use zero2prod::startup::{run, Application};
use zero2prod::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = configuration::get_configuration().expect("Failed to read configuration.");
    let application = Application::build(configuration).await?;
    application.run_until_stopped().await?;
    Ok(())
}
