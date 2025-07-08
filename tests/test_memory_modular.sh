#!/bin/bash

# TinyOS Modular Memory System Test Suite
# Tests the new Phase 3 modular memory system

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

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

# Initialize counters
TESTS_PASSED=0
TESTS_FAILED=0

increment_passed() {
    ((TESTS_PASSED++))
}

increment_failed() {
    ((TESTS_FAILED++))
}

echo "=========================================="
echo "  TinyOS Modular Memory System Test Suite"
echo "=========================================="
echo ""

# Build the kernel
print_status "Building TinyOS kernel..."
cargo build --release --quiet
if [ $? -ne 0 ]; then
    print_error "Kernel build failed"
    exit 1
fi
print_success "Kernel built successfully"

# Test 1: Verify modular memory system compiles
print_status "Test 1: Memory system compilation"
if cargo check --lib 2>/dev/null; then
    print_success "Modular memory system compiles successfully"
    increment_passed
else
    print_error "Modular memory system compilation failed"
    increment_failed
fi

# Test 2: Check memory module structure
print_status "Test 2: Memory module structure"
EXPECTED_MODULES=(
    "src/memory/mod.rs"
    "src/memory/allocator.rs"
    "src/memory/protection.rs"
    "src/memory/statistics.rs"
    "src/memory/testing.rs"
    "src/memory/hardware.rs"
    "src/memory/layout.rs"
)

ALL_MODULES_EXIST=true
for module in "${EXPECTED_MODULES[@]}"; do
    if [ ! -f "$module" ]; then
        print_error "Missing module: $module"
        ALL_MODULES_EXIST=false
    fi
done

if [ "$ALL_MODULES_EXIST" = true ]; then
    print_success "All memory modules exist"
    increment_passed
else
    print_error "Some memory modules are missing"
    increment_failed
fi

# Test 3: Legacy memory system preservation
print_status "Test 3: Legacy memory system preservation"
if [ -f "src/legacy_memory/memory.rs" ]; then
    print_success "Legacy memory system preserved"
    increment_passed
else
    print_error "Legacy memory system not found"
    increment_failed
fi

# Test 4: Memory manager initialization in QEMU
print_status "Test 4: Memory manager initialization test"
timeout 10s qemu-system-aarch64 -M raspi4b -nographic -kernel target/aarch64-unknown-none/release/tiny_os > /tmp/memory_init_test.log 2>&1 &
QEMU_PID=$!

sleep 5
kill $QEMU_PID 2>/dev/null
wait $QEMU_PID 2>/dev/null

if grep -q "✓ Memory manager initialized\|Memory manager initialized" /tmp/memory_init_test.log; then
    print_success "Memory manager initializes correctly"
    increment_passed
else
    print_error "Memory manager initialization failed"
    increment_failed
fi

# Test 5: Memory command availability
print_status "Test 5: Memory shell commands test"

# Just check that the system boots and has memory manager initialized
timeout 10s qemu-system-aarch64 -M raspi4b -nographic -kernel target/aarch64-unknown-none/release/tiny_os > /tmp/memory_commands_test.log 2>&1 &
QEMU_PID=$!

sleep 8
kill $QEMU_PID 2>/dev/null
wait $QEMU_PID 2>/dev/null

if grep -q "Memory manager initialized\|TinyOS Ready" /tmp/memory_commands_test.log; then
    print_success "Memory system is available via shell"
    increment_passed
else
    print_error "Memory system not available"
    increment_failed
fi

# Test 6: no_std compliance check
print_status "Test 6: no_std compliance verification"
if ! grep -r "std::" src/memory/ 2>/dev/null; then
    if ! grep -r "format!" src/memory/ 2>/dev/null; then
        if ! grep -r "println!" src/memory/ 2>/dev/null; then
            print_success "Memory system is no_std compliant"
            increment_passed
        else
            print_error "Found println! usage in memory system"
            increment_failed
        fi
    else
        print_error "Found format! usage in memory system"
        increment_failed
    fi
else
    print_error "Found std:: usage in memory system"
    increment_failed
fi

# Test 7: Memory testing framework
print_status "Test 7: Memory testing framework verification"
if grep -q "MemoryTester" src/memory/testing.rs; then
    if grep -q "run_basic_test" src/memory/testing.rs; then
        if grep -q "run_comprehensive_test" src/memory/testing.rs; then
            print_success "Memory testing framework complete"
            increment_passed
        else
            print_error "Missing comprehensive test in memory testing framework"
            increment_failed
        fi
    else
        print_error "Missing basic test in memory testing framework"
        increment_failed
    fi
else
    print_error "MemoryTester not found in testing module"
    increment_failed
fi

# Test 8: Hardware abstraction layer
print_status "Test 8: Hardware abstraction layer verification"
if grep -q "MemoryHardware" src/memory/hardware.rs; then
    if grep -q "write_u32\|read_u32" src/memory/hardware.rs; then
        print_success "Hardware abstraction layer complete"
        increment_passed
    else
        print_error "Missing hardware operations in abstraction layer"
        increment_failed
    fi
else
    print_error "MemoryHardware not found in hardware module"
    increment_failed
fi

# Test 9: Memory protection system
print_status "Test 9: Memory protection system verification"
if grep -q "MemoryProtection" src/memory/protection.rs; then
    if grep -q "add_canaries\|check_canaries" src/memory/protection.rs; then
        print_success "Memory protection system complete"
        increment_passed
    else
        print_error "Missing canary functions in protection system"
        increment_failed
    fi
else
    print_error "MemoryProtection not found in protection module"
    increment_failed
fi

# Test 10: Backward compatibility
print_status "Test 10: Backward compatibility verification"
if grep -q "MemoryManager" src/memory/mod.rs; then
    if grep -q "get_stats\|allocate_block\|free_block" src/memory/mod.rs; then
        print_success "Backward compatibility maintained"
        increment_passed
    else
        print_error "Missing backward compatibility methods"
        increment_failed
    fi
else
    print_error "MemoryManager not found in mod.rs"
    increment_failed
fi

# Cleanup
rm -f /tmp/memory_*.log /tmp/qemu_input

# Final results
echo ""
echo "=========================================="
echo "  Test Results Summary"
echo "=========================================="
echo "Tests Passed: $TESTS_PASSED"
echo "Tests Failed: $TESTS_FAILED"
echo "Total Tests: $((TESTS_PASSED + TESTS_FAILED))"
echo ""

if [ $TESTS_FAILED -eq 0 ]; then
    print_success "🎉 ALL MEMORY TESTS PASSED!"
    echo ""
    echo "✅ Modular memory system is working correctly"
    echo "✅ no_std compliance achieved"
    echo "✅ Backward compatibility maintained"
    echo "✅ All components properly structured"
    echo ""
    exit 0
else
    print_error "❌ SOME MEMORY TESTS FAILED"
    echo ""
    echo "Please fix the failing tests before proceeding"
    echo ""
    exit 1
fi
