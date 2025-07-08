#!/bin/bash

# TinyOS Automated Memory Test (No expect required)
# Simple automated memory testing without interactive dependencies

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

# Test 1: Boot and verify memory manager initialization
print_status "Test 1: Memory manager initialization"
timeout 10s qemu-system-aarch64 -M raspi4b -nographic -kernel target/aarch64-unknown-none/release/tiny_os > /tmp/memory_boot_test.log 2>&1 &
QEMU_PID=$!

sleep 5
kill $QEMU_PID 2>/dev/null
wait $QEMU_PID 2>/dev/null

if grep -q "✓ Memory manager initialized\|Memory manager initialized" /tmp/memory_boot_test.log; then
    print_success "Memory manager initialization verified"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    print_error "Memory manager initialization failed"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

# Test 2: Verify memory system is operational (check for TinyOS Ready)
print_status "Test 2: Memory system operational verification"
if grep -q "✓ TinyOS Ready" /tmp/memory_boot_test.log; then
    print_success "Memory system operational (system ready)"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    print_error "Memory system may not be operational"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

# Test 3: Check for complete initialization sequence
print_status "Test 3: Complete system initialization verification"
if grep -q "Initializing system components" /tmp/memory_boot_test.log && grep -q "Available commands" /tmp/memory_boot_test.log; then
    print_success "Complete initialization sequence detected"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    print_error "Incomplete initialization sequence"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

# Test 4: Source code validation
print_status "Test 4: Memory manager source validation"
if grep -q "fn allocate_block\|fn allocate_blocks" src/memory.rs && grep -q "fn deallocate\|fn free" src/memory.rs; then
    print_success "Allocation/deallocation functions present"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    print_error "Missing allocation/deallocation functions"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

# Test 5: Memory constants validation
print_status "Test 5: Memory constants validation"
if grep -q "HEAP_SIZE" src/memory.rs && grep -q "BLOCK_SIZE" src/memory.rs; then
    print_success "Memory constants defined"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    print_error "Missing memory constants"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

# Cleanup
rm -f /tmp/memory_boot_test.log

# Results
TOTAL_TESTS=$((TESTS_PASSED + TESTS_FAILED))
echo
echo "======================================="
echo "  Memory Test Results"
echo "======================================="
echo "Total Tests: $TOTAL_TESTS"
echo "Passed: $TESTS_PASSED"
echo "Failed: $TESTS_FAILED"

if [ $TESTS_FAILED -eq 0 ]; then
    print_success "All memory tests passed!"
    exit 0
else
    print_error "Some memory tests failed"
    exit 1
fi
