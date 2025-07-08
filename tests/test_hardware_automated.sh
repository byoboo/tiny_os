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

# Test 2: UART driver validation (modular architecture)
print_status "Test 2: UART driver validation (modular)"
if [ -d "src/drivers/uart" ] && [ -f "src/drivers/uart/mod.rs" ] && grep -q "pub mod uart" src/lib.rs; then
    print_success "UART modular driver structure validated"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    print_error "UART modular driver validation failed"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

# Test 3: GPIO driver validation (modular architecture)
print_status "Test 3: GPIO driver validation (modular)"
if [ -d "src/drivers/gpio" ] && [ -f "src/drivers/gpio/mod.rs" ] && grep -q "pub mod gpio" src/lib.rs; then
    print_success "GPIO modular driver structure validated"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    print_error "GPIO modular driver validation failed"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

# Test 4: Timer driver validation (modular architecture)
print_status "Test 4: Timer driver validation (modular)"
if [ -d "src/drivers/timer" ] && [ -f "src/drivers/timer/mod.rs" ] && grep -q "pub mod timer" src/lib.rs; then
    print_success "Timer modular driver structure validated"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    print_error "Timer modular driver validation failed"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

# Test 5: Hardware abstraction completeness (modular architecture)
print_status "Test 5: Modular hardware abstraction completeness"
DRIVER_COUNT=0
[ -d "src/drivers/uart" ] && DRIVER_COUNT=$((DRIVER_COUNT + 1))
[ -d "src/drivers/gpio" ] && DRIVER_COUNT=$((DRIVER_COUNT + 1))
[ -d "src/drivers/timer" ] && DRIVER_COUNT=$((DRIVER_COUNT + 1))
[ -d "src/drivers/sdcard" ] && DRIVER_COUNT=$((DRIVER_COUNT + 1))

if [ $DRIVER_COUNT -ge 4 ]; then
    print_success "Core modular hardware drivers present ($DRIVER_COUNT/4)"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    print_error "Missing modular hardware drivers ($DRIVER_COUNT/4)"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

# Test 6: Legacy driver compatibility
print_status "Test 6: Legacy driver compatibility"
LEGACY_COUNT=0
[ -f "src/legacy_drivers/uart.rs" ] && LEGACY_COUNT=$((LEGACY_COUNT + 1))
[ -f "src/legacy_drivers/gpio.rs" ] && LEGACY_COUNT=$((LEGACY_COUNT + 1))
[ -f "src/legacy_drivers/timer.rs" ] && LEGACY_COUNT=$((LEGACY_COUNT + 1))
[ -f "src/legacy_drivers/sdcard.rs" ] && LEGACY_COUNT=$((LEGACY_COUNT + 1))

if [ $LEGACY_COUNT -ge 4 ]; then
    print_success "Legacy drivers archived correctly ($LEGACY_COUNT/4)"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    print_error "Legacy driver archival incomplete ($LEGACY_COUNT/4)"
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
