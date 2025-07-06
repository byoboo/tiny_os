#!/bin/bash

# Build script for TinyOS Raspberry Pi kernel

echo "Building TinyOS kernel for Raspberry Pi 4/5..."

# Build the kernel
cargo build --release --target aarch64-unknown-none

if [ $? -eq 0 ]; then
    echo "Build successful!"
    
    # Extract executable code for Pi firmware (CRITICAL: only .text section)
    echo "Creating kernel binary for Raspberry Pi..."
    rust-objcopy -j .text -O binary target/aarch64-unknown-none/release/tiny_os kernel8.img
    
    echo "Kernel image created: kernel8.img ($(ls -lh kernel8.img | cut -d' ' -f5))"
    echo ""
    echo "To run in QEMU:"
    echo "qemu-system-aarch64 -M raspi4b -kernel target/aarch64-unknown-none/release/tiny_os -serial stdio -display none"
    echo ""
    echo "To deploy to Raspberry Pi:"
    echo "1. Copy kernel8.img to SD card root"
    echo "2. Ensure config.txt, firmware files, and device tree are present"
    echo ""
    echo "Or use: cargo run"
else
    echo "Build failed!"
    exit 1
fi
