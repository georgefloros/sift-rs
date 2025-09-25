use axum::{
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sift_rs::sift;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tracing::{error, info};

#[derive(Debug, Deserialize)]
struct ValidationItem {
    /// The input object to validate
    input: Value,
    /// The MongoDB-style query to validate against
    query: Value,
}

#[derive(Debug, Serialize)]
struct ValidationResult {
    /// Whether the input matches the query
    valid: bool,
}

#[derive(Debug, Serialize)]
struct ErrorResponse {
    error: String,
    message: String,
}

#[derive(Debug, Serialize)]
struct HealthResponse {
    status: String,
    version: String,
    message: String,
}

/// Health check endpoint
async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "healthy".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        message: "Sift-rs API is running".to_string(),
    })
}

/// Validate endpoint - validates an array of input/query pairs
///
/// POST /validate
/// Body: [{ "input": {...}, "query": {...} }, ...]
/// Response: [{ "valid": true/false }, ...]
async fn validate(
    Json(payload): Json<Vec<ValidationItem>>,
) -> Result<Json<Vec<ValidationResult>>, (StatusCode, Json<ErrorResponse>)> {
    info!("Processing validation request with {} items", payload.len());

    let mut results = Vec::with_capacity(payload.len());

    for (index, item) in payload.into_iter().enumerate() {
        match sift(&item.query, &item.input) {
            Ok(valid) => {
                results.push(ValidationResult { valid });
            }
            Err(e) => {
                error!("Validation failed for item {}: {}", index, e);
                return Err((
                    StatusCode::BAD_REQUEST,
                    Json(ErrorResponse {
                        error: "ValidationFailed".to_string(),
                        message: format!("Failed to validate item {}: {}", index, e),
                    }),
                ));
            }
        }
    }

    info!("Validation completed: processed {} items", results.len());
    Ok(Json(results))
}

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();
    info!("Starting Sift-rs API server");
    // Build the application with routes
    let app = Router::new()
        .route("/health", get(health))
        .route("/validate", post(validate))
        .layer(ServiceBuilder::new().layer(CorsLayer::permissive()));
    // Run the server
    // port from environment variable or default to 3000
    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    info!("Binding to port {}", port);
    // use TcpListener to bind to the specified port
    let bind_address = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(&bind_address)
        .await
        .expect("Failed to bind to port");

    info!("Server running on http://0.0.0.0:{}", port);
    info!("Available endpoints:");
    info!("  GET  /health    - Health check");
    info!("  POST /validate  - Validate array of input/query pairs");
    info!("");
    info!("Example request to /validate:");
    info!("POST /validate");
    info!("Content-Type: application/json");
    info!("");
    info!("[");
    info!("  {");
    info!("    \"input\": {\"name\": \"Alice\", \"age\": 30},");
    info!("    \"query\": {\"age\": {\"$gte\": 25}}");
    info!("  },");
    info!("  {");
    info!("    \"input\": {\"name\": \"Bob\", \"age\": 20},");
    info!("    \"query\": {\"age\": {\"$gte\": 25}}");
    info!("  }");
    info!("]");
    info!("");
    info!("Response:");
    info!("[");
    info!("  {\"valid\": true},");
    info!("  {\"valid\": false}");
    info!("]");

    axum::serve(listener, app)
        .await
        .expect("Server failed to start");
}