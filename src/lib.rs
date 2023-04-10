use axum::extract::{rejection::FormRejection, Form};
use axum::{response::IntoResponse, routing::get, routing::post,  Router};
use hyper::StatusCode;
use std::net::{SocketAddr, TcpListener};
pub async fn new_server(listener: TcpListener) -> Result<(), hyper::Error> {
    let app = Router::new()
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe));

    let router = app;
    axum::Server::from_tcp(listener)
        .expect("Failed to bind to address")
        .serve(router.into_make_service())
        .await
}

async fn health_check() -> impl IntoResponse {
    "Hello world"
}

#[derive(Debug, serde::Deserialize)]
struct FormData {
    name: String,
    email: String,
}

async fn subscribe(_: Form<FormData>) -> impl IntoResponse {
    StatusCode::OK
}
    