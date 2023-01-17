use std::net::TcpListener;
use sqlx::{PgConnection, Connection};
use newsletter_service::{run, configuration::get_configuration};

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let app_address = spawn_app();
    let client = reqwest::Client::new();

    // Act
    let response = client
        // Use the returned application address
        .get(&format!("{}/health", &app_address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_200_on_valid_form_data() {
    // Arrange
    let app_address = spawn_app();
    let configuration = get_configuration().expect("Failed to get configuration");
    let connection_string = configuration.database.connection_string();
    let connection = PgConnection::connect(&connection_string).await.expect("Failed to connect to Postgres");
    let client = reqwest::Client::new();

    let body = "name=Frau%20Blucher&email=f.blucher%40gmail.com";

    // Act
    let response = client
        // Use the returned application address
        .post(&format!("{}/subscriptions", &app_address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn subscribe_returns_400_on_invalid_form_data() {
    // Arrange
    let app_address = spawn_app();
    let client = reqwest::Client::new();

    let test_cases = vec![
        ("name=Frau%20Blucher", "missing email"),
        ("email=f.blucher%40gmail.com", "missing name"),
        ("", "missing both email and name")
    ];

    for(body, message) in test_cases {
        // Act
        let response = client
        .post(&format!("{}/subscriptions", &app_address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

        // Assert
        assert_eq!(
            400, 
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload was {}.", 
            message
        );
    }
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0")
                                    .expect("Failed to bind random port");
    
    let port = listener.local_addr().unwrap().port();
    let server = run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    // We return the application address to the caller!
    format!("http://127.0.0.1:{}", port)
}