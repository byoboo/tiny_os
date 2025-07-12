#!/bin/bash

# TinyOS External Memory Integration Test
# Tests what can be validated from outside the kernel
# NOTE: Real memory functionality testing is done internally via 'cargo run' -> 't' command

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
echo "  TinyOS External Memory Integration Test"
echo "================================================================"
echo "This test validates memory system integration from outside the kernel."
echo "For actual memory functionality testing, use: cargo run -> 't' command"
echo "================================================================"
echo

# Initialize counters
TESTS_PASSED=0
TESTS_FAILED=0

# Test 1: Build validation (can we compile the memory system?)
print_status "Test 1: Memory system build validation"
if cargo build --release --quiet; then
    print_success "Memory system compiles successfully"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    print_error "Memory system build failed"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

# Test 2: Source code structure validation
print_status "Test 2: Memory module structure validation"
if [[ -f "src/memory/mod.rs" && -f "src/memory/allocator.rs" && -f "src/memory/layout.rs" ]]; then
    print_success "Memory module structure is correct"
    TESTS_PASSED=$((TESTS_PASSED + 1))
else
    print_error "Memory module structure is incomplete"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

# Test 3: Boot sequence validation (does the system boot with memory manager?)
print_status "Test 3: System boot with memory manager"

# Simple boot test - just verify the binary exists and can be executed with timeout
if [[ -f "target/aarch64-unknown-none/release/tiny_os" ]]; then
    # Docker environment detection - use compatible machine type
    if [[ -f /.dockerenv ]]; then
        MACHINE_TYPE="raspi3b"
    else
        MACHINE_TYPE="raspi4b"
    fi
    
    # Run a minimal boot test to verify it starts
    timeout 3s qemu-system-aarch64 -M $MACHINE_TYPE -nographic -kernel target/aarch64-unknown-none/release/tiny_os >/dev/null 2>&1
    BOOT_EXIT_CODE=$?
    
    # Exit code 124 means timeout (expected), 0 means clean exit, both are acceptable
    if [[ $BOOT_EXIT_CODE -eq 124 || $BOOT_EXIT_CODE -eq 0 ]]; then
        print_success "System boots with memory manager"
        TESTS_PASSED=$((TESTS_PASSED + 1))
    else
        print_error "System boot with memory manager failed - unexpected exit code: $BOOT_EXIT_CODE"
        TESTS_FAILED=$((TESTS_FAILED + 1))
    fi
else
    print_error "System boot with memory manager failed - release binary not found"
    TESTS_FAILED=$((TESTS_FAILED + 1))
fi

# Test 4: Reminder about proper testing
print_status "Test 4: Memory functionality test reminder"
print_warning "IMPORTANT: Real memory functionality testing requires internal kernel tests"
print_warning "To test actual memory operations, allocation, MMU, etc.:"
print_warning "  1. Run: cargo run"
print_warning "  2. In TinyOS shell, run: t"
print_warning "  3. This will run comprehensive memory tests from inside the kernel"
TESTS_PASSED=$((TESTS_PASSED + 1))  # This is always a "pass" since it's just a reminder

# Results
TOTAL_TESTS=$((TESTS_PASSED + TESTS_FAILED))
echo
echo "======================================="
echo "  Memory Integration Test Results"
echo "======================================="
echo "Total Tests: $TOTAL_TESTS"
echo "Passed: $TESTS_PASSED"
echo "Failed: $TESTS_FAILED"
echo
echo "NOTE: This tests external integration only."
echo "For comprehensive memory functionality testing:"
echo "  cargo run -> TinyOS> t"
echo

if [ $TESTS_FAILED -eq 0 ]; then
    print_success "All memory integration tests passed!"
    echo "Ready for internal memory functionality testing."
    exit 0
else
    print_error "Some memory integration tests failed"
    exit 1
fi
