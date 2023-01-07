use rust_web_server::configuration::get_configuration;
use rust_web_server::startup::run;
use rust_web_server::telemetry::{get_subscriber, init_subscriber};
use secrecy::ExposeSecret;
use sqlx::PgPool;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber(
        "rust_web_server".into(), 
        "info".into(), 
        std::io::stdout
    );
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Faild to read configuration.");
    let connection_pool = PgPool::connect(&configuration.database.connection_string().expose_secret())
        .await
        .expect("Failed to connect to Postgres.");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await
}
