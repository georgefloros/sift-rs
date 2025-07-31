use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Request model for chat messages
#[derive(Debug, Deserialize)]
pub struct ChatRequest {
    pub message: String,
    pub schema: Option<serde_json::Value>,
    pub sample_data: Option<serde_json::Value>,
}

/// Response model for chat messages
#[derive(Debug, Serialize)]
pub struct ChatResponse {
    pub message: String,
    pub query: Option<String>,
    pub explanation: Option<String>,
}

/// WebSocket message types
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum WebSocketMessage {
    #[serde(rename = "user_message")]
    UserMessage {
        id: String,
        message: String,
        schema: Option<serde_json::Value>,
        sample_data: Option<serde_json::Value>,
    },
    #[serde(rename = "ai_response")]
    AiResponse {
        id: String,
        message: String,
        query: Option<String>,
        explanation: Option<String>,
    },
    #[serde(rename = "query_update")]
    QueryUpdate {
        id: String,
        query: String,
        explanation: String,
    },
    #[serde(rename = "error")]
    Error {
        id: String,
        error: String,
    },
    #[serde(rename = "ping")]
    Ping,
    #[serde(rename = "pong")]
    Pong,
}

/// AI agent response structure
#[derive(Debug, Clone)]
pub struct AgentResponse {
    pub message: String,
    pub query: Option<String>,
    pub explanation: Option<String>,
}

impl WebSocketMessage {
    pub fn new_user_message(message: String, schema: Option<serde_json::Value>, sample_data: Option<serde_json::Value>) -> Self {
        Self::UserMessage {
            id: Uuid::new_v4().to_string(),
            message,
            schema,
            sample_data,
        }
    }

    pub fn new_ai_response(message: String, query: Option<String>, explanation: Option<String>) -> Self {
        Self::AiResponse {
            id: Uuid::new_v4().to_string(),
            message,
            query,
            explanation,
        }
    }

    pub fn new_query_update(query: String, explanation: String) -> Self {
        Self::QueryUpdate {
            id: Uuid::new_v4().to_string(),
            query,
            explanation,
        }
    }

    pub fn new_error(error: String) -> Self {
        Self::Error {
            id: Uuid::new_v4().to_string(),
            error,
        }
    }
}

/// Sift validation request structure
#[derive(Debug, Serialize)]
pub struct SiftValidationRequest {
    pub input: serde_json::Value,
    pub query: serde_json::Value,
}

/// Sift validation response structure
#[derive(Debug, Deserialize)]
pub struct SiftValidationResponse {
    pub valid: bool,
}
