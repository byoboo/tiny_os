#!/bin/bash

# Simple Interrupt Test for TinyOS
# Just tests that interrupt system boots and is accessible

cd "$(dirname "$0")"

echo "=== Simple TinyOS Interrupt Test ==="
echo "Building kernel..."

# Build the kernel
if ! cargo build --target aarch64-unknown-none --release > /dev/null 2>&1; then
    echo "❌ Build failed!"
    exit 1
fi

echo "✅ Build successful"
echo "⚡ Running interrupt boot test..."

# Run QEMU and capture output
qemu-system-aarch64 -M raspi4b -nographic -kernel target/aarch64-unknown-none/release/tiny_os > /tmp/interrupt_test.log 2>&1 &
QEMU_PID=$!

# Wait for boot
sleep 8

# Kill QEMU
if kill -0 $QEMU_PID 2>/dev/null; then
    kill $QEMU_PID 2>/dev/null || true
    wait $QEMU_PID 2>/dev/null || true
fi

# Check results
if [ -f /tmp/interrupt_test.log ]; then
    echo "📋 Interrupt system analysis:"
    
    INTERRUPT_SUCCESS=true
    
    # Check for interrupt controller initialization
    if grep -q "Interrupt Controller initialized" /tmp/interrupt_test.log; then
        echo "✅ Interrupt controller initialized"
    else
        echo "❌ Interrupt controller initialization failed"
        INTERRUPT_SUCCESS=false
    fi
    
    # Check that interrupt commands are available
    if grep -q "Type 'h' for help" /tmp/interrupt_test.log; then
        echo "✅ Interrupt commands accessible via shell"
    else
        echo "❌ Shell not accessible for interrupt commands"
        INTERRUPT_SUCCESS=false
    fi
    
    # Check for any interrupt-related errors
    if grep -q "Interrupt.*FAIL\|Interrupt.*ERROR\|PANIC.*interrupt" /tmp/interrupt_test.log; then
        echo "❌ Interrupt errors detected"
        INTERRUPT_SUCCESS=false
    fi
    
    if [ "$INTERRUPT_SUCCESS" = true ]; then
        echo "🎉 Interrupt system test PASSED"
        rm -f /tmp/interrupt_test.log
        exit 0
    else
        echo "💥 Interrupt system test FAILED"
        echo "Boot log:"
        cat /tmp/interrupt_test.log
        rm -f /tmp/interrupt_test.log
        exit 1
    fi
else
    echo "❌ No test output captured"
    exit 1
fi
