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
	@echo "  build       - Build TinyOS kernel (auto-extracts binary)"
	@echo "  build-local - Build and extract binary for local use"
	@echo "  build-pi    - Build kernel8.img for Raspberry Pi hardware"
	@echo "  extract-binary - Extract binary from container to host"
	@echo "  test        - Run complete test suite"
	@echo "  dev-cycle   - Quick build + test cycle"
	@echo "  validate-ci - Validate CI environment matches local"
	@echo ""
	@echo "Hardware Testing:"
	@echo "  run-local   - Run TinyOS locally with QEMU"
	@echo "  check-binary - Check if binary exists and show info"
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
	@echo "Extracting binary for local use..."
	@docker-compose run --rm build cp /workspace/target/aarch64-unknown-none/release/tiny_os /workspace/ 2>/dev/null || true
	@if [ -f tiny_os ]; then \
		echo "✅ Binary extracted: ./tiny_os ($(shell ls -lh tiny_os 2>/dev/null | cut -d' ' -f5))"; \
	else \
		echo "⚠️  Binary extraction failed - check Docker container"; \
	fi

# Extract binary from container to host
extract-binary:
	@echo "Extracting binary from Docker container..."
	@docker-compose run --rm build cp /workspace/target/aarch64-unknown-none/release/tiny_os /workspace/
	@if [ -f ./tiny_os ]; then \
		echo "✅ Binary extracted: ./tiny_os ($$(ls -lh ./tiny_os | cut -d' ' -f5))"; \
	else \
		echo "❌ Binary extraction failed"; \
	fi
	@echo "Extracting kernel8.img from Docker container..."
	@docker-compose run --rm build bash -c "if [ -f /workspace/kernel8.img ]; then cp /workspace/kernel8.img /workspace/; fi"
	@if [ -f ./kernel8.img ]; then \
		echo "✅ Raspberry Pi image extracted: ./kernel8.img ($$(ls -lh ./kernel8.img | cut -d' ' -f5))"; \
	else \
		echo "⚠️  kernel8.img not found (run 'make build-pi' first)"; \
	fi

# Build and extract in one step (alias for build)
build-local: build

# Create kernel8.img for Raspberry Pi hardware
build-pi:
	@echo "Creating kernel8.img for Raspberry Pi..."
	@docker-compose run --rm build bash -c "cd /workspace && ./build.sh"
	@if [ -f kernel8.img ]; then \
		echo "✅ Raspberry Pi image created: kernel8.img ($$(ls -lh kernel8.img | cut -d' ' -f5))"; \
	else \
		echo "❌ kernel8.img creation failed"; \
	fi

# Quick development cycle: build + test
dev-cycle: build test
	@echo "🚀 Development cycle complete!"

# Run TinyOS locally with QEMU (requires binary)
run-local:
	@if [ -f tiny_os ]; then \
		echo "🚀 Running TinyOS in QEMU..."; \
		echo "Press Ctrl+A then X to exit QEMU"; \
		echo "----------------------------------------"; \
		docker-compose run --rm dev bash -c "qemu-system-aarch64 -M raspi3b -kernel /workspace/tiny_os -serial stdio -display none -no-reboot -d guest_errors"; \
	else \
		echo "❌ TinyOS binary not found - run 'make build' first"; \
	fi

# Check if binary exists and show info
check-binary:
	@if [ -f tiny_os ]; then \
		echo "✅ TinyOS binary found: ./tiny_os"; \
		echo "   Size: $(shell ls -lh tiny_os | cut -d' ' -f5)"; \
		echo "   Modified: $(shell ls -l tiny_os | cut -d' ' -f6-8)"; \
		file tiny_os 2>/dev/null || echo "   Type: ARM64 executable"; \
	else \
		echo "❌ TinyOS binary not found"; \
		echo "   Run 'make build' to create it"; \
	fi
	@if [ -f kernel8.img ]; then \
		echo "✅ Raspberry Pi kernel found: ./kernel8.img"; \
		echo "   Size: $(shell ls -lh kernel8.img 2>/dev/null | cut -d' ' -f5)"; \
		echo "   Modified: $(shell ls -l kernel8.img 2>/dev/null | cut -d' ' -f6-8)"; \
	else \
		echo "❌ Raspberry Pi kernel not found"; \
		echo "   Run 'make build-pi' to create it"; \
	fi

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
	@echo "Cleaning local binaries..."
	@rm -f tiny_os kernel8.img
	@echo "Cleaning Docker volumes..."
	docker-compose down --volumes
	@echo "✅ Clean complete"

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
