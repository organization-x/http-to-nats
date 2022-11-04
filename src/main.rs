use std::{net::SocketAddr, sync::Arc};

use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::post, Json, Router};
use serde::Deserialize;
struct AppState {
    nats: nats::asynk::Connection,
}

#[tokio::main]
async fn main() {
    let nats = nats::asynk::connect(std::env::var("NATS_URL").unwrap())
        .await
        .unwrap();
    let state = Arc::new(AppState { nats });

    let app = Router::with_state(state).route("/", post(root));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Deserialize)]
struct NatsPublish {
    subject: String,
    payload: String,
}

async fn root(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<NatsPublish>,
) -> impl IntoResponse {
    state
        .nats
        .publish(&payload.subject, &payload.payload)
        .await
        .unwrap();
    let response = StatusCode::OK;
    response
}
