use std::net::{SocketAddr, TcpListener};

use secrecy::ExposeSecret;
use zero2prod::{
    configuration::get_configuration,
    storage,
    telemetry::{get_subscriber, init_subscriber},
};
#[tokio::main]
async fn main() {
    let subscriber = get_subscriber("zero2prod".into(), "debug".into(), std::io::stdout);
    init_subscriber(subscriber);

    let config = get_configuration().expect("unable to read configuration");
    let addr = SocketAddr::from(([127, 0, 0, 1], config.application_port));
    let storage = storage::Storage::new(&config.database.connection_string().expose_secret())
        .await
        .expect("unable to connect to database");
    run(addr, storage).await;
}

pub async fn run(addr: SocketAddr, storage: storage::Storage) {
    tracing::debug!("listening on {}", addr.to_string());
    let listener = TcpListener::bind(addr).expect("uanble to bind to port");
    let server = zero2prod::routes::new_server(listener, storage);
    server.await.expect("unable to start server");
}
