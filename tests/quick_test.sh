#!/bin/bash

# Quick TinyOS Test
echo "=== Quick TinyOS Test ==="

# Test 1: Check if we can build
echo "Testing build..."
if cargo build --target aarch64-unknown-none >/dev/null 2>&1; then
    echo "✓ Build successful"
else
    echo "✗ Build failed"
    exit 1
fi

# Test 2: Check if binary exists and has reasonable size
BINARY="target/aarch64-unknown-none/debug/tiny_os"
if [[ -f "$BINARY" ]]; then
    SIZE=$(stat -c%s "$BINARY" 2>/dev/null || stat -f%z "$BINARY" 2>/dev/null || echo "0")
    if [[ $SIZE -gt 100000 ]]; then
        echo "✓ Binary exists and has reasonable size ($SIZE bytes)"
    else
        echo "✗ Binary too small ($SIZE bytes)"
        exit 1
    fi
else
    echo "✗ Binary not found"
    exit 1
fi

# Test 3: Check source files exist
echo "Checking source files..."
for file in src/main.rs src/boot.s src/uart.rs src/gpio.rs src/timer.rs src/memory.rs src/interrupts.rs; do
    if [[ -f "$file" ]]; then
        echo "  ✓ $file"
    else
        echo "  ✗ $file missing"
        exit 1
    fi
done

echo "✓ All quick tests passed!"
echo "========================"
