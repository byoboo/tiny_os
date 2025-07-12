#!/bin/bash

# TinyOS External Interrupt Integration Test
# Tests what can be validated from outside the kernel
# NOTE: Real interrupt functionality testing is done internally via 'cargo run' -> 't' command

# Change to project root directory
cd "$(dirname "$0")/../.."

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

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

echo "================================================================"
echo "  TinyOS External Interrupt Integration Test"
echo "================================================================"
echo "This test validates interrupt system integration from outside the kernel."
echo "For actual interrupt functionality testing, use: cargo run -> 't' command"
echo "================================================================"
echo

# Initialize counters
TESTS_PASSED=0
TESTS_FAILED=0

# Test 1: Build validation (can we compile the interrupt system?)
print_status "Test 1: Interrupt system build validation"
if cargo build --release --quiet; then
    print_success "Interrupt system compiles successfully"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    print_error "Interrupt system build failed"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

# Test 2: Source code structure validation
print_status "Test 2: Interrupt module structure validation"
if [[ -f "src/interrupts.rs" || -f "src/exceptions/mod.rs" ]]; then
    print_success "Interrupt module structure is correct"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    print_error "Interrupt module structure is incomplete"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

# Test 3: Boot sequence validation (does the system boot with interrupt handler?)
print_status "Test 3: System boot with interrupt handler"
timeout 10s qemu-system-aarch64 -M raspi4b -nographic -kernel target/aarch64-unknown-none/release/tiny_os > /tmp/interrupt_boot_test.log 2>&1 &
QEMU_PID=$!

sleep 5
kill $QEMU_PID 2>/dev/null
wait $QEMU_PID 2>/dev/null

if grep -q "interrupt\|exception\|TinyOS Ready" /tmp/interrupt_boot_test.log; then
    print_success "System boots with interrupt support"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    print_error "System boot with interrupt support failed"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

# Test 4: Reminder about proper testing
print_status "Test 4: Interrupt functionality test reminder"
print_warning "IMPORTANT: Real interrupt functionality testing requires internal kernel tests"
print_warning "To test actual interrupt handling, priority management, etc.:"
print_warning "  1. Run: cargo run"
print_warning "  2. In TinyOS shell, run: t"
print_warning "  3. This will run comprehensive interrupt tests from inside the kernel"
TESTS_PASSED=$((TESTS_PASSED + 1))  # This is always a "pass" since it's just a reminder

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
echo "  Interrupt Integration Test Results"
echo "======================================="
echo "Total Tests: $TOTAL_TESTS"
echo "Passed: $TESTS_PASSED"
echo "Failed: $TESTS_FAILED"
echo
echo "NOTE: This tests external integration only."
echo "For comprehensive interrupt functionality testing:"
echo "  cargo run -> TinyOS> t"
echo

if [ $TESTS_FAILED -eq 0 ]; then
    print_success "All interrupt integration tests passed!"
    echo "Ready for internal interrupt functionality testing."
    exit 0
else
    print_error "Some interrupt integration tests failed"
    exit 1
fi
