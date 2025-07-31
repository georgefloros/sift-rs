use axum::extract::ws::{Message, WebSocket};
use futures_util::stream::StreamExt;
use tracing::{info, warn};

use crate::{
    models::WebSocketMessage,
    AppState,
};

pub async fn websocket_handler(mut socket: WebSocket, state: AppState) {
    info!("New WebSocket connection established");

    while let Some(Ok(msg)) = socket.next().await {
        match msg {
            Message::Text(text) => {
                match serde_json::from_str::<WebSocketMessage>(&text) {
                    Ok(WebSocketMessage::UserMessage { message, schema, sample_data, .. }) => {
                        info!("Received message from client: {}", message);

                        match state.chat_agent.process_message(
                            &message, 
                            schema.as_ref(), 
                            sample_data.as_ref()
                        ).await {
                            Ok(response) => {
                                let reply = WebSocketMessage::new_ai_response(
                                    response.message,
                                    response.query,
                                    response.explanation,
                                );
                                let reply_json = serde_json::to_string(&reply).unwrap();
                                if socket.send(Message::Text(reply_json.into())).await.is_err() {
                                    warn!("Failed to send AI response to client");
                                    break;
                                }
                            }
                            Err(e) => {
                                warn!("AI agent failed to process message: {}", e);
                                let error_reply = WebSocketMessage::new_error(
                                    "Failed to process your request. Please try again.".to_string(),
                                );
                                let error_json = serde_json::to_string(&error_reply).unwrap();
                                if socket.send(Message::Text(error_json.into())).await.is_err() {
                                    warn!("Failed to send error response to client");
                                    break;
                                }
                            }
                        }
                    }
                    Ok(WebSocketMessage::Ping) => {
                        let pong = WebSocketMessage::Pong;
                        let pong_json = serde_json::to_string(&pong).unwrap();
                        if socket.send(Message::Text(pong_json.into())).await.is_err() {
                            warn!("Failed to send pong to client");
                            break;
                        }
                    }
                    _ => {
                        warn!("Received unexpected message from client");
                    }
                }
            }
            Message::Close(_) => {
                info!("WebSocket connection closed");
                break;
            }
            _ => { /* Ignore other message types */ }
        }
    }
}
