# PowerShell build script for sift-rs project
# Builds for server, library, and WASM targets

# Set error action preference
$ErrorActionPreference = "Stop"

# Colors for output
$green = $([System.ConsoleColor]::Green)
$yellow = $([System.ConsoleColor]::Yellow)
$red = $([System.ConsoleColor]::Red)
$blue = $([System.ConsoleColor]::Blue)
$white = $([System.ConsoleColor]::White)

Write-Host "Sift-rs Build Script" -ForegroundColor $blue
Write-Host "=====================" -ForegroundColor $blue

# Function to print colored output
function Write-Status {
    param([string]$message)
    Write-Host "[INFO] $message" -ForegroundColor $green
}

function Write-Warning {
    param([string]$message)
    Write-Host "[WARNING] $message" -ForegroundColor $yellow
}

function Write-ErrorCustom {
    param([string]$message)
    Write-Host "[ERROR] $message" -ForegroundColor $red
}

# Function to check if a command exists
function Test-Command {
    param([string]$cmd)
    $null = Get-Command $cmd -ErrorAction SilentlyContinue
    return $LASTEXITCODE -eq 0
}

# Parse arguments
param(
    [string]$Type = "all",
    [switch]$Clean,
    [switch]$Verbose
)

# Clean target if requested
if ($Clean) {
    Write-Status "Cleaning target directory..."
    cargo clean
    Remove-Item -Recurse -Force "sift-rs-wasm/pkg" -ErrorAction SilentlyContinue
    Remove-Item -Recurse -Force "example-web-app/pkg" -ErrorAction SilentlyContinue
}

# Check for required tools
Write-Status "Checking required tools..."

if (!(Test-Command "cargo")) {
    Write-ErrorCustom "Cargo is not installed. Please install Rust and Cargo first."
    exit 1
}

if ($Type -eq "wasm" -or $Type -eq "all") {
    if (!(Test-Command "wasm-pack")) {
        Write-ErrorCustom "wasm-pack is required for WASM builds. Install with: cargo install wasm-pack"
        exit 1
    }
    
    # Check for WASM target
    $targets = rustup target list --installed
    if ($targets -notmatch "wasm32-unknown-unknown") {
        Write-Warning "WASM target not installed. Installing..."
        rustup target add wasm32-unknown-unknown
    }
}

Write-Status "All required tools are available."

# Build functions
function Build-Lib {
    Write-Status "Building sift-rs library..."
    cargo build
    Write-Status "Library build completed successfully."
}

function Build-Server {
    Write-Status "Building sift-rs server binary..."
    cargo build --bin sift-rs-server --features server
    Write-Status "Server binary build completed successfully."
}

function Build-Wasm-Web {
    Write-Status "Building WASM package for web..."
    Set-Location sift-rs-wasm
    wasm-pack build --target web --out-dir pkg --features wasm
    Set-Location ..
    
    # Copy the WASM package to the example web app
    Write-Status "Copying WASM package to example web app..."
    Copy-Item -Recurse -Force "sift-rs-wasm/pkg" "example-web-app/pkg"
    Write-Status "WASM web build completed successfully."
}

function Build-Wasm-NodeJS {
    Write-Status "Building WASM package for Node.js..."
    Set-Location sift-rs-wasm
    wasm-pack build --target nodejs --out-dir pkg-node --features wasm
    Set-Location ..
    Write-Status "WASM Node.js build completed successfully."
}

function Build-Wasm-Bundler {
    Write-Status "Building WASM package for bundlers (Webpack, etc.)..."
    Set-Location sift-rs-wasm
    wasm-pack build --target bundler --out-dir pkg-bundler --features wasm
    Set-Location ..
    Write-Status "WASM bundler build completed successfully."
}

function Build-ChatBackend {
    Write-Status "Building chat backend..."
    Set-Location chat-backend
    cargo build
    Set-Location ..
    Write-Status "Chat backend build completed successfully."
}

function Build-Web {
    Write-Status "Building web frontend..."
    Set-Location web
    
    # Check if node and npm are available
    if (!(Test-Command "npm")) {
        Write-ErrorCustom "npm is required for web frontend build."
        exit 1
    }
    
    # Install dependencies if node_modules doesn't exist
    if (!(Test-Path "node_modules")) {
        Write-Status "Installing web dependencies..."
        npm install
    }
    
    # Build the web application
    npm run build
    Set-Location ..
    Write-Status "Web frontend build completed successfully."
}

# Build based on type
switch ($Type) {
    "lib" {
        Build-Lib
    }
    "server" {
        Build-Server
    }
    "wasm" {
        Build-Wasm-Web
        Build-Wasm-NodeJS
        Build-Wasm-Bundler
    }
    "chat" {
        Build-ChatBackend
    }
    "web" {
        Build-Web
    }
    "all" {
        Write-Status "Building all targets..."
        
        Build-Lib
        Build-Server
        Build-Wasm-Web
        Build-Wasm-NodeJS
        Build-Wasm-Bundler
        Build-ChatBackend
        
        # Only build web if requested separately due to dependencies
        if (Test-Command "npm") {
            Build-Web
        } else {
            Write-Warning "npm not found, skipping web frontend build"
        }
        
        Write-Status "All builds completed successfully!"
    }
    default {
        Write-ErrorCustom "Invalid build type: $Type"
        Write-ErrorCustom "Valid types: all, server, lib, wasm, chat, web"
        exit 1
    }
}

# Show build artifacts
Write-Host ""
Write-Status "Build artifacts:"
Write-Host "  - Server binary: .\target\debug\sift-rs-server.exe"
Write-Host "  - WASM (web): .\sift-rs-wasm\pkg\"
Write-Host "  - WASM (Node.js): .\sift-rs-wasm\pkg-node\"
Write-Host "  - WASM (bundler): .\sift-rs-wasm\pkg-bundler\"
Write-Host "  - Example web app: .\example-web-app\pkg\"
Write-Host "  - Chat backend: .\chat-backend\target\debug\chat-backend.exe"
if (Test-Path "web\.next") {
    Write-Host "  - Web frontend: .\web\.next\"
}

Write-Host ""
Write-Status "Build completed successfully! 🎉"