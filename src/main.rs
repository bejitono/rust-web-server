use rust_web_server::configuration::get_configuration;
use rust_web_server::email_client::EmailClient;
use rust_web_server::startup::run;
use rust_web_server::telemetry::{get_subscriber, init_subscriber};
use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("rust-web-server".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(configuration.database.with_db());

    let sender_email = configuration.email_client.sender()
        .expect("Invalide sender email address.");
    let email_client = EmailClient::new(
        configuration.email_client.base_url, 
        sender_email,
    );

    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool, email_client)?.await?;
    Ok(())
}
