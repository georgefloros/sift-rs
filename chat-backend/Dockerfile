# Build stage
FROM rust:1.88.0-slim-trixie AS builder

WORKDIR /app

# Install necessary build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy Cargo files
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src ./src

# Build the application
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install necessary runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    curl \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the binary from builder stage
COPY --from=builder /app/target/release/chat-backend .

# Create a non-root user
RUN useradd -r -s /bin/false appuser && chown appuser:appuser /app/chat-backend
USER appuser

# Set default port (can be overridden)
ENV PORT=3001

# Expose port
EXPOSE $PORT

# Run the application
CMD ["./chat-backend"]
