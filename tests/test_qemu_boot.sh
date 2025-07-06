#!/bin/bash

# Simple QEMU Boot Test for TinyOS
# Tests that TinyOS boots successfully without user interaction

# Change to project root directory
cd "$(dirname "$0")/.."

echo "=== TinyOS Boot Test ==="
echo "Building kernel..."

# Build the kernel
if ! cargo build --target aarch64-unknown-none --release > /dev/null 2>&1; then
    echo "❌ Build failed!"
    exit 1
fi

echo "✅ Build successful"
echo "🚀 Testing TinyOS boot..."

# Run QEMU with timeout and capture output
# Use gtimeout if available (from coreutils), otherwise fallback to built-in method
if command -v gtimeout >/dev/null 2>&1; then
    gtimeout 15s qemu-system-aarch64 -M raspi4b -nographic -kernel target/aarch64-unknown-none/release/tiny_os > /tmp/boot_test.log 2>&1 &
    QEMU_PID=$!
    wait $QEMU_PID
elif command -v timeout >/dev/null 2>&1; then
    timeout 15s qemu-system-aarch64 -M raspi4b -nographic -kernel target/aarch64-unknown-none/release/tiny_os > /tmp/boot_test.log 2>&1 &
    QEMU_PID=$!
    wait $QEMU_PID
else
    # Fallback: start QEMU and kill after timeout
    qemu-system-aarch64 -M raspi4b -nographic -kernel target/aarch64-unknown-none/release/tiny_os > /tmp/boot_test.log 2>&1 &
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
    if grep -q "TinyOS v0.1.0" /tmp/boot_test.log; then
        echo "✅ TinyOS version banner found"
        BOOT_SUCCESS=true
    else
        echo "❌ TinyOS version banner missing"
    fi
    
    if grep -q "Type 'h' for help" /tmp/boot_test.log; then
        echo "✅ Interactive shell started"
        BOOT_SUCCESS=true
    else
        echo "❌ Interactive shell not detected"
    fi
    
    if grep -q "UART System: ✓ PASS" /tmp/boot_test.log; then
        echo "✅ UART system initialized"
    fi
    
    if grep -q "GPIO System: ✓ PASS" /tmp/boot_test.log; then
        echo "✅ GPIO system initialized"
    fi
    
    if grep -q "Timer System: ✓ PASS" /tmp/boot_test.log; then
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
