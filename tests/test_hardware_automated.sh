#!/bin/bash

# TinyOS Automated Hardware Test (No expect required)
# Simple automated hardware testing without interactive dependencies

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

# Test 1: Boot and verify hardware initialization
print_status "Test 1: Hardware initialization"
timeout 10s qemu-system-aarch64 -M raspi4b -nographic -kernel target/aarch64-unknown-none/release/tiny_os > /tmp/hardware_boot_test.log 2>&1 &
QEMU_PID=$!

sleep 5
kill $QEMU_PID 2>/dev/null
wait $QEMU_PID 2>/dev/null

if grep -q "GPIO" /tmp/hardware_boot_test.log; then
    print_success "GPIO system initialization verified"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    print_error "GPIO system initialization failed"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

# Test 2: UART driver validation
print_status "Test 2: UART driver validation"
if grep -q "struct Uart" src/uart.rs && grep -q "fn puts" src/uart.rs; then
    print_success "UART driver structure validated"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    print_error "UART driver validation failed"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

# Test 3: GPIO driver validation
print_status "Test 3: GPIO driver validation"
if grep -q "struct Gpio" src/gpio.rs && grep -q "set_high\|set_low" src/gpio.rs; then
    print_success "GPIO driver structure validated"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    print_error "GPIO driver validation failed"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

# Test 4: Timer driver validation
print_status "Test 4: Timer driver validation"
if grep -q "struct Timer\|SystemTimer" src/timer.rs; then
    print_success "Timer driver structure validated"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    print_error "Timer driver validation failed"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

# Test 5: Hardware abstraction completeness
print_status "Test 5: Hardware abstraction completeness"
DRIVER_COUNT=0
[ -f "src/uart.rs" ] && DRIVER_COUNT=$((DRIVER_COUNT + 1))
[ -f "src/gpio.rs" ] && DRIVER_COUNT=$((DRIVER_COUNT + 1))
[ -f "src/timer.rs" ] && DRIVER_COUNT=$((DRIVER_COUNT + 1))

if [ $DRIVER_COUNT -ge 3 ]; then
    print_success "Core hardware drivers present ($DRIVER_COUNT/3)"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    print_error "Missing core hardware drivers ($DRIVER_COUNT/3)"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

# Cleanup
rm -f /tmp/hardware_boot_test.log

# Results
TOTAL_TESTS=$((TESTS_PASSED + TESTS_FAILED))
echo
echo "======================================="
echo "  Hardware Test Results"
echo "======================================="
echo "Total Tests: $TOTAL_TESTS"
echo "Passed: $TESTS_PASSED"
echo "Failed: $TESTS_FAILED"

if [ $TESTS_FAILED -eq 0 ]; then
    print_success "All hardware tests passed!"
    exit 0
else
    print_error "Some hardware tests failed"
    exit 1
fi
