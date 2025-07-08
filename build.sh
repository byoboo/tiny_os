#!/bin/bash

# Build script for TinyOS Raspberry Pi kernel

echo "Building TinyOS kernel for Raspberry Pi 4/5..."

# Build the kernel
cargo build --release --target aarch64-unknown-none

if [ $? -eq 0 ]; then
    echo "Build successful!"
    
    # Extract executable code for Pi firmware (CRITICAL: only .text section)
    echo "Creating kernel binary for Raspberry Pi..."
    
    # Check if llvm-tools-preview is installed, install if needed
    if ! command -v rust-objcopy >/dev/null 2>&1; then
        echo "Installing llvm-tools-preview component..."
        rustup component add llvm-tools-preview
        if [ $? -ne 0 ]; then
            echo "Failed to install llvm-tools-preview. Trying alternative methods..."
            
            # Try using llvm-objcopy directly
            if command -v llvm-objcopy >/dev/null 2>&1; then
                llvm-objcopy -j .text -O binary target/aarch64-unknown-none/release/tiny_os kernel8.img
            elif command -v objcopy >/dev/null 2>&1; then
                objcopy -j .text -O binary target/aarch64-unknown-none/release/tiny_os kernel8.img
            else
                echo "❌ No objcopy tool available. Please install llvm-tools-preview:"
                echo "   rustup component add llvm-tools-preview"
                echo ""
                echo "✅ Kernel ELF file is ready at: target/aarch64-unknown-none/release/tiny_os"
                echo "   (You can use this directly with QEMU)"
                exit 1
            fi
        else
            # llvm-tools-preview installed successfully, try again
            rust-objcopy -j .text -O binary target/aarch64-unknown-none/release/tiny_os kernel8.img
        fi
    else
        # rust-objcopy is available
        rust-objcopy -j .text -O binary target/aarch64-unknown-none/release/tiny_os kernel8.img
    fi
    
    if [ -f kernel8.img ]; then
        echo "Kernel image created: kernel8.img ($(ls -lh kernel8.img | cut -d' ' -f5))"
    else
        echo "❌ Failed to create kernel8.img"
        echo "✅ Kernel ELF file is ready at: target/aarch64-unknown-none/release/tiny_os"
        echo "   (You can use this directly with QEMU)"
        exit 1
    fi
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
