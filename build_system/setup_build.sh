#!/bin/bash

# setup_build.sh
# Build script for Life Browser
# Compiles the Rust backend and React frontend, integrating WebAssembly (WASM) for communication.
# Author: sujit9331
# License: Apache License 2.0

set -e  # Exit immediately if a command exits with a non-zero status
set -u  # Treat unset variables as an error
set -o pipefail  # Prevent errors in a pipeline from being masked

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[0;33m'
NC='\033[0m' # No Color

# Directories
RUST_BACKEND_DIR="src"
REACT_FRONTEND_DIR="ui"
BUILD_OUTPUT_DIR="dist"

# Functions
function print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

function print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

function print_info() {
    echo -e "${YELLOW}[INFO]${NC} $1"
}

# Step 1: Build the Rust backend
function build_rust_backend() {
    print_info "Building Rust backend..."
    cd "$RUST_BACKEND_DIR"

    # Ensure the WebAssembly target is installed
    if ! rustup target list | grep -q "wasm32-unknown-unknown (installed)"; then
        print_info "Installing WebAssembly target for Rust..."
        rustup target add wasm32-unknown-unknown
    fi

    # Build the Rust project for WebAssembly
    cargo build --release --target wasm32-unknown-unknown

    # Move the WASM output to the build directory
    mkdir -p "../$BUILD_OUTPUT_DIR/backend"
    cp target/wasm32-unknown-unknown/release/*.wasm "../$BUILD_OUTPUT_DIR/backend/"

    cd - > /dev/null
    print_success "Rust backend built successfully."
}

# Step 2: Build the React frontend
function build_react_frontend() {
    print_info "Building React frontend..."
    cd "$REACT_FRONTEND_DIR"

    # Install dependencies if node_modules is missing
    if [ ! -d "node_modules" ]; then
        print_info "Installing React dependencies..."
        npm install
    fi

    # Build the React application
    npm run build

    # Move the build output to the build directory
    mkdir -p "../$BUILD_OUTPUT_DIR/frontend"
    cp -r build/* "../$BUILD_OUTPUT_DIR/frontend/"

    cd - > /dev/null
    print_success "React frontend built successfully."
}

# Step 3: Integrate backend and frontend
function integrate_backend_frontend() {
    print_info "Integrating backend and frontend..."
    mkdir -p "$BUILD_OUTPUT_DIR"

    # Combine backend and frontend into a single output directory
    cp -r "$BUILD_OUTPUT_DIR/backend" "$BUILD_OUTPUT_DIR/"
    cp -r "$BUILD_OUTPUT_DIR/frontend" "$BUILD_OUTPUT_DIR/"

    print_success "Integration completed successfully."
}

# Step 4: Clean up previous builds
function clean_build() {
    print_info "Cleaning up previous builds..."
    rm -rf "$BUILD_OUTPUT_DIR"
    print_success "Clean up completed."
}

# Main script execution
function main() {
    print_info "Starting build process for Life Browser..."

    clean_build
    build_rust_backend
    build_react_frontend
    integrate_backend_frontend

    print_success "Build process completed successfully. Output available in '$BUILD_OUTPUT_DIR'."
}

# Error handling
trap 'print_error "An error occurred. Exiting..."; exit 1;' ERR

# Run the main function
main
