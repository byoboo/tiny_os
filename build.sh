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
    
    # Find the correct objcopy tool - prioritize LLVM tools
    OBJCOPY_TOOL=""
    RUST_TOOLCHAIN=$(rustc --print sysroot)
    echo "Rust toolchain: $RUST_TOOLCHAIN"
    
    # First, try to find objcopy in rustup toolchain directory (most reliable)
    POSSIBLE_PATHS=(
        "$RUST_TOOLCHAIN/lib/rustlib/x86_64-apple-darwin/bin/llvm-objcopy"
        "$RUST_TOOLCHAIN/lib/rustlib/aarch64-apple-darwin/bin/llvm-objcopy"
        "$RUST_TOOLCHAIN/lib/rustlib/x86_64-unknown-linux-gnu/bin/llvm-objcopy"
        "$RUST_TOOLCHAIN/lib/rustlib/aarch64-unknown-linux-gnu/bin/llvm-objcopy"
    )
    
    echo "Searching for LLVM objcopy in toolchain..."
    for path in "${POSSIBLE_PATHS[@]}"; do
        echo "  Checking: $path"
        if [ -f "$path" ]; then
            OBJCOPY_TOOL="$path"
            echo "  ✓ Found LLVM objcopy in toolchain: $path"
            break
        else
            echo "  ✗ Not found"
        fi
    done
    
    # If not found in toolchain, try system-wide LLVM tools
    if [ -z "$OBJCOPY_TOOL" ]; then
        echo "Searching for system-wide LLVM tools..."
        if command -v llvm-objcopy >/dev/null 2>&1; then
            OBJCOPY_TOOL="llvm-objcopy"
            echo "  ✓ Found system-wide llvm-objcopy: $(which llvm-objcopy)"
        elif command -v rust-objcopy >/dev/null 2>&1; then
            OBJCOPY_TOOL="rust-objcopy"
            echo "  ✓ Found rust-objcopy: $(which rust-objcopy)"
        else
            echo "  ✗ No system-wide LLVM tools found"
        fi
    fi
    
    # Last resort: try system objcopy (but warn this often fails with ARM64)
    if [ -z "$OBJCOPY_TOOL" ]; then
        echo "Checking for system objcopy as last resort..."
        if command -v objcopy >/dev/null 2>&1; then
            OBJCOPY_TOOL="objcopy"
            echo "  ⚠️  WARNING: Using system objcopy: $(which objcopy)"
            echo "     This may fail with ARM64 binaries!"
        else
            echo "  ✗ No system objcopy found"
        fi
    fi
    
    # Final check
    if [ -z "$OBJCOPY_TOOL" ]; then
        echo "❌ No objcopy tool found. Please make sure llvm-tools-preview is installed:"
        echo "   rustup component add llvm-tools-preview"
        echo ""
        echo "If the problem persists, try installing LLVM tools directly:"
        echo "   Ubuntu/Debian: sudo apt-get install llvm"
        echo "   macOS: brew install llvm"
        echo ""
        echo "✅ Kernel ELF file is ready at: target/aarch64-unknown-none/release/tiny_os"
        echo "   (You can use this directly with QEMU)"
        exit 1
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
