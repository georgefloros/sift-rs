#!/bin/bash

# Build script for sift-rs project
# Builds for server, library, and WASM targets

set -e  # Exit immediately if a command exits with a non-zero status

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}Sift-rs Build Script${NC}"
echo "====================="

# Function to print colored output
print_status() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Function to check if a command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Parse command line arguments
BUILD_TYPE="all"
CLEAN_FIRST=false
VERBOSE=false

while [[ $# -gt 0 ]]; do
    case $1 in
        -t|--type)
            BUILD_TYPE="$2"
            shift 2
            ;;
        -c|--clean)
            CLEAN_FIRST=true
            shift
            ;;
        -v|--verbose)
            VERBOSE=true
            shift
            ;;
        -h|--help)
            echo "Usage: $0 [OPTIONS]"
            echo "Build script for sift-rs project"
            echo ""
            echo "Options:"
            echo "  -t, --type TYPE     Build type: all, server, lib, wasm, chat, web (default: all)"
            echo "  -c, --clean         Clean before building"
            echo "  -v, --verbose       Verbose output"
            echo "  -h, --help          Show this help"
            exit 0
            ;;
        *)
            print_error "Unknown option: $1"
            exit 1
            ;;
    esac
done

# Clean target if requested
if [ "$CLEAN_FIRST" = true ]; then
    print_status "Cleaning target directory..."
    cargo clean
    rm -rf sift-rs-wasm/pkg
    rm -rf example-web-app/pkg
fi

# Check for required tools
print_status "Checking required tools..."

if ! command_exists "cargo"; then
    print_error "Cargo is not installed. Please install Rust and Cargo first."
    exit 1
fi

if [ "$BUILD_TYPE" = "wasm" ] || [ "$BUILD_TYPE" = "all" ]; then
    if ! command_exists "wasm-pack"; then
        print_error "wasm-pack is required for WASM builds. Install with: cargo install wasm-pack"
        exit 1
    fi
    
    # Check for WASM target
    if ! rustup target list --installed | grep -q "wasm32-unknown-unknown"; then
        print_warning "WASM target not installed. Installing..."
        rustup target add wasm32-unknown-unknown
    fi
fi

print_status "All required tools are available."

# Build function
build_lib() {
    print_status "Building sift-rs library..."
    cargo build
    print_status "Library build completed successfully."
}

build_server() {
    print_status "Building sift-rs server binary..."
    cargo build --bin sift-rs-server --features server
    print_status "Server binary build completed successfully."
}

build_wasm_web() {
    print_status "Building WASM package for web..."
    cd sift-rs-wasm
    wasm-pack build --target web --out-dir pkg
    cd ..
    
    # Copy the WASM package to the example web app
    print_status "Copying WASM package to example web app..."
    cp -r sift-rs-wasm/pkg example-web-app/pkg/
    print_status "WASM web build completed successfully."
}

build_wasm_nodejs() {
    print_status "Building WASM package for Node.js..."
    cd sift-rs-wasm
    wasm-pack build --target nodejs --out-dir pkg-node
    cd ..
    print_status "WASM Node.js build completed successfully."
}

build_wasm_bundler() {
    print_status "Building WASM package for bundlers (Webpack, etc.)..."
    cd sift-rs-wasm
    wasm-pack build --target bundler --out-dir pkg-bundler
    cd ..
    print_status "WASM bundler build completed successfully."
}

build_chat_backend() {
    print_status "Building chat backend..."
    cd chat-backend
    cargo build
    cd ..
    print_status "Chat backend build completed successfully."
}

build_web() {
    print_status "Building web frontend..."
    cd web
    
    # Check if node and npm are available
    if ! command_exists "npm"; then
        print_error "npm is required for web frontend build."
        exit 1
    fi
    
    # Install dependencies if node_modules doesn't exist
    if [ ! -d "node_modules" ]; then
        print_status "Installing web dependencies..."
        npm install
    fi
    
    # Build the web application
    npm run build
    cd ..
    print_status "Web frontend build completed successfully."
}

# Build based on type
case $BUILD_TYPE in
    "lib")
        build_lib
        ;;
    "server")
        build_server
        ;;
    "wasm")
        build_wasm_web
        build_wasm_nodejs
        build_wasm_bundler
        ;;
    "chat")
        build_chat_backend
        ;;
    "web")
        build_web
        ;;
    "all")
        print_status "Building all targets..."
        
        build_lib
        build_server
        build_wasm_web
        build_wasm_nodejs
        build_wasm_bundler
        build_chat_backend
        
        # Only build web if requested separately due to dependencies
        if command_exists "npm"; then
            build_web
        else
            print_warning "npm not found, skipping web frontend build"
        fi
        
        print_status "All builds completed successfully!"
        ;;
    *)
        print_error "Invalid build type: $BUILD_TYPE"
        print_error "Valid types: all, server, lib, wasm, chat, web"
        exit 1
        ;;
esac

# Show build artifacts
echo ""
print_status "Build artifacts:"
echo "  - Server binary: ./target/debug/sift-rs-server"
echo "  - WASM (web): sift-rs-wasm/pkg/"
echo "  - WASM (Node.js): sift-rs-wasm/pkg-node/"
echo "  - WASM (bundler): sift-rs-wasm/pkg-bundler/"
echo "  - Example web app: example-web-app/pkg/"
echo "  - Chat backend: chat-backend/target/debug/chat-backend"
if [ -d "web/.next" ]; then
    echo "  - Web frontend: web/.next/"
fi

echo ""
print_status "Build completed successfully! 🎉"