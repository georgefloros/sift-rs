# Chat Backend - AI-Powered Sift Query Builder

A Rust-based chat backend that uses [Rig.rs](https://rig.rs/) to power natural language to MongoDB query conversion for the Sift-rs query builder.

## Features

- 🤖 **AI-Powered Query Generation**: Uses OpenAI GPT-4 through Rig.rs to convert natural language to MongoDB queries
- 🔄 **Real-time Communication**: WebSocket support for live chat interaction
- 🔗 **Sift Integration**: Direct integration with the sift-rs validation API
- 📊 **REST API**: Alternative HTTP endpoints for simple request/response interaction
- 🚀 **High Performance**: Built with Axum and Tokio for async performance
- 📝 **Structured Responses**: Returns both the generated query and explanations

## Architecture

```
Frontend (Next.js) <-> Chat Backend (Rust + Rig.rs) <-> Sift-rs API
                                   ^
                                   |
                              OpenAI GPT-4
```

## API Endpoints

### REST API

- `GET /health` - Health check
- `POST /chat` - Send a chat message and get AI response

### WebSocket

- `GET /ws` - WebSocket endpoint for real-time chat

## WebSocket Message Format

### Incoming Messages (Client -> Server)

```json
{
  "type": "user_message",
  "id": "uuid",
  "message": "Find users older than 25"
}
```

### Outgoing Messages (Server -> Client)

```json
{
  "type": "ai_response",
  "id": "uuid", 
  "message": "I'll help you create a query to find users older than 25.",
  "query": "{\"age\": {\"$gt\": 25}}",
  "explanation": "This query uses the $gt operator to find documents where the age field is greater than 25."
}
```

## Setup

1. **Environment Variables**

   Copy the example environment file:
   ```bash
   cp .env.example .env
   ```

   Configure your environment variables:
   ```env
   OPENAI_API_KEY=your_openai_api_key_here
   PORT=3001
   SIFT_API_URL=http://localhost:3000
   RUST_LOG=info
   ```

2. **Install Dependencies**

   ```bash
   cargo build
   ```

3. **Run the Server**

   ```bash
   cargo run
   ```

   The server will start on `http://localhost:3001` (or the port specified in your environment).

## Development

### Running in Development Mode

```bash
# With auto-reload
cargo install cargo-watch
cargo watch -x run
```

### Testing the WebSocket Connection

You can test the WebSocket connection using a tool like `wscat`:

```bash
npm install -g wscat
wscat -c ws://localhost:3001/ws

# Send a message
{"type": "user_message", "id": "test-1", "message": "Find users with age greater than 25"}
```

### Testing the REST API

```bash
curl -X POST http://localhost:3001/chat \
  -H "Content-Type: application/json" \
  -d '{"message": "Find users older than 25"}'
```

## Dependencies

- **rig-core**: AI framework for Rust
- **axum**: Web framework
- **tokio**: Async runtime
- **serde**: Serialization
- **tracing**: Logging
- **anyhow**: Error handling

## Integration with Frontend

The chat backend is designed to work with the Next.js frontend. The frontend should:

1. Establish a WebSocket connection to `/ws`
2. Send user messages in the specified format
3. Listen for AI responses with query and explanation data
4. Update the query builder UI automatically when queries are received

Example frontend integration:

```javascript
const ws = new WebSocket('ws://localhost:3001/ws');

ws.onmessage = (event) => {
  const message = JSON.parse(event.data);
  
  if (message.type === 'ai_response') {
    // Update chat UI
    appendMessage(message.message);
    
    // Update query builder if query is provided
    if (message.query) {
      updateQueryBuilder(JSON.parse(message.query));
    }
  }
};

// Send user message
ws.send(JSON.stringify({
  type: 'user_message',
  id: generateId(),
  message: userInput
}));
```

## Configuration

The AI agent can be configured by modifying the system message in `src/ai_agent.rs`. Key parameters:

- **Temperature**: Controls creativity (0.0-1.0, currently 0.2 for consistency)
- **Max Tokens**: Maximum response length (currently 1024)
- **Model**: OpenAI model to use (currently GPT-4)

## Error Handling

The backend provides comprehensive error handling:

- Invalid OpenAI API keys
- Network connectivity issues
- Sift-rs API unavailability
- Malformed requests
- WebSocket connection errors

All errors are logged with appropriate detail levels and user-friendly error messages are returned to the client.

## Performance Considerations

- Uses connection pooling for HTTP requests
- Implements WebSocket ping/pong for connection health
- Async/await throughout for non-blocking operations
- Structured logging for observability

## Security

- CORS enabled for cross-origin requests
- Input validation on all endpoints
- Environment-based configuration
- No sensitive data in logs

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## License

MIT License - see the LICENSE file for details.
