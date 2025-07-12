# TinyOS Development Makefile
# Provides standardized development tasks with Docker integration

.PHONY: help setup build test clean format lint dev-shell ci-shell validate-ci

# Default target
help:
	@echo "TinyOS Development Commands:"
	@echo ""
	@echo "Docker Environment:"
	@echo "  setup       - Build Docker development environment"
	@echo "  dev-shell   - Enter interactive development shell"
	@echo "  ci-shell    - Enter CI environment shell"
	@echo ""
	@echo "Build & Test:"
	@echo "  build       - Build TinyOS kernel"
	@echo "  test        - Run complete test suite"
	@echo "  validate-ci - Validate CI environment matches local"
	@echo ""
	@echo "Code Quality:"
	@echo "  format      - Format Rust code"
	@echo "  lint        - Run clippy linter"
	@echo "  clean       - Clean build artifacts"
	@echo ""
	@echo "Legacy Support:"
	@echo "  build-host  - Build on host (requires Rust nightly)"
	@echo "  test-host   - Test on host (requires QEMU)"

# Docker environment setup
setup:
	@echo "Building Docker development environment..."
	docker-compose build dev

# Interactive development shell
dev-shell:
	@echo "Starting development shell..."
	docker-compose run --rm dev /bin/bash

# CI environment shell
ci-shell:
	@echo "Starting CI environment shell..."
	docker-compose run --rm ci /bin/bash

# Build targets
build:
	@echo "Building TinyOS kernel..."
	docker-compose run --rm build

# Test targets
test:
	@echo "Running complete test suite..."
	docker-compose run --rm test

# CI validation
validate-ci:
	@echo "Validating CI environment..."
	docker-compose run --rm ci ./test_tinyos.sh --validate-only

# Code quality
format:
	@echo "Formatting Rust code..."
	docker-compose run --rm dev cargo fmt

lint:
	@echo "Running Clippy linter..."
	docker-compose run --rm dev cargo clippy --all-targets --all-features -- -D warnings

# Clean build artifacts
clean:
	@echo "Cleaning build artifacts..."
	docker-compose run --rm dev cargo clean
	docker-compose down --volumes

# Legacy host-based commands (for backwards compatibility)
build-host:
	@echo "Building on host (legacy mode)..."
	@if ! command -v rustc >/dev/null 2>&1; then \
		echo "Error: Rust not found. Use 'make build' for Docker-based build."; \
		exit 1; \
	fi
	./build.sh

test-host:
	@echo "Testing on host (legacy mode)..."
	@if ! command -v qemu-system-aarch64 >/dev/null 2>&1; then \
		echo "Error: QEMU not found. Use 'make test' for Docker-based testing."; \
		exit 1; \
	fi
	./test_tinyos.sh

# Quick development workflow
dev-quick: build test
	@echo "Quick development cycle complete!"

# CI simulation
ci-local:
	@echo "Running local CI simulation..."
	docker-compose run --rm ci ./test_tinyos.sh
	@echo "CI simulation complete!"

# Development environment status
status:
	@echo "Docker Environment Status:"
	@docker-compose ps
	@echo ""
	@echo "Docker Images:"
	@docker images | grep tiny_os || echo "No TinyOS images found"
	@echo ""
	@echo "Docker Volumes:"
	@docker volume ls | grep tiny_os || echo "No TinyOS volumes found"
