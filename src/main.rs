use rust_web_server::configuration::get_configuration;
use rust_web_server::startup::build;
use rust_web_server::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("rust-web-server".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration.");
    let server = build(configuration).await?;
    server.await?;
    Ok(())
}
