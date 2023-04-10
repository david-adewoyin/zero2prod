use std::net::{SocketAddr, TcpListener};

fn spawn_app() -> String {
    let addr = SocketAddr::from(([127, 0, 0, 1], 0));
    let listener = TcpListener::bind(addr).expect("uanble to bind to port");
    let addr = listener.local_addr().unwrap();
    let server = zero2prod::new_server(listener);
    tokio::spawn(server);

    return addr.to_string();
}

#[tokio::test(flavor = "multi_thread")]
async fn health_check_works() {
    let addr = spawn_app();
    let client = ureq::AgentBuilder::new().build();

    let response = client
        .get(&format!("http://{:}/health_check", addr))
        .call()
        .expect("unable to send request");

    assert_eq!(response.status(), 200);
}

#[tokio::test(flavor = "multi_thread")]
async fn subscribe_returns_a_200_for_valid_form_data() {
    let addr = spawn_app();
    let client = ureq::AgentBuilder::new().build();

    let response = client
        .post(&format!("http://{:}/subscriptions", addr))
        .set("Content-Type", "application/x-www-form-urlencoded")
        .send_form(&[("name", "a name"), ("email", "an email")])
        .expect("Failed to execute request");

    assert_eq!(response.status(), 200);
}

#[tokio::test(flavor = "multi_thread")]
async fn subscribe_returns_a_400_when_data_is_missing() {
    let addr = spawn_app();
    let client = ureq::AgentBuilder::new().build();

    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(&format!("http://{:}/subscriptions", addr))
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
