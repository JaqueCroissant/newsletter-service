use newsletter_service::startup::run;
use newsletter_service::configuration::get_configuration;
use sqlx::PgPool;
use std::net::TcpListener;


#[tokio::main]
async fn main() -> std::io::Result<()> {

    let config = get_configuration().expect("Failed to read configuration.yaml");
    let connection_pool = PgPool::connect(
        &config.database.connection_string()
    )
    .await
    .expect("Failed to connect to Postgres DB");

    let address = format!("127.0.0.1:{}", config.application_port);
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await
}