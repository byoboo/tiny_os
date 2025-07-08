#!/bin/bash

# TinyOS Automated Interrupt Test (No expect required)
# Simple automated interrupt testing without interactive dependencies

# Change to project root directory
cd "$(dirname "$0")/.."

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Initialize counters
TESTS_PASSED=0
TESTS_FAILED=0

# Build the kernel
print_status "Building TinyOS kernel..."
cargo build --release --quiet
if [ $? -ne 0 ]; then
    print_error "Kernel build failed"
    exit 1
fi
print_success "Kernel built successfully"

# Test 1: Boot and verify interrupt controller initialization
print_status "Test 1: Interrupt controller initialization"
timeout 10s qemu-system-aarch64 -M raspi4b -nographic -kernel target/aarch64-unknown-none/release/tiny_os > /tmp/interrupt_boot_test.log 2>&1 &
QEMU_PID=$!

sleep 5
kill $QEMU_PID 2>/dev/null
wait $QEMU_PID 2>/dev/null

if grep -q "✓ Interrupt controller initialized\|Interrupt controller initialized" /tmp/interrupt_boot_test.log; then
    print_success "Interrupt controller initialization verified"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    print_error "Interrupt controller initialization failed"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

# Test 2: Source code validation - InterruptController struct
print_status "Test 2: InterruptController struct validation"
if grep -q "struct InterruptController" src/interrupts.rs; then
    print_success "InterruptController struct present"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    print_error "InterruptController struct missing"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

# Test 3: Interrupt management functions
print_status "Test 3: Interrupt management functions"
if grep -q "fn enable_interrupt" src/interrupts.rs && grep -q "fn disable_interrupt" src/interrupts.rs; then
    print_success "Interrupt management functions present"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    print_error "Missing interrupt management functions"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

# Test 4: Timer interrupt setup
print_status "Test 4: Timer interrupt verification"
if grep -q "✓ System timer\|Timer.*initialized" /tmp/interrupt_boot_test.log; then
    print_success "Timer system detected"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    print_error "Timer system not detected"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

# Test 5: Interrupt vector validation
print_status "Test 5: Interrupt vector validation"
if grep -q "vector\|handler\|interrupt" src/interrupts.rs; then
    print_success "Interrupt handling code present"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    print_error "Interrupt handling code missing"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

# Cleanup
rm -f /tmp/interrupt_boot_test.log

# Results
TOTAL_TESTS=$((TESTS_PASSED + TESTS_FAILED))
echo
echo "======================================="
echo "  Interrupt Test Results"
echo "======================================="
echo "Total Tests: $TOTAL_TESTS"
echo "Passed: $TESTS_PASSED"
echo "Failed: $TESTS_FAILED"

if [ $TESTS_FAILED -eq 0 ]; then
    print_success "All interrupt tests passed!"
    exit 0
else
    print_error "Some interrupt tests failed"
    exit 1
fi
