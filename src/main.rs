use std::{net::{SocketAddr, TcpListener}};

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3500));
    run(addr).await;
}

pub async fn run(addr: SocketAddr) {
    let listener = TcpListener::bind(addr).expect("uanble to bind to port");
    let server = zero2prod::new_server(listener);
    server.await.expect("unable to start server");
}

