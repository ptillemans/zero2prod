use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::configuration;
use zero2prod::startup::run;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

#[actix_web::main]
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
    let db_pool = PgPool::connect_lazy(&configuration.database.connection_string())
        .expect("Failed to connect to postgres database");
    run(listener, db_pool)?.await
}
