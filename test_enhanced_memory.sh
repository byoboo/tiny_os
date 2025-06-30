#!/bin/bash

cd "$(dirname "$0")"

echo "=== TinyOS Enhanced Memory Management Test ==="
echo "Building kernel in release mode..."
cargo build --target aarch64-unknown-none --release

if [ $? -eq 0 ]; then
    echo "Build successful! Testing enhanced memory management..."
    echo ""
    echo "Commands to test:"
    echo "  h - Help menu (shows new memory commands)"
    echo "  m - Show detailed memory statistics"
    echo "  a - Allocate memory blocks"
    echo "  f - Free allocated blocks"
    echo "  g - Memory corruption check"
    echo "  r - Defragment memory"
    echo "  x - Run memory test"
    echo "  c - Full system health check (includes enhanced memory test)"
    echo ""
    echo "Starting QEMU with enhanced TinyOS..."
    echo "Press Ctrl+A then X to exit QEMU"
    echo "----------------------------------------"
    
    qemu-system-aarch64 -M raspi4b -nographic -kernel target/aarch64-unknown-none/release/tiny_os
    
    echo "Enhanced memory test complete!"
else
    echo "Build failed!"
    exit 1
fi
