#!/bin/bash

# Simple QEMU Boot Test for TinyOS
# Tests that TinyOS boots successfully without user interaction

# Change to project root directory
cd "$(dirname "$0")/.."

echo "=== TinyOS Boot Test ==="
echo "Building kernel..."

# Build the kernel
# Use Pi 3 compatible build for QEMU emulation
if ! cargo build --target aarch64-unknown-none --release --features raspi3 > /dev/null 2>&1; then
    echo "❌ Build failed!"
    exit 1
fi

echo "✅ Build successful"
echo "🚀 Testing TinyOS boot..."

# Check if qemu-system-aarch64 is available
if ! command -v qemu-system-aarch64 >/dev/null 2>&1; then
    echo "❌ qemu-system-aarch64 not found"
    echo "Available QEMU commands:"
    ls -la /usr/bin/qemu-* 2>/dev/null || echo "No QEMU found"
    exit 1
fi

echo "📋 QEMU version: $(qemu-system-aarch64 --version | head -1)"

# Show available machine types for debugging
echo "📋 Available machine types:"
qemu-system-aarch64 -machine help | grep -E "(raspi|virt)" | head -5

# Determine the best machine type to use
# For QEMU 8.2.2 (Ubuntu 24.04), prioritize supported ARM64 machines
MACHINE_TYPE=""
if qemu-system-aarch64 -machine help | grep -q "raspi3b"; then
    MACHINE_TYPE="raspi3b"
    echo "📋 Using Raspberry Pi 3B emulation (Cortex-A53)"
elif qemu-system-aarch64 -machine help | grep -q "raspi3ap"; then
    MACHINE_TYPE="raspi3ap"
    echo "📋 Using Raspberry Pi 3A+ emulation (Cortex-A53)"
elif qemu-system-aarch64 -machine help | grep -q "raspi4b"; then
    MACHINE_TYPE="raspi4b"
    echo "📋 Using Raspberry Pi 4B emulation (Cortex-A72)"
elif qemu-system-aarch64 -machine help | grep -q "virt"; then
    MACHINE_TYPE="virt"
    echo "⚠️  Using generic virt machine (no Pi-specific hardware)"
else
    echo "❌ No compatible ARM64 machine type found"
    echo "Available machine types:"
    qemu-system-aarch64 -machine help | head -10
    exit 1
fi

echo "📋 Using machine type: $MACHINE_TYPE"

# Run QEMU with timeout and capture output
# Use gtimeout if available (from coreutils), otherwise fallback to built-in method
if command -v gtimeout >/dev/null 2>&1; then
    gtimeout 15s qemu-system-aarch64 -M $MACHINE_TYPE -nographic -kernel target/aarch64-unknown-none/release/tiny_os > /tmp/boot_test.log 2>&1 &
    QEMU_PID=$!
    wait $QEMU_PID
elif command -v timeout >/dev/null 2>&1; then
    timeout 15s qemu-system-aarch64 -M $MACHINE_TYPE -nographic -kernel target/aarch64-unknown-none/release/tiny_os > /tmp/boot_test.log 2>&1 &
    QEMU_PID=$!
    wait $QEMU_PID
else
    # Fallback: start QEMU and kill after timeout
    qemu-system-aarch64 -M $MACHINE_TYPE -nographic -kernel target/aarch64-unknown-none/release/tiny_os > /tmp/boot_test.log 2>&1 &
    QEMU_PID=$!
    
    # Wait for boot or timeout (15 seconds)
    sleep 15
    
    # Kill QEMU if still running
    if kill -0 $QEMU_PID 2>/dev/null; then
        kill $QEMU_PID 2>/dev/null || true
        wait $QEMU_PID 2>/dev/null || true
    fi
fi

# Check boot output
if [ -f /tmp/boot_test.log ]; then
    echo "📋 Boot output analysis:"
    
    # Check for successful boot indicators
    BOOT_SUCCESS=false
    if grep -q "TinyOS Starting\|TinyOS Ready" /tmp/boot_test.log; then
        echo "✅ TinyOS boot sequence found"
        BOOT_SUCCESS=true
    else
        echo "❌ TinyOS boot sequence missing"
    fi
    
    if grep -q "Available commands (type 'h' for help)" /tmp/boot_test.log; then
        echo "✅ Interactive shell started"
        BOOT_SUCCESS=true
    else
        echo "❌ Interactive shell not detected"
    fi
    
    if grep -q "✓ UART\|UART initialized\|✓ GPIO\|GPIO initialized" /tmp/boot_test.log; then
        echo "✅ Hardware systems initialized"
    fi
    
    if grep -q "✓ System timer\|Timer.*initialized" /tmp/boot_test.log; then
        echo "✅ Timer system initialized"
    fi
    
    # Check for any panics or errors
    if grep -q "PANIC\|ERROR\|FAIL" /tmp/boot_test.log; then
        echo "❌ Errors detected in boot log"
        echo "Error details:"
        grep "PANIC\|ERROR\|FAIL" /tmp/boot_test.log
        BOOT_SUCCESS=false
    fi
    
    if [ "$BOOT_SUCCESS" = true ]; then
        echo "🎉 Boot test PASSED"
        rm -f /tmp/boot_test.log
        exit 0
    else
        echo "💥 Boot test FAILED"
        echo "Full boot log:"
        cat /tmp/boot_test.log
        rm -f /tmp/boot_test.log
        exit 1
    fi
else
    echo "❌ No boot output captured"
    exit 1
fi
