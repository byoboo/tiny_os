#!/bin/bash

cd "$(dirname "$0")"

echo "=== TinyOS Comprehensive Memory Testing ==="
echo "Building kernel with enhanced memory tests..."
cargo build --target aarch64-unknown-none --release

if [ $? -eq 0 ]; then
    echo ""
    echo "Build successful! Starting TinyOS with memory testing..."
    echo ""
    echo "Available Memory Test Commands:"
    echo "  h - Help menu"
    echo "  m - Show memory statistics"
    echo "  x - Run basic memory test"
    echo "  z - Run COMPREHENSIVE memory test suite (NEW!)"
    echo "  g - Memory corruption check"
    echo "  r - Defragment memory"
    echo "  c - Full system health check (includes all memory tests)"
    echo ""
    echo "The comprehensive test suite (z) includes:"
    echo "  1. Basic allocation/deallocation test"
    echo "  2. Memory stress test with 50 blocks"
    echo "  3. Boundary and alignment testing"
    echo "  4. Multi-block allocation testing"
    echo "  5. Memory corruption detection"
    echo ""
    echo "Starting QEMU... Press Ctrl+A then X to exit"
    echo "============================================="
    
    qemu-system-aarch64 -M raspi4b -nographic -kernel target/aarch64-unknown-none/release/tiny_os
    
    echo ""
    echo "Memory testing session complete!"
else
    echo "Build failed!"
    exit 1
fi
