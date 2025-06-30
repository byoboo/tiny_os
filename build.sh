#!/bin/bash

# Build script for TinyOS Raspberry Pi kernel

echo "Building TinyOS kernel for Raspberry Pi 4/5..."

# Build the kernel
cargo build --release

if [ $? -eq 0 ]; then
    echo "Build successful!"
    
    # Copy kernel to a standard location
    cp target/aarch64-raspi/release/tiny_os kernel8.img
    
    echo "Kernel image created: kernel8.img"
    echo ""
    echo "To run in QEMU:"
    echo "qemu-system-aarch64 -M raspi4 -kernel kernel8.img -serial stdio -display none"
    echo ""
    echo "Or use: cargo run"
else
    echo "Build failed!"
    exit 1
fi
