#!/bin/bash

# TinyOS Memory Management Test Suite
# Comprehensive testing for all memory-related functionality

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
    echo -e "${BLUE}    TinyOS Memory Management Test Suite${NC}"
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
            echo "TinyOS Memory Management Test Suite"
            echo ""
            echo "Usage: $0 [OPTIONS]"
            echo ""
            echo "Options:"
            echo "  --automated, -a   Run automated tests (non-interactive)"
            echo "  --quick, -q       Run quick tests only"
            echo "  --help, -h        Show this help message"
            echo ""
            echo "Interactive Mode Commands:"
            echo "  m - Show memory statistics"
            echo "  a - Allocate memory block"
            echo "  f - Free last allocated block"
            echo "  x - Run basic memory test"
            echo "  z - Run comprehensive memory test suite"
            echo "  g - Memory corruption check"
            echo "  r - Defragment memory"
            echo "  c - System health check (includes memory tests)"
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
    print_status "Running automated memory tests..."
    
    # Check if expect is available
    if ! command -v expect &> /dev/null; then
        print_error "expect command not found. Install with: brew install expect (macOS) or apt-get install expect (Ubuntu)"
        exit 1
    fi
    
    # Create automated test script
    cat > /tmp/memory_test_automated.exp << 'EOF'
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

# Test 1: Show initial memory statistics
puts "🧠 Test 1: Initial memory statistics"
send "m\r"
expect "Memory Statistics"
expect "==="
puts "✅ Memory statistics displayed"

# Test 2: Basic memory allocation test
puts "🧠 Test 2: Basic memory allocation"
send "x\r"
expect {
    "Memory test: ✓ PASSED" {
        puts "✅ Basic memory test passed"
    }
    "Memory test: ✗ FAILED" {
        puts "❌ Basic memory test failed"
        exit 1
    }
}

# Test 3: Comprehensive memory test suite
puts "🧠 Test 3: Comprehensive memory test suite"
send "z\r"
expect {
    timeout {
        puts "❌ Comprehensive test timeout"
        exit 1
    }
    "✓ ALL TESTS PASSED" {
        puts "✅ All comprehensive memory tests passed"
    }
    "⚠️  SOME TESTS FAILED" {
        puts "❌ Some comprehensive tests failed"
        exit 1
    }
}

# Test 4: Memory corruption check
puts "🧠 Test 4: Memory corruption check"
send "g\r"
expect {
    "✓ PASSED" {
        puts "✅ Memory corruption check passed"
    }
    "⚠️  WARNING" {
        puts "⚠️  Memory corruption warning detected"
    }
}

# Test 5: Memory defragmentation
puts "🧠 Test 5: Memory defragmentation"
send "r\r"
expect "Defragmentation complete"
puts "✅ Memory defragmentation completed"

# Test 6: Allocate and free specific blocks
puts "🧠 Test 6: Manual allocation/deallocation"
send "a\r"
expect "Allocated block"
send "a\r"
expect "Allocated block"
send "f\r"
expect "Freed block"
puts "✅ Manual allocation/deallocation test passed"

# Exit QEMU
send "\x01"
send "x"
expect eof

puts "✅ All automated memory tests completed successfully!"
EOF

    chmod +x /tmp/memory_test_automated.exp
    
    if /tmp/memory_test_automated.exp; then
        print_success "All automated memory tests passed!"
    else
        print_error "Automated memory tests failed"
        exit 1
    fi
    
    # Cleanup
    rm -f /tmp/memory_test_automated.exp
    
else
    # Interactive mode
    print_status "Starting interactive memory testing session..."
    echo ""
    echo "Memory Management Test Commands:"
    echo "  m - Show memory statistics and layout"
    echo "  a - Allocate a memory block"
    echo "  f - Free the last allocated block"
    echo "  x - Run basic memory allocation/deallocation test"
    echo "  z - Run comprehensive memory test suite (5 tests)"
    echo "  g - Run memory corruption detection check"
    echo "  r - Defragment memory and coalesce free blocks"
    echo "  c - Run full system health check (includes all memory tests)"
    echo "  h - Show help menu with all available commands"
    echo ""
    echo "Advanced Memory Features:"
    echo "  - Block-based allocation with bitmap tracking"
    echo "  - Memory corruption detection"
    echo "  - Fragmentation analysis and defragmentation"
    echo "  - Stress testing with up to 50 concurrent blocks"
    echo "  - Boundary and alignment testing"
    echo "  - Real-time memory usage statistics"
    echo ""
    
    if [ "$QUICK_TEST" = true ]; then
        echo "Quick Test Sequence: m -> x -> z -> g -> c"
        echo "Press Enter to continue with quick test, or Ctrl+C to start interactive session..."
        read -r
        echo -e 'm\nx\nz\ng\nc\n' | qemu-system-aarch64 -M raspi4b -nographic -kernel target/aarch64-unknown-none/release/tiny_os
    else
        echo "Starting QEMU in interactive mode..."
        echo "Press Ctrl+A then X to exit QEMU"
        echo "═══════════════════════════════════════════════════════════════"
        
        qemu-system-aarch64 -M raspi4b -nographic -kernel target/aarch64-unknown-none/release/tiny_os
    fi
fi

print_success "Memory management testing session complete!"
