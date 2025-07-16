#!/bin/bash

# Simple validation test
echo "Testing validation script..."

# Change to project root directory
cd "$(dirname "$0")/../.."

# Test build
echo "Testing build..."
if cargo build --release --quiet; then
    echo "Build: PASS"
else
    echo "Build: FAIL"
    exit 1
fi

# Test boot
echo "Testing boot..."
BOOT_OUTPUT=$(timeout 5s qemu-system-aarch64 -M raspi4b -nographic -kernel target/aarch64-unknown-none/release/tiny_os 2>&1)
BOOT_EXIT_CODE=$?

if [[ $BOOT_EXIT_CODE -eq 124 || $BOOT_EXIT_CODE -eq 143 ]] && echo "$BOOT_OUTPUT" | grep -q "TinyOS Ready" && echo "$BOOT_OUTPUT" | grep -q "TinyOS Starting"; then
    echo "Boot: PASS"
else
    echo "Boot: FAIL"
    echo "Boot output: $BOOT_OUTPUT"
    echo "Boot exit code: $BOOT_EXIT_CODE"
fi

echo "Validation test complete!"
