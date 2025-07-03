#!/bin/bash

# Simple Memory Test for TinyOS
# Just tests that memory system boots and is accessible

cd "$(dirname "$0")"

echo "=== Simple TinyOS Memory Test ==="
echo "Building kernel..."

# Build the kernel
if ! cargo build --target aarch64-unknown-none --release > /dev/null 2>&1; then
    echo "❌ Build failed!"
    exit 1
fi

echo "✅ Build successful"
echo "🧠 Running memory boot test..."

# Run QEMU and capture output
qemu-system-aarch64 -M raspi4b -nographic -kernel target/aarch64-unknown-none/release/tiny_os > /tmp/memory_test.log 2>&1 &
QEMU_PID=$!

# Wait for boot
sleep 8

# Kill QEMU
if kill -0 $QEMU_PID 2>/dev/null; then
    kill $QEMU_PID 2>/dev/null || true
    wait $QEMU_PID 2>/dev/null || true
fi

# Check results
if [ -f /tmp/memory_test.log ]; then
    echo "📋 Memory system analysis:"
    
    MEMORY_SUCCESS=true
    
    # Check for memory manager initialization
    if grep -q "Memory Manager initialized" /tmp/memory_test.log; then
        echo "✅ Memory manager initialized"
    else
        echo "❌ Memory manager initialization failed"
        MEMORY_SUCCESS=false
    fi
    
    # Check for heap setup
    if grep -q "Heap Size.*bytes" /tmp/memory_test.log; then
        echo "✅ Heap configured successfully"
        HEAP_SIZE=$(grep "Heap Size" /tmp/memory_test.log | sed 's/.*Heap Size: \([0-9]*\) bytes.*/\1/')
        echo "   Heap size: $HEAP_SIZE bytes"
    else
        echo "❌ Heap configuration failed"
        MEMORY_SUCCESS=false
    fi
    
    # Check that memory commands are available
    if grep -q "Type 'h' for help" /tmp/memory_test.log; then
        echo "✅ Memory commands accessible via shell"
    else
        echo "❌ Shell not accessible for memory commands"
        MEMORY_SUCCESS=false
    fi
    
    # Check for any memory-related errors
    if grep -q "Memory.*FAIL\|Memory.*ERROR\|PANIC.*memory" /tmp/memory_test.log; then
        echo "❌ Memory errors detected"
        MEMORY_SUCCESS=false
    fi
    
    if [ "$MEMORY_SUCCESS" = true ]; then
        echo "🎉 Memory system test PASSED"
        rm -f /tmp/memory_test.log
        exit 0
    else
        echo "💥 Memory system test FAILED"
        echo "Boot log:"
        cat /tmp/memory_test.log
        rm -f /tmp/memory_test.log
        exit 1
    fi
else
    echo "❌ No test output captured"
    exit 1
fi
