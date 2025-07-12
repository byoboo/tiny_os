#!/bin/bash

# Run script for TinyOS kernel in QEMU
# Supports both host and Docker environments

# Docker environment detection
if [ -f /.dockerenv ]; then
    echo "🐳 Building and running TinyOS kernel in Docker..."
    DOCKER_ENV=true
else
    echo "🏠 Building and running TinyOS kernel on host..."
    DOCKER_ENV=false
fi

echo "Building TinyOS kernel..."

# Build the kernel
cargo build

if [ $? -eq 0 ]; then
    echo "Build successful! Starting QEMU..."
    echo "Press Ctrl+A then X to exit QEMU"
    echo "----------------------------------------"
    
    # Check if QEMU is available
    if ! command -v qemu-system-aarch64 >/dev/null 2>&1; then
        echo "❌ QEMU not found!"
        if [ "$DOCKER_ENV" = true ]; then
            echo "Docker environment should include QEMU. Please rebuild container."
        else
            echo "Please install QEMU:"
            echo "  Ubuntu/Debian: sudo apt-get install qemu-system-arm"
            echo "  macOS: brew install qemu"
            echo "  Or use Docker: make dev-shell"
        fi
        exit 1
    fi
    
    # Run in QEMU - Raspberry Pi 4B
    qemu-system-aarch64 \
        -M raspi4b \
        -kernel target/aarch64-unknown-none/debug/tiny_os \
        -serial stdio \
        -display none \
        -no-reboot \
        -d guest_errors
else
    echo "Build failed!"
    if [ "$DOCKER_ENV" = true ]; then
        echo "Docker build failure. Check container setup."
    else
        echo "Host build failure. Try 'make build' for containerized build."
    fi
    exit 1
fi
