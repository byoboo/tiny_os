#!/bin/bash

# TinyOS Hardware Test Suite
# Comprehensive testing for hardware drivers and peripherals

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
    echo -e "${BLUE}    TinyOS Hardware Test Suite${NC}"
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
            echo "TinyOS Hardware Test Suite"
            echo ""
            echo "Usage: $0 [OPTIONS]"
            echo ""
            echo "Options:"
            echo "  --automated, -a   Run automated tests (non-interactive)"
            echo "  --quick, -q       Run quick tests only"
            echo "  --help, -h        Show this help message"
            echo ""
            echo "Interactive Mode Commands:"
            echo "  1/0 - Turn LED ON/OFF"
            echo "  l   - Toggle LED state"
            echo "  t   - Show current system time"
            echo "  d   - Hardware diagnostics"
            echo "  s   - System information"
            echo "  c   - System health check (includes hardware tests)"
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
    print_status "Running automated hardware tests..."
    
    # Check if expect is available
    if ! command -v expect &> /dev/null; then
        print_error "expect command not found. Install with: brew install expect (macOS) or apt-get install expect (Ubuntu)"
        exit 1
    fi
    
    # Create automated test script
    cat > /tmp/hardware_test_automated.exp << 'EOF'
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

# Test 1: System information
puts "🔧 Test 1: System information"
send "s\r"
expect "System Information"
expect "Timer Frequency: 1MHz"
expect "UART Base: 0xFE201000"
expect "GPIO Base: 0xFE200000"
puts "✅ System information displayed correctly"

# Test 2: Hardware diagnostics
puts "🔧 Test 2: Hardware diagnostics"
send "d\r"
expect "Hardware Diagnostics"
expect "CPU: ARM Cortex"
expect "Timer: BCM2835 System Timer"
expect "UART: PL011 UART"
expect "GPIO: BCM2835 GPIO Controller"
puts "✅ Hardware diagnostics completed"

# Test 3: Timer functionality
puts "🔧 Test 3: Timer functionality"
send "t\r"
expect "Current system time:"
puts "✅ Timer functionality verified"

# Test 4: LED control - Turn ON
puts "🔧 Test 4: LED control - Turn ON"
send "1\r"
expect "LED turned ON"
puts "✅ LED turned ON successfully"

# Wait a moment
sleep 1

# Test 5: LED control - Turn OFF
puts "🔧 Test 5: LED control - Turn OFF"
send "0\r"
expect "LED turned OFF"
puts "✅ LED turned OFF successfully"

# Test 6: LED toggle functionality
puts "🔧 Test 6: LED toggle functionality"
send "l\r"
expect "LED toggled"
puts "✅ LED toggle functionality verified"

# Test 7: Comprehensive system health check
puts "🔧 Test 7: System health check (includes hardware tests)"
send "c\r"
expect {
    timeout {
        puts "❌ Health check timeout"
        exit 1
    }
    "Overall Status: ✓ HEALTHY" {
        puts "✅ System health check passed"
    }
    "Timer System: ✓ PASS" {
        puts "✅ Timer system test passed"
        exp_continue
    }
    "UART System: ✓ PASS" {
        puts "✅ UART system test passed"
        exp_continue
    }
    "GPIO System: ✓ PASS" {
        puts "✅ GPIO system test passed"
        exp_continue
    }
    "LED Test: ✓ COMPLETE" {
        puts "✅ LED test completed"
        exp_continue
    }
}

# Test 8: Multiple LED blink sequence
puts "🔧 Test 8: LED blink sequence test"
for {set i 0} {$i < 3} {incr i} {
    send "1\r"
    expect "LED turned ON"
    sleep 0.5
    send "0\r"
    expect "LED turned OFF"
    sleep 0.5
}
puts "✅ LED blink sequence completed"

# Exit QEMU
send "\x01"
send "x"
expect eof

puts "✅ All automated hardware tests completed successfully!"
EOF

    chmod +x /tmp/hardware_test_automated.exp
    
    if /tmp/hardware_test_automated.exp; then
        print_success "All automated hardware tests passed!"
    else
        print_error "Automated hardware tests failed"
        exit 1
    fi
    
    # Cleanup
    rm -f /tmp/hardware_test_automated.exp
    
else
    # Interactive mode
    print_status "Starting interactive hardware testing session..."
    echo ""
    echo "Hardware Test Commands:"
    echo "  1 - Turn LED ON (GPIO pin 42)"
    echo "  0 - Turn LED OFF (GPIO pin 42)"
    echo "  l - Toggle LED state"
    echo "  t - Show current system time (Timer test)"
    echo "  d - Hardware diagnostics (CPU, Timer, UART, GPIO, GIC)"
    echo "  s - System information (Hardware specifications)"
    echo "  c - System health check (Comprehensive hardware tests)"
    echo "  h - Show help menu with all available commands"
    echo ""
    echo "Hardware Components Tested:"
    echo "  🔧 Timer: BCM2835 System Timer @ 1MHz"
    echo "  🔧 UART: PL011 UART for serial communication"
    echo "  🔧 GPIO: BCM2835 GPIO Controller (Pin 42 for LED)"
    echo "  🔧 GIC: ARM Generic Interrupt Controller"
    echo "  🔧 CPU: ARM Cortex-A72/A76 (Raspberry Pi 4/5)"
    echo ""
    echo "Test Features:"
    echo "  - Real-time LED control and status feedback"
    echo "  - Timer precision and functionality testing"
    echo "  - UART communication verification"
    echo "  - GPIO pin configuration and control"
    echo "  - Hardware register access verification"
    echo ""
    
    if [ "$QUICK_TEST" = true ]; then
        echo "Quick Test Sequence: s -> d -> t -> 1 -> 0 -> l -> c"
        echo "Press Enter to continue with quick test, or Ctrl+C to start interactive session..."
        read -r
        echo -e 's\nd\nt\n1\n0\nl\nc\n' | qemu-system-aarch64 -M raspi4b -nographic -kernel target/aarch64-unknown-none/release/tiny_os
    else
        echo "Starting QEMU in interactive mode..."
        echo "Press Ctrl+A then X to exit QEMU"
        echo "═══════════════════════════════════════════════════════════════"
        
        qemu-system-aarch64 -M raspi4b -nographic -kernel target/aarch64-unknown-none/release/tiny_os
    fi
fi

print_success "Hardware testing session complete!"
