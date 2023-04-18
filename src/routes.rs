use axum::extract::State;
use axum::extract::{rejection::FormRejection, Form};
use axum::{response::IntoResponse, routing::get, routing::post, Router};
use hyper::{Body, Request, Response, StatusCode};
use tracing::Span;
use std::net::{SocketAddr, TcpListener};
use std::sync::Arc;
use std::time::Duration;
use tower_http::trace::TraceLayer;
use uuid::Uuid;

#[derive(Clone, Debug)]
struct AppState {
    storage: Arc<Storage>,
}

use crate::storage::{self, Storage};
pub async fn new_server(
    listener: TcpListener,
    storage: storage::Storage,
) -> Result<(), hyper::Error> {
    let app_state = AppState {
        storage: Arc::new(storage),
    };

    let app = Router::new()
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe))
        //.layer(            TraceLayer::new_for_http()   )
        .with_state(app_state);

    let router = app;
    axum::Server::from_tcp(listener)
        .expect("Failed to bind to address")
        .serve(router.into_make_service())
        .await
}

#[tracing::instrument]
async fn health_check() -> impl IntoResponse {
    tracing::info!("Health check request received");
    "Hello world"
}

#[derive(Debug, serde::Deserialize)]
struct FormData {
    name: String,
    email: String,
}
#[tracing::instrument(skip(state,form)
,fields(http.method = "POST", http.path = "/subscriptions dead", request_id = %Uuid::new_v4(),subscriber_email = %form.email,
subscriber_name = %form.name))]
async fn subscribe(State(state): State<AppState>, Form(form): Form<FormData>) -> impl IntoResponse {
    match state
        .storage
        .insert_subscriber(&form.name, &form.email)
        .await
    {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
