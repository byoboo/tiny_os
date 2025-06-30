#!/bin/bash

# Run script for TinyOS kernel in QEMU

echo "Building and running TinyOS kernel..."

# Build the kernel
cargo build

if [ $? -eq 0 ]; then
    echo "Build successful! Starting QEMU..."
    echo "Press Ctrl+A then X to exit QEMU"
    echo "----------------------------------------"
    
    # Run in QEMU
    qemu-system-aarch64 \
        -M raspi4b \
        -kernel target/aarch64-unknown-none/debug/tiny_os \
        -serial stdio \
        -display none \
        -no-reboot \
        -d guest_errors
else
    echo "Build failed!"
    exit 1
fi
