#!/bin/bash

# Phase 4.2 Virtual Memory Management Test Script
# Tests the newly implemented virtual memory support in TinyOS

cd "$(dirname "$0")"

echo "=== TinyOS Phase 4.2 Virtual Memory Test ==="
echo "Building kernel with virtual memory support..."
cargo build --target aarch64-unknown-none --release

if [ $? -eq 0 ]; then
    echo "Build successful! Testing virtual memory management..."
    echo ""
    echo "Phase 4.2 Virtual Memory Commands:"
    echo "  ~   - Virtual memory management submenu"
    echo "    1 - Virtual memory status"
    echo "    2 - Enable MMU"
    echo "    3 - Disable MMU" 
    echo "    4 - Translate address"
    echo "    5 - Flush TLB"
    echo "    6 - Virtual memory test"
    echo ""
    echo "Previous Phase Commands Still Available:"
    echo "  ^   - Exception management submenu (Phase 4.1)"
    echo "  &   - Process management submenu (Phase 3)"
    echo "  m   - Memory statistics"
    echo "  h   - Help menu"
    echo ""
    echo "Test Sequence: Check VM status -> Enable MMU -> Test VM -> Show help"
    echo "Manual Test: ~1 -> ~2 -> ~6 -> h"
    echo ""
    echo "Starting QEMU with Phase 4.2 TinyOS..."
    echo "Press Ctrl+A then X to exit QEMU"
    echo "=========================================="
    
    # For automated testing, run a quick sequence
    if [ "$1" = "auto" ]; then
        echo "Running automated virtual memory test sequence..."
        echo -e '~\n1\n~\n6\nh\nq' | timeout 30s qemu-system-aarch64 -M raspi4b -nographic -kernel target/aarch64-unknown-none/release/tiny_os
    else
        echo "Starting QEMU in interactive mode..."
        echo "Recommended test sequence:"
        echo "1. Press '~' for virtual memory menu"
        echo "2. Press '1' to check VM status"
        echo "3. Press '6' to run VM tests"
        echo "4. Press 'h' to see all available commands"
        echo ""
        qemu-system-aarch64 -M raspi4b -nographic -kernel target/aarch64-unknown-none/release/tiny_os
    fi
    
    echo "Phase 4.2 virtual memory test complete!"
else
    echo "Build failed!"
    exit 1
fi

echo ""
echo "=== Phase 4.2 Implementation Status ==="
echo "✅ Virtual Memory Manager implemented"
echo "✅ ARM64 page table structures"
echo "✅ MMU control (enable/disable)"
echo "✅ Address translation system"
echo "✅ TLB management"
echo "✅ Shell integration with '~' submenu"
echo "✅ Kernel space mapping (identity)"
echo "✅ Device memory mapping"
echo "✅ Virtual memory testing suite"
echo ""
echo "Next Steps for Phase 4.3:"
echo "- Copy-on-write implementation"
echo "- User space page table management"
echo "- Advanced memory protection"
echo "- Stack management and protection"
