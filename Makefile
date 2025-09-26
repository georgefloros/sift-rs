# Makefile for sift-rs project
# Builds for server, library, and WASM targets

.PHONY: all lib server wasm wasm-web wasm-nodejs wasm-bundler chat web clean help

# Default target
all: lib server wasm

# Build the library
lib:
	@echo \"Building sift-rs library...\"
	@cargo build
	@echo \"Library build completed successfully.\"

# Build the server binary
server:
	@echo \"Building sift-rs server binary...\"
	@cargo build --bin sift-rs-server --features server
	@echo \"Server binary build completed successfully.\"

# Build WASM for web
wasm-web:
	@echo \"Building WASM package for web...\"
	@cd sift-rs-wasm && wasm-pack build --target web --out-dir pkg --features wasm
	@echo \"Copying WASM package to example web app...\"
	@cp -r sift-rs-wasm/pkg example-web-app/pkg/
	@echo \"WASM web build completed successfully.\"

# Build WASM for Node.js
wasm-nodejs:
	@echo \"Building WASM package for Node.js...\"
	@cd sift-rs-wasm && wasm-pack build --target nodejs --out-dir pkg-node --features wasm
	@echo \"WASM Node.js build completed successfully.\"

# Build WASM for bundlers (Webpack, etc.)
wasm-bundler:
	@echo \"Building WASM package for bundlers (Webpack, etc.)...\"
	@cd sift-rs-wasm && wasm-pack build --target bundler --out-dir pkg-bundler --features wasm
	@echo \"WASM bundler build completed successfully.\"

# Build all WASM targets
wasm: wasm-web wasm-nodejs wasm-bundler

# Build chat backend
chat:
	@echo \"Building chat backend...\"
	@cd chat-backend && cargo build
	@echo \"Chat backend build completed successfully.\"

# Build web frontend
web:
	@echo \"Building web frontend...\"
	@cd web
	@if [ ! -d \"node_modules\" ]; then \\
		echo \"Installing web dependencies...\"; \\
		npm install; \\
	fi
	@npm run build
	@cd ..
	@echo \"Web frontend build completed successfully.\"

# Run tests
test:
	@echo \"Running tests...\"
	@cargo test
	@echo \"Tests completed successfully.\"

# Run benchmarks
bench:
	@echo \"Running benchmarks...\"
	@cargo bench
	@echo \"Benchmarks completed successfully.\"

# Clean build artifacts
clean:
	@echo \"Cleaning build artifacts...\"
	@cargo clean
	@rm -rf sift-rs-wasm/pkg
	@rm -rf sift-rs-wasm/pkg-node
	@rm -rf sift-rs-wasm/pkg-bundler
	@rm -rf example-web-app/pkg

# Show help
help:
	@echo \"Sift-rs Makefile\"
	@echo \"================\"
	@echo \"\"
	@echo \"Usage:\"
	@echo \"  make [target]\"
	@echo \"\"
	@echo \"Targets:\"
	@echo \"  all         - Build all targets (default)\"
	@echo \"  lib         - Build the sift-rs library\"
	@echo \"  server      - Build the server binary\"
	@echo \"  wasm        - Build all WASM targets\"
	@echo \"  wasm-web    - Build WASM for web\"
	@echo \"  wasm-nodejs - Build WASM for Node.js\"
	@echo \"  wasm-bundler - Build WASM for bundlers (Webpack, etc.)\"
	@echo \"  chat        - Build chat backend\"
	@echo \"  web         - Build web frontend\"
	@echo \"  test        - Run tests\"
	@echo \"  bench       - Run benchmarks\"
	@echo \"  clean       - Clean build artifacts\"
	@echo \"  help        - Show this help\"