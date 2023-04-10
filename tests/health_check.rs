use std::net::SocketAddr;

fn spawn_app() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3500));
    let server = zero2prod::new_server(addr);
    tokio::spawn(server);
}

#[tokio::test(flavor = "multi_thread")]
async fn health_check_works() {
    // Arrange
    spawn_app();

    let client = ureq::AgentBuilder::new().build();

    let response = client
        .get("http://127.0.0.1:3500/health_check")
        .call()
        .expect("unable to send request");

    assert_eq!(response.status(), 200, "error code 222");
}
