# Build stage
FROM rust:1.85-slim as builder

# Install build tools and dependencies needed for V8 compilation
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    build-essential \
    cmake \
    git \
    python3 \
    python3-distutils \
    curl \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /app

# Copy source code
COPY . .

# Build the application
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install SSL runtime libraries
RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*

# Copy the binary
COPY --from=builder /app/target/release/sift-rs /usr/local/bin/sift-rs

# Set default port (can be overridden)
ENV PORT=3000

# Expose port
EXPOSE $PORT

# Run the application
CMD ["sift-rs"]
