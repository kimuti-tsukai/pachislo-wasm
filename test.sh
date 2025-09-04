#!/bin/bash

# Pachislo WASM Test Runner Script
# This script provides convenient commands for running different types of tests

set -e  # Exit on any error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Print colored output
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if required tools are installed
check_dependencies() {
    print_status "Checking dependencies..."

    if ! command -v cargo &> /dev/null; then
        print_error "cargo is not installed. Please install Rust."
        exit 1
    fi

    if ! command -v wasm-pack &> /dev/null; then
        print_warning "wasm-pack is not installed. Installing..."
        curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
    fi

    # Check if wasm32 target is installed
    if ! rustup target list --installed | grep -q "wasm32-unknown-unknown"; then
        print_status "Installing wasm32-unknown-unknown target..."
        rustup target add wasm32-unknown-unknown
    fi

    print_success "All dependencies are ready"
}

# Run unit tests
run_unit_tests() {
    print_status "Running unit tests..."
    cargo test --lib --verbose
    print_success "Unit tests completed"
}

# Run integration tests
run_integration_tests() {
    print_status "Running integration tests..."
    cargo test --test integration_tests --verbose
    print_success "Integration tests completed"
}

# Run benchmark tests
run_benchmark_tests() {
    print_status "Running benchmark tests..."
    cargo test --test benchmark_tests --verbose
    print_success "Benchmark tests completed"
}

# Run all Rust tests
run_all_rust_tests() {
    print_status "Running all Rust tests..."
    cargo test --verbose
    print_success "All Rust tests completed"
}

# Run WASM tests in browser
run_wasm_tests() {
    local browser=${1:-"chrome"}
    local headless=${2:-"--headless"}

    print_status "Running WASM tests in $browser..."

    case $browser in
        "chrome")
            wasm-pack test --chrome $headless
            ;;
        "firefox")
            wasm-pack test --firefox $headless
            ;;
        "safari")
            wasm-pack test --safari $headless
            ;;
        *)
            print_error "Unsupported browser: $browser"
            print_status "Supported browsers: chrome, firefox, safari"
            exit 1
            ;;
    esac

    print_success "WASM tests completed"
}

# Run linting and formatting checks
run_lint() {
    print_status "Running linting checks..."

    # Check formatting
    cargo fmt -- --check || {
        print_warning "Code formatting issues found. Run 'cargo fmt' to fix."
    }

    # Run clippy
    cargo clippy -- -D warnings

    print_success "Linting completed"
}

# Build the project
build_project() {
    print_status "Building project..."
    cargo build --verbose
    print_success "Build completed"
}

# Build WASM package
build_wasm() {
    local target=${1:-"web"}

    print_status "Building WASM package for $target..."

    case $target in
        "web")
            wasm-pack build --target web --out-dir pkg
            ;;
        "nodejs")
            wasm-pack build --target nodejs --out-dir pkg-node
            ;;
        "bundler")
            wasm-pack build --target bundler --out-dir pkg-bundler
            ;;
        *)
            print_error "Unsupported target: $target"
            print_status "Supported targets: web, nodejs, bundler"
            exit 1
            ;;
    esac

    print_success "WASM build completed for $target"
}

# Clean build artifacts
clean() {
    print_status "Cleaning build artifacts..."
    cargo clean
    rm -rf pkg pkg-node pkg-bundler
    print_success "Clean completed"
}

# Run comprehensive test suite
run_comprehensive_tests() {
    print_status "Running comprehensive test suite..."

    # Build first
    build_project

    # Run all Rust tests
    run_all_rust_tests

    # Run linting
    run_lint

    # Run WASM tests if possible
    if command -v google-chrome &> /dev/null || command -v chromium-browser &> /dev/null; then
        run_wasm_tests "chrome" "--headless"
    else
        print_warning "Chrome not found, skipping WASM tests"
    fi

    print_success "Comprehensive test suite completed"
}

# Show usage information
show_help() {
    echo "Pachislo WASM Test Runner"
    echo ""
    echo "Usage: $0 [COMMAND] [OPTIONS]"
    echo ""
    echo "Commands:"
    echo "  unit                Run unit tests only"
    echo "  integration         Run integration tests only"
    echo "  benchmark          Run benchmark tests only"
    echo "  rust               Run all Rust tests"
    echo "  wasm [browser]     Run WASM tests (chrome|firefox|safari)"
    echo "  lint               Run linting and formatting checks"
    echo "  build              Build the project"
    echo "  build-wasm [target] Build WASM package (web|nodejs|bundler)"
    echo "  clean              Clean build artifacts"
    echo "  comprehensive      Run all tests and checks"
    echo "  deps               Check and install dependencies"
    echo "  help               Show this help message"
    echo ""
    echo "Examples:"
    echo "  $0 unit                    # Run unit tests"
    echo "  $0 wasm chrome             # Run WASM tests in Chrome"
    echo "  $0 wasm firefox --no-headless  # Run WASM tests in Firefox with GUI"
    echo "  $0 build-wasm nodejs       # Build for Node.js target"
    echo "  $0 comprehensive           # Run everything"
    echo ""
}

# Main command processing
main() {
    case ${1:-help} in
        "unit")
            check_dependencies
            run_unit_tests
            ;;
        "integration")
            check_dependencies
            run_integration_tests
            ;;
        "benchmark")
            check_dependencies
            run_benchmark_tests
            ;;
        "rust")
            check_dependencies
            run_all_rust_tests
            ;;
        "wasm")
            check_dependencies
            local headless_flag="--headless"
            if [[ "$3" == "--no-headless" ]]; then
                headless_flag=""
            fi
            run_wasm_tests "${2:-chrome}" "$headless_flag"
            ;;
        "lint")
            check_dependencies
            run_lint
            ;;
        "build")
            check_dependencies
            build_project
            ;;
        "build-wasm")
            check_dependencies
            build_wasm "${2:-web}"
            ;;
        "clean")
            clean
            ;;
        "comprehensive")
            check_dependencies
            run_comprehensive_tests
            ;;
        "deps")
            check_dependencies
            ;;
        "help"|"-h"|"--help")
            show_help
            ;;
        *)
            print_error "Unknown command: $1"
            echo ""
            show_help
            exit 1
            ;;
    esac
}

# Run main function with all arguments
main "$@"
