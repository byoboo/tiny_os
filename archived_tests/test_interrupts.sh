#!/bin/bash

cd "$(dirname "$0")"

echo "=== TinyOS Interrupt Management Testing ==="
echo "Building kernel with interrupt support..."
cargo build --target aarch64-unknown-none --release

if [ $? -eq 0 ]; then
    echo ""
    echo "Build successful! Starting TinyOS with interrupt management..."
    echo ""
    echo "New Interrupt Commands Available:"
    echo "  h - Help menu (shows all commands including interrupts)"
    echo "  i - Show interrupt status and statistics"
    echo "  e - Enable all major interrupt sources"
    echo "  j - Run comprehensive interrupt test"
    echo "  c - System health check (now includes interrupt testing)"
    echo "  s - System information (includes interrupt stats)"
    echo "  d - Hardware diagnostics (includes interrupt info)"
    echo ""
    echo "Interrupt Features:"
    echo "  - ARM Generic Interrupt Controller (GIC) simulation"
    echo "  - Timer, UART, and GPIO interrupt support"
    echo "  - Interrupt statistics and monitoring"
    echo "  - Comprehensive interrupt testing"
    echo "  - Enable/disable interrupt management"
    echo ""
    echo "Starting QEMU... Press Ctrl+A then X to exit"
    echo "============================================="
    
    qemu-system-aarch64 -M raspi4b -nographic -kernel target/aarch64-unknown-none/release/tiny_os
    
    echo ""
    echo "Interrupt testing session complete!"
else
    echo "Build failed!"
    exit 1
fi
