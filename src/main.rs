use rust_web_server::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    run().await
}
