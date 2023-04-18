use once_cell::sync::Lazy;
use secrecy::ExposeSecret;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::net::{SocketAddr, TcpListener};
use uuid::Uuid;
use zero2prod::{
    configuration::{get_configuration, DatabaseSettings},
    telemetry::{get_subscriber, init_subscriber},
};

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

static TRACING: Lazy<()> = Lazy::new(|| {
    let filter_level = "info".to_string();
    let subscriber_name = "test".to_string();

    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber(subscriber_name, filter_level, std::io::stdout);
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber(subscriber_name, filter_level, std::io::sink);
        init_subscriber(subscriber);
    }
});
fn run_migration(url: &str) {
    let output = std::process::Command::new("dbmate")
        .arg("-u")
        .arg(format!("{}?sslmode=disable", url))
        .arg("migrate")
        .output()
        .expect("Failed to migrate database");
    assert!(output.status.success());
}
pub async fn configure_database(config: &DatabaseSettings) -> PgPool {
    let mut connection = PgConnection::connect(&config.connection_string_without_db().expose_secret())
        .await
        .expect("unable to connect to database any");

    connection
        .execute(format!(r#"Create Database "{}";"#, config.database_name).as_str())
        .await
        .expect("Failed to create database");

    let connection_pool = PgPool::connect(&config.connection_string().expose_secret())
        .await
        .expect("failed to connect to postgres");

    run_migration(&config.connection_string().expose_secret());

    connection_pool
}
async fn spawn_app() -> TestApp {
    Lazy::force(&TRACING);
    let addr = SocketAddr::from(([127, 0, 0, 1], 0));
    let listener = TcpListener::bind(addr).expect("uanble to bind to port");
    let addr = listener.local_addr().unwrap();

    let mut configuration = get_configuration().expect("Failed to read configuration.");
    configuration.database.database_name = Uuid::new_v4().to_string();
    let con_pool = configure_database(&configuration.database).await;

    let storage = zero2prod::storage::Storage::new(&configuration.database.connection_string().expose_secret())
        .await
        .expect("Failed to connect to database.");

    let server = zero2prod::routes::new_server(listener, storage);
    tokio::spawn(server);

    return TestApp {
        address: addr.to_string(),
        db_pool: con_pool,
    };
}

#[tokio::test(flavor = "multi_thread")]
async fn health_check_works() {
    let test_app = spawn_app().await;
    let client = ureq::AgentBuilder::new().build();

    let response = client
        .get(&format!("http://{:}/health_check", test_app.address))
        .call()
        .expect("unable to send request");

    assert_eq!(response.status(), 200);
}

#[tokio::test(flavor = "multi_thread")]
async fn subscribe_returns_a_200_for_valid_form_data() {
    let test_app = spawn_app().await;

    let client = ureq::AgentBuilder::new().build();
    let form = &[("name", "david"), ("email", "davy@gmail.com")];
    let response = client
        .post(&format!("http://{:}/subscriptions", test_app.address))
        .set("Content-Type", "application/x-www-form-urlencoded")
        .send_form(form)
        .expect("Failed to execute request");
    #[derive(sqlx::FromRow)]
    struct User {
        name: String,
        email: String,
    }
    let saved: User = sqlx::query_as("SELECT email, name FROM subscriptions")
        .fetch_one(&test_app.db_pool)
        .await
        .expect("unable to fetch saved subscription");

    assert_eq!(response.status(), 200);
    assert_eq!("david", saved.name);
    assert_eq!("davy@gmail.com", saved.email);
}

#[tokio::test(flavor = "multi_thread")]
async fn subscribe_returns_a_400_when_data_is_missing() {
    let test_app = spawn_app().await;
    let client = ureq::AgentBuilder::new().build();

    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(&format!("http://{:}/subscriptions", test_app.address))
            .set("Content-Type", "application/x-www-form-urlencoded")
            .send_string(invalid_body)
            .err()
            .unwrap()
            .into_response()
            .unwrap();

        assert_eq!(
            response.status(),
            422,
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        );
    }
}
