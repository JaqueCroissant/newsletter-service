use newsletter_service::run;
use newsletter_service::configuration::get_configuration;
use std::net::TcpListener;


#[tokio::main]
async fn main() -> std::io::Result<()> {

    let config = get_configuration().expect("Failed to read configuration.yaml");

    let address = format!("127.0.0.1:{}", config.application_port);
    let listener = TcpListener::bind(address)?;
    run(listener)?.await
}