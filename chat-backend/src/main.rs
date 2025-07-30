use anyhow::Result;
use axum::{
    extract::{ws::WebSocketUpgrade, State},
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::{get, post},
    Router,
};
use dotenv::dotenv;
use std::sync::Arc;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::{
    cors::CorsLayer,
    trace::TraceLayer,
};
use tracing::{info, warn};

mod ai_agent;
mod chat_handler;
mod models;
mod sift_integration;

use ai_agent::ChatAgent;
use chat_handler::websocket_handler;
use models::*;
use sift_integration::SiftClient;

/// Application state containing shared services
#[derive(Clone)]
pub struct AppState {
    pub chat_agent: Arc<ChatAgent>,
    pub sift_client: Arc<SiftClient>,
}

/// Health check endpoint
async fn health() -> impl IntoResponse {
    Json(serde_json::json!({
        "status": "healthy",
        "service": "chat-backend",
        "version": env!("CARGO_PKG_VERSION")
    }))
}

/// REST endpoint for one-shot queries (alternative to WebSocket)
async fn chat(
    State(state): State<AppState>,
    Json(request): Json<ChatRequest>,
) -> Result<Json<ChatResponse>, StatusCode> {
    match state.chat_agent.process_message(
        &request.message,
        request.schema.as_ref(),
        request.sample_data.as_ref(),
    ).await {
        Ok(response) => Ok(Json(ChatResponse {
            message: response.message,
            query: response.query,
            explanation: response.explanation,
        })),
        Err(e) => {
            warn!("Error processing chat request: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// WebSocket upgrade endpoint
async fn websocket(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| websocket_handler(socket, state))
}

#[tokio::main]
async fn main() -> Result<()> {
    // Load environment variables
    dotenv().ok();

    // Initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .compact()
        .init();

    info!("Starting Chat Backend Server...");

    // Initialize services
    let chat_agent = Arc::new(ChatAgent::new().await?);
    let sift_client = Arc::new(SiftClient::new());

    let app_state = AppState {
        chat_agent,
        sift_client,
    };

    // Build our application with routes
    let app = Router::new()
        .route("/health", get(health))
        .route("/chat", post(chat))
        .route("/ws", get(websocket))
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(CorsLayer::permissive()),
        )
        .with_state(app_state);

    // Get port from environment or default to 3001
    let port = std::env::var("PORT").unwrap_or_else(|_| "3001".to_string());
    let bind_address = format!("0.0.0.0:{}", port);
    
    info!("Server binding to {}", bind_address);
    let listener = TcpListener::bind(&bind_address).await?;
    
    info!("🚀 Chat Backend Server running on http://0.0.0.0:{}", port);
    info!("📡 WebSocket endpoint: ws://0.0.0.0:{}/ws", port);
    info!("🔗 REST endpoint: http://0.0.0.0:{}/chat", port);
    info!("❤️  Health check: http://0.0.0.0:{}/health", port);

    axum::serve(listener, app).await?;
    
    Ok(())
}
