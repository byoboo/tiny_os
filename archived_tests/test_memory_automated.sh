#!/bin/bash

# Automated Memory Test for TinyOS
# Non-interactive version for test automation

cd "$(dirname "$0")"

echo "=== Automated TinyOS Memory Test ==="
echo "Building kernel..."

# Build the kernel
if ! cargo build --target aarch64-unknown-none --release > /dev/null 2>&1; then
    echo "❌ Build failed!"
    exit 1
fi

echo "✅ Build successful"
echo "🧠 Running automated memory tests..."

# Create a temporary expect script to automate the QEMU interaction
cat > /tmp/memory_test_script.exp << 'EOF'
#!/usr/bin/expect -f
set timeout 30

# Start QEMU
spawn qemu-system-aarch64 -M raspi4b -nographic -kernel target/aarch64-unknown-none/release/tiny_os

# Wait for the system to boot and show the prompt
expect {
    timeout { 
        puts "❌ Boot timeout"
        exit 1 
    }
    "Type 'h' for help" {
        puts "✅ System booted successfully"
    }
}

# Run comprehensive memory test
send "z\r"
expect {
    timeout { 
        puts "❌ Memory test timeout"
        exit 1 
    }
    "All memory tests: ✓ PASS" {
        puts "✅ Comprehensive memory test passed"
    }
    "FAIL" {
        puts "❌ Memory test failed"
        exit 1
    }
}

# Run system health check
send "c\r"
expect {
    timeout { 
        puts "❌ Health check timeout"
        exit 1 
    }
    "All systems: ✓ HEALTHY" {
        puts "✅ System health check passed"
    }
    "UNHEALTHY" {
        puts "❌ System health check failed"
        exit 1
    }
}

# Exit QEMU
send "\x01"
send "x"

expect eof
puts "✅ Memory tests completed successfully"
exit 0
EOF

# Make the expect script executable
chmod +x /tmp/memory_test_script.exp

# Check if expect is available
if ! command -v expect &> /dev/null; then
    echo "⚠️  expect not available, running simplified test..."
    
    # Fallback: run QEMU with timeout and check if it starts
    qemu-system-aarch64 -M raspi4b -nographic -kernel target/aarch64-unknown-none/release/tiny_os > /tmp/qemu_output.log 2>&1 &
    QEMU_PID=$!
    
    # Wait a bit for boot
    sleep 8
    
    # Kill QEMU
    kill $QEMU_PID 2>/dev/null || true
    wait $QEMU_PID 2>/dev/null || true
    
    # Check if TinyOS started properly
    if grep -q "TinyOS" /tmp/qemu_output.log && grep -q "Type 'h' for help" /tmp/qemu_output.log; then
        echo "✅ TinyOS boot test passed"
        rm -f /tmp/qemu_output.log
        exit 0
    else
        echo "❌ TinyOS boot test failed"
        echo "QEMU output:"
        cat /tmp/qemu_output.log
        rm -f /tmp/qemu_output.log
        exit 1
    fi
else
    # Run the expect script
    if /tmp/memory_test_script.exp; then
        echo "✅ All memory tests passed!"
        rm -f /tmp/memory_test_script.exp
        exit 0
    else
        echo "❌ Memory tests failed!"
        rm -f /tmp/memory_test_script.exp
        exit 1
    fi
fi
