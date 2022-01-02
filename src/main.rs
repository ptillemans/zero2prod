use sqlx::PgPool;
use std::net::TcpListener;
use tokio::time::timeout;
use zero2prod::configuration;
use zero2prod::email_client::EmailClient;
use zero2prod::startup::run;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = configuration::get_configuration().expect("Failed to read configuration.");
    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port,
    );
    tracing::info!("Starting server on  address {}", address);
    let listener =
        TcpListener::bind(address).expect("Could not bind to port. Is the port already in use?");
    let db_pool = PgPool::connect_lazy_with(configuration.database.with_db());
    let sender = configuration
        .email_client
        .sender()
        .expect("Invalid sender email address");
    let timeout = configuration.email_client.timeout();
    let email_client = EmailClient::new(
        configuration.email_client.base_url,
        sender,
        configuration.email_client.authorization_token,
        timeout,
    );
    run(listener, db_pool, email_client)?.await
}
