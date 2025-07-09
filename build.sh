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
    echo "Installing llvm-tools-preview component..."
    rustup component add llvm-tools-preview
    
    # Find the correct objcopy tool
    OBJCOPY_TOOL=""
    
    # Try different objcopy tools in order of preference
    if command -v llvm-objcopy >/dev/null 2>&1; then
        OBJCOPY_TOOL="llvm-objcopy"
    elif command -v rust-objcopy >/dev/null 2>&1; then
        OBJCOPY_TOOL="rust-objcopy"
    elif command -v objcopy >/dev/null 2>&1; then
        OBJCOPY_TOOL="objcopy"
    else
        # Try to find objcopy in rustup toolchain directory
        RUST_TOOLCHAIN=$(rustc --print sysroot)
        if [ -f "$RUST_TOOLCHAIN/lib/rustlib/x86_64-apple-darwin/bin/llvm-objcopy" ]; then
            OBJCOPY_TOOL="$RUST_TOOLCHAIN/lib/rustlib/x86_64-apple-darwin/bin/llvm-objcopy"
        elif [ -f "$RUST_TOOLCHAIN/lib/rustlib/aarch64-apple-darwin/bin/llvm-objcopy" ]; then
            OBJCOPY_TOOL="$RUST_TOOLCHAIN/lib/rustlib/aarch64-apple-darwin/bin/llvm-objcopy"
        else
            echo "❌ No objcopy tool found. Please make sure llvm-tools-preview is installed:"
            echo "   rustup component add llvm-tools-preview"
            echo ""
            echo "If the problem persists, try installing LLVM tools directly:"
            echo "   brew install llvm"
            echo ""
            echo "✅ Kernel ELF file is ready at: target/aarch64-unknown-none/release/tiny_os"
            echo "   (You can use this directly with QEMU)"
            exit 1
        fi
    fi
    
    echo "Using objcopy tool: $OBJCOPY_TOOL"
    $OBJCOPY_TOOL -j .text -O binary target/aarch64-unknown-none/release/tiny_os kernel8.img
    
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
