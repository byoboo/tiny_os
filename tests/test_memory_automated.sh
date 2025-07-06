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

if grep -q "Memory Manager initialized" /tmp/memory_boot_test.log; then
    print_success "Memory manager initialization verified"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    print_error "Memory manager initialization failed"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

# Test 2: Verify heap size is reported
print_status "Test 2: Heap size verification"
if grep -q "Heap Size:" /tmp/memory_boot_test.log; then
    HEAP_SIZE=$(grep "Heap Size:" /tmp/memory_boot_test.log | head -1)
    print_success "Heap size reported: $HEAP_SIZE"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    print_error "Heap size not reported"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

# Test 3: Check for block management
print_status "Test 3: Block management verification"
if grep -q "blocks" /tmp/memory_boot_test.log; then
    print_success "Block management system detected"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    print_error "Block management system not detected"
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
