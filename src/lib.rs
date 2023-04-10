use axum::{ routing::get, Router, response::IntoResponse};
use std::net::SocketAddr;

pub async fn new_server(addr: SocketAddr) -> Result<(), hyper::Error> {
    let app = Router::new().route("/health_check", get(health_check));
    let router = app;
    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
}

async fn health_check() -> impl IntoResponse {
    "Hello world"
}
