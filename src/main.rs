use std::net::TcpListener;
use sqlx::{Connection, PgConnection};
use rust_web_server::startup::run;
use rust_web_server::configuration::get_configuration;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Faild to read configuration.");
    let connection = PgConnection::connect(
            &configuration.database.connection_string()
        )
        .await
        .expect("Failed to connect to Postgres.");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    run(listener, connection)?.await
}
