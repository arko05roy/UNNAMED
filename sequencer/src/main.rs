use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use tower_http::cors::CorsLayer;
use zkcredit_program::{CreditOp, CreditState};

mod batch;
mod prover;
mod submitter;

use batch::BatchManager;

#[derive(Clone)]
struct AppState {
    batch_manager: Arc<Mutex<BatchManager>>,
}

#[derive(Deserialize)]
struct SubmitOpRequest {
    operation: CreditOp,
}

#[derive(Serialize)]
struct SubmitOpResponse {
    success: bool,
    batch_position: usize,
    batch_size: usize,
}

#[derive(Serialize)]
pub struct BatchStatusResponse {
    pub pending_ops: usize,
    pub batch_capacity: usize,
    pub last_batch_number: u64,
    pub state_root: String,
}

#[derive(Serialize)]
struct ForceResponse {
    success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    tx_hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    new_state_root: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

async fn submit_op(
    State(state): State<AppState>,
    Json(req): Json<SubmitOpRequest>,
) -> Json<SubmitOpResponse> {
    let mut batch_manager = state.batch_manager.lock().await;
    let (position, size) = batch_manager.add_operation(req.operation);

    Json(SubmitOpResponse {
        success: true,
        batch_position: position,
        batch_size: size,
    })
}

async fn batch_status(State(state): State<AppState>) -> Json<BatchStatusResponse> {
    let batch_manager = state.batch_manager.lock().await;
    Json(batch_manager.get_status())
}

async fn force_batch(State(state): State<AppState>) -> Json<ForceResponse> {
    let mut batch_manager = state.batch_manager.lock().await;

    match batch_manager.force_prove_and_submit().await {
        Ok((tx_hash, new_root)) => Json(ForceResponse {
            success: true,
            tx_hash: Some(tx_hash),
            new_state_root: Some(hex::encode(new_root)),
            error: None,
        }),
        Err(e) => Json(ForceResponse {
            success: false,
            tx_hash: None,
            new_state_root: None,
            error: Some(e.to_string()),
        }),
    }
}

async fn health() -> &'static str {
    "OK"
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let batch_manager = Arc::new(Mutex::new(BatchManager::new()));
    let state = AppState { batch_manager };

    let app = Router::new()
        .route("/submit-op", post(submit_op))
        .route("/batch-status", get(batch_status))
        .route("/force-batch", post(force_batch))
        .route("/health", get(health))
        .layer(CorsLayer::permissive())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001")
        .await
        .expect("Failed to bind to port 3001");

    tracing::info!("zkCredit Sequencer running on http://localhost:3001");

    axum::serve(listener, app).await.unwrap();
}
