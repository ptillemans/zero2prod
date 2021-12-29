use sqlx::{Connection, PgPool};
use std::net::TcpListener;
use zero2prod::configuration;
use zero2prod::startup::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let configuration = configuration::get_configuration().expect("Failed to read configuration.");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)
        .expect("Could not bind to port 8000. Is the port already in use?");
    let connection = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to postgres database");
    run(listener, connection)?.await
}
