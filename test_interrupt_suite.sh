#!/bin/bash

# TinyOS Interrupt Management Test Suite
# Comprehensive testing for all interrupt-related functionality

set -e

cd "$(dirname "$0")"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

print_header() {
    echo -e "${BLUE}═══════════════════════════════════════════════════════════════${NC}"
    echo -e "${BLUE}    TinyOS Interrupt Management Test Suite${NC}"
    echo -e "${BLUE}═══════════════════════════════════════════════════════════════${NC}"
}

print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Command line options
INTERACTIVE_MODE=true
AUTOMATED_MODE=false
QUICK_TEST=false

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --automated|-a)
            INTERACTIVE_MODE=false
            AUTOMATED_MODE=true
            shift
            ;;
        --quick|-q)
            QUICK_TEST=true
            shift
            ;;
        --help|-h)
            echo "TinyOS Interrupt Management Test Suite"
            echo ""
            echo "Usage: $0 [OPTIONS]"
            echo ""
            echo "Options:"
            echo "  --automated, -a   Run automated tests (non-interactive)"
            echo "  --quick, -q       Run quick tests only"
            echo "  --help, -h        Show this help message"
            echo ""
            echo "Interactive Mode Commands:"
            echo "  i - Show interrupt status and statistics"
            echo "  e - Enable all major interrupt sources"
            echo "  j - Run comprehensive interrupt test"
            echo "  d - Hardware diagnostics (includes interrupt info)"
            echo "  s - System information (includes interrupt stats)"
            exit 0
            ;;
        *)
            print_warning "Unknown option: $1"
            shift
            ;;
    esac
done

print_header

# Build the kernel
print_status "Building TinyOS kernel..."
if cargo build --target aarch64-unknown-none --release; then
    print_success "Kernel build completed successfully"
else
    print_error "Kernel build failed"
    exit 1
fi

if [ "$AUTOMATED_MODE" = true ]; then
    print_status "Running automated interrupt tests..."
    
    # Check if expect is available
    if ! command -v expect &> /dev/null; then
        print_error "expect command not found. Install with: brew install expect (macOS) or apt-get install expect (Ubuntu)"
        exit 1
    fi
    
    # Create automated test script
    cat > /tmp/interrupt_test_automated.exp << 'EOF'
#!/usr/bin/expect -f
set timeout 60

# Start QEMU
spawn qemu-system-aarch64 -M raspi4b -nographic -kernel target/aarch64-unknown-none/release/tiny_os

# Wait for boot
expect {
    timeout { 
        puts "❌ Boot timeout"
        exit 1 
    }
    "Type 'h' for help" {
        puts "✅ System booted successfully"
    }
}

# Test 1: Show initial interrupt status
puts "⚡ Test 1: Initial interrupt status"
send "i\r"
expect "Interrupt Status"
expect "==="
puts "✅ Interrupt status displayed"

# Test 2: Enable interrupt sources
puts "⚡ Test 2: Enable interrupt sources"
send "e\r"
expect {
    "Timer interrupts: ✓ ENABLED" {
        puts "✅ Timer interrupts enabled"
    }
    "Timer interrupts: ✗ FAILED" {
        puts "❌ Timer interrupt enable failed"
        exit 1
    }
}
expect {
    "UART interrupts: ✓ ENABLED" {
        puts "✅ UART interrupts enabled"
    }
    "UART interrupts: ✗ FAILED" {
        puts "❌ UART interrupt enable failed"
        exit 1
    }
}
expect {
    "GPIO interrupts: ✓ ENABLED" {
        puts "✅ GPIO interrupts enabled"
    }
    "GPIO interrupts: ✗ FAILED" {
        puts "❌ GPIO interrupt enable failed"
        exit 1
    }
}

# Test 3: Run comprehensive interrupt test
puts "⚡ Test 3: Comprehensive interrupt test"
send "j\r"
expect {
    timeout {
        puts "❌ Interrupt test timeout"
        exit 1
    }
    "All interrupt sources functioning correctly" {
        puts "✅ Comprehensive interrupt test passed"
    }
    "FAIL" {
        puts "❌ Interrupt test failed"
        exit 1
    }
}

# Test 4: Check interrupt statistics after testing
puts "⚡ Test 4: Interrupt statistics after testing"
send "i\r"
expect "Interrupt Status"
puts "✅ Interrupt statistics updated"

# Test 5: Hardware diagnostics
puts "⚡ Test 5: Hardware diagnostics"
send "d\r"
expect "Hardware Diagnostics"
expect "Interrupts:"
puts "✅ Hardware diagnostics with interrupt info displayed"

# Test 6: System information
puts "⚡ Test 6: System information"
send "s\r"
expect "System Information"
expect "Active Interrupts:"
puts "✅ System information with interrupt stats displayed"

# Exit QEMU
send "\x01"
send "x"
expect eof

puts "✅ All automated interrupt tests completed successfully!"
EOF

    chmod +x /tmp/interrupt_test_automated.exp
    
    if /tmp/interrupt_test_automated.exp; then
        print_success "All automated interrupt tests passed!"
    else
        print_error "Automated interrupt tests failed"
        exit 1
    fi
    
    # Cleanup
    rm -f /tmp/interrupt_test_automated.exp
    
else
    # Interactive mode
    print_status "Starting interactive interrupt testing session..."
    echo ""
    echo "Interrupt Management Test Commands:"
    echo "  i - Show interrupt status and detailed statistics"
    echo "  e - Enable all major interrupt sources (Timer, UART, GPIO)"
    echo "  j - Run comprehensive interrupt functionality test"
    echo "  d - Hardware diagnostics (includes interrupt controller info)"
    echo "  s - System information (includes interrupt statistics)"
    echo "  c - System health check (includes interrupt testing)"
    echo "  h - Show help menu with all available commands"
    echo ""
    echo "Interrupt System Features:"
    echo "  - ARM Generic Interrupt Controller (GIC) simulation"
    echo "  - Timer interrupts (IRQ 64) for system timing"
    echo "  - UART interrupts (IRQ 153) for serial communication"
    echo "  - GPIO interrupts (IRQ 129) for hardware events"
    echo "  - Interrupt enable/disable management"
    echo "  - Real-time interrupt statistics and monitoring"
    echo "  - Comprehensive interrupt testing framework"
    echo ""
    
    if [ "$QUICK_TEST" = true ]; then
        echo "Quick Test Sequence: i -> e -> j -> i -> d"
        echo "Press Enter to continue with quick test, or Ctrl+C to start interactive session..."
        read -r
        echo -e 'i\ne\nj\ni\nd\n' | qemu-system-aarch64 -M raspi4b -nographic -kernel target/aarch64-unknown-none/release/tiny_os
    else
        echo "Starting QEMU in interactive mode..."
        echo "Press Ctrl+A then X to exit QEMU"
        echo "═══════════════════════════════════════════════════════════════"
        
        qemu-system-aarch64 -M raspi4b -nographic -kernel target/aarch64-unknown-none/release/tiny_os
    fi
fi

print_success "Interrupt management testing session complete!"
