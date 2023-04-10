use std::{net::SocketAddr};

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3500));
    run(addr).await;
}

pub async fn run(addr: SocketAddr) {
    println!("Starting server");
    let server = zero2prod::new_server(addr);
    server.await.expect("unable to start server");
}
