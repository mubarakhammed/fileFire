# FileFire Makefile
.PHONY: help build build-all test clean install dev docs lint format check-deps

# Default target
help:
	@echo "FileFire Document SDK - Available targets:"
	@echo ""
	@echo "  build        - Build core and plugins"
	@echo "  build-all    - Build for all platforms"
	@echo "  test         - Run all tests"
	@echo "  clean        - Clean build artifacts"
	@echo "  install      - Install dependencies"
	@echo "  dev          - Start development environment"
	@echo "  docs         - Generate documentation"
	@echo "  lint         - Run linting"
	@echo "  format       - Format code"
	@echo "  check-deps   - Check for missing dependencies"
	@echo ""
	@echo "Platform-specific builds:"
	@echo "  build-flutter - Build Flutter binding"
	@echo "  build-ios     - Build iOS binding"
	@echo "  build-android - Build Android binding"
	@echo "  build-wasm    - Build WebAssembly binding"
	@echo "  build-dotnet  - Build .NET binding"
	@echo "  build-cloud   - Build cloud API"
	@echo ""

# Core build targets
build:
	@echo "Building FileFire core and plugins..."
	cargo build --release --workspace

build-all: build build-flutter build-wasm build-cloud
	@echo "All builds completed successfully!"

# Testing
test:
	@echo "Running Rust tests..."
	cargo test --workspace
	@echo "Running Flutter tests..."
	cd bindings/flutter && flutter test
	@echo "Running example app builds..."
	$(MAKE) test-examples

test-examples:
	@echo "Testing Flutter example..."
	cd examples/flutter_app && flutter pub get && flutter analyze

# Platform-specific builds
build-flutter:
	@echo "Building Flutter binding..."
	cd bindings/flutter && flutter pub get && flutter analyze

build-ios:
	@echo "Building iOS binding..."
	# TODO: Add iOS build commands
	@echo "iOS build not yet implemented"

build-android:
	@echo "Building Android binding..."
	# TODO: Add Android build commands
	@echo "Android build not yet implemented"

build-wasm:
	@echo "Building WebAssembly binding..."
	cd core && wasm-pack build --target web --out-dir ../bindings/wasm/pkg

build-dotnet:
	@echo "Building .NET binding..."
	# TODO: Add .NET build commands
	@echo ".NET build not yet implemented"

build-cloud:
	@echo "Building cloud API..."
	cargo build --release --package filefire-cloud-api

# Development
dev:
	@echo "Starting development environment..."
	# Start cloud API in development mode
	RUST_LOG=debug cargo run --package filefire-cloud-api &
	# Start documentation server
	cd docs && python3 -m http.server 8001 &
	@echo "Development environment started:"
	@echo "  - Cloud API: http://localhost:3000"
	@echo "  - Documentation: http://localhost:8001"

# Documentation
docs:
	@echo "Generating documentation..."
	cargo doc --workspace --no-deps
	cd bindings/flutter && dart doc .
	@echo "Documentation generated in target/doc/"

# Code quality
lint:
	@echo "Running linters..."
	cargo clippy --workspace --all-targets --all-features -- -D warnings
	cd bindings/flutter && flutter analyze

format:
	@echo "Formatting code..."
	cargo fmt --all
	cd bindings/flutter && dart format .

# Dependency management
install:
	@echo "Installing dependencies..."
	$(MAKE) check-deps
	@echo "Installing Rust dependencies..."
	cargo build --workspace
	@echo "Installing Flutter dependencies..."
	cd bindings/flutter && flutter pub get
	cd examples/flutter_app && flutter pub get

check-deps:
	@echo "Checking dependencies..."
	@command -v cargo >/dev/null 2>&1 || { echo "❌ Rust/Cargo not found. Install from https://rustup.rs/"; exit 1; }
	@command -v flutter >/dev/null 2>&1 || { echo "❌ Flutter not found. Install from https://flutter.dev/"; exit 1; }
	@command -v docker >/dev/null 2>&1 || { echo "⚠️  Docker not found. Install for cloud development."; }
	@echo "✅ Core dependencies found"

# Clean targets
clean:
	@echo "Cleaning build artifacts..."
	cargo clean
	rm -rf bindings/wasm/pkg
	cd bindings/flutter && flutter clean
	cd examples/flutter_app && flutter clean

# Docker targets
docker-build:
	@echo "Building Docker image..."
	docker build -f cloud/docker/Dockerfile -t filefire/api:latest .

docker-run:
	@echo "Running Docker container..."
	docker run -p 3000:3000 filefire/api:latest

docker-compose-up:
	@echo "Starting Docker Compose stack..."
	cd cloud/docker && docker-compose up -d

docker-compose-down:
	@echo "Stopping Docker Compose stack..."
	cd cloud/docker && docker-compose down

# Release targets
release-check:
	@echo "Checking release readiness..."
	$(MAKE) test
	$(MAKE) lint
	$(MAKE) build-all
	@echo "✅ Release checks passed"

release-build:
	@echo "Building release artifacts..."
	cargo build --release --workspace
	$(MAKE) build-wasm
	$(MAKE) docker-build
	@echo "✅ Release artifacts built"

# Benchmarks
bench:
	@echo "Running benchmarks..."
	cargo bench --workspace

# Security audit
audit:
	@echo "Running security audit..."
	cargo audit

# Installation targets for system-wide installation
install-system:
	@echo "Installing FileFire system-wide..."
	cargo install --path core --force
	# TODO: Add system installation for bindings

# Development database setup (for cloud API)
setup-db:
	@echo "Setting up development database..."
	cd cloud/docker && docker-compose up -d postgres redis
	sleep 5
	# TODO: Run database migrations

# Generate API documentation
api-docs:
	@echo "Generating API documentation..."
	cd cloud/api && cargo doc --no-deps --open

# Performance testing
perf-test:
	@echo "Running performance tests..."
	# TODO: Add performance test suite

# Integration tests
integration-test:
	@echo "Running integration tests..."
	$(MAKE) docker-compose-up
	sleep 10
	# TODO: Run integration tests against running services
	$(MAKE) docker-compose-down

# Pre-commit hook
pre-commit: format lint test
	@echo "✅ Pre-commit checks passed"

# CI simulation
ci: check-deps build-all test lint
	@echo "✅ CI checks passed"