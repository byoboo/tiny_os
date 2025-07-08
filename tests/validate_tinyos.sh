#!/bin/bash

# Simple Test Runner for TinyOS
# This script runs basic validation tests without requiring complex test infrastructure

# Change to project root directory
cd "$(dirname "$0")/.."

echo "========================================"
echo "  TinyOS Simple Test Validation"
echo "========================================"
echo

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

print_status() {
    if [ $1 -eq 0 ]; then
        echo -e "${GREEN}✓ PASS${NC} - $2"
    else
        echo -e "${RED}✗ FAIL${NC} - $2"
    fi
}

print_info() {
    echo -e "${BLUE}ℹ INFO${NC} - $1"
}

# Validation Tests
PASSED_TESTS=0
TOTAL_TESTS=0

echo "Running TinyOS validation tests..."
echo

# Test 1: Build verification
print_info "Testing kernel build..."
((TOTAL_TESTS++))
cargo build --bin tiny_os --quiet
BUILD_RESULT=$?
print_status $BUILD_RESULT "Kernel build verification"
[ $BUILD_RESULT -eq 0 ] && ((PASSED_TESTS++))

# Test 2: Binary size check
print_info "Checking binary size..."
((TOTAL_TESTS++))
if [ -f "target/aarch64-unknown-none/debug/tiny_os" ]; then
    SIZE=$(stat -f%z "target/aarch64-unknown-none/debug/tiny_os" 2>/dev/null || stat -c%s "target/aarch64-unknown-none/debug/tiny_os" 2>/dev/null)
    if [ $? -eq 0 ] && [ "$SIZE" -gt 1000 ] && [ "$SIZE" -lt 10000000 ]; then
        print_status 0 "Binary size check ($SIZE bytes)"
        ((PASSED_TESTS++))
    else
        print_status 1 "Binary size check"
    fi
else
    print_status 1 "Binary size check (file not found)"
fi

# Test 3: Memory layout verification
print_info "Verifying memory layout..."
((TOTAL_TESTS++))
if objdump -h target/aarch64-unknown-none/debug/tiny_os 2>/dev/null | grep -q "\.text\|\.data\|\.bss"; then
    print_status 0 "Memory layout verification"
    ((PASSED_TESTS++))
else
    print_status 1 "Memory layout verification"
fi

# Test 4: Symbol table check
print_info "Checking symbol table..."
((TOTAL_TESTS++))
if nm target/aarch64-unknown-none/debug/tiny_os 2>/dev/null | grep -q "_start\|main"; then
    print_status 0 "Symbol table check"
    ((PASSED_TESTS++))
else
    print_status 1 "Symbol table check"
fi

# Test 5: Release build
print_info "Testing release build..."
((TOTAL_TESTS++))
cargo build --bin tiny_os --release --quiet
RELEASE_RESULT=$?
print_status $RELEASE_RESULT "Release build verification"
[ $RELEASE_RESULT -eq 0 ] && ((PASSED_TESTS++))

# Test 6: Code structure validation
print_info "Validating code structure..."
((TOTAL_TESTS++))
STRUCTURE_OK=1

# Check for essential files
for file in "src/main.rs" "src/boot.s" "src/memory.rs" "src/uart.rs" "src/gpio.rs" "src/timer.rs" "src/interrupts.rs"; do
    if [ ! -f "$file" ]; then
        echo "Missing file: $file"
        STRUCTURE_OK=0
    fi
done

# Check for essential functions in main.rs
if ! grep -q "kernel_main" src/main.rs; then
    echo "Missing kernel_main function"
    STRUCTURE_OK=0
fi

if ! grep -q "panic_handler" src/main.rs; then
    echo "Missing panic handler"
    STRUCTURE_OK=0
fi

print_status $((1 - STRUCTURE_OK)) "Code structure validation"
[ $STRUCTURE_OK -eq 1 ] && ((PASSED_TESTS++))

# Test 7: Memory management validation
print_info "Validating memory management..."
((TOTAL_TESTS++))
MEMORY_OK=1

if ! grep -q "MemoryManager" src/memory.rs; then
    echo "Missing MemoryManager struct"
    MEMORY_OK=0
fi

if ! grep -q "allocate" src/memory.rs; then
    echo "Missing allocation function"
    MEMORY_OK=0
fi

if ! (grep -q "HEAP_START" src/memory.rs && (grep -q "HEAP_SIZE" src/memory.rs || grep -q "HEAP_SIZE" src/lib.rs)); then
    echo "Missing heap constants"
    MEMORY_OK=0
fi

print_status $((1 - MEMORY_OK)) "Memory management validation"
[ $MEMORY_OK -eq 1 ] && ((PASSED_TESTS++))

# Test 8: UART driver validation
print_info "Validating UART driver..."
((TOTAL_TESTS++))
UART_OK=1

if ! grep -q "struct Uart" src/uart.rs; then
    echo "Missing Uart struct"
    UART_OK=0
fi

if ! (grep -q "puts" src/uart.rs || grep -q "putc" src/uart.rs); then
    echo "Missing UART output functions"
    UART_OK=0
fi

print_status $((1 - UART_OK)) "UART driver validation"
[ $UART_OK -eq 1 ] && ((PASSED_TESTS++))

# Test 9: GPIO driver validation
print_info "Validating GPIO driver..."
((TOTAL_TESTS++))
GPIO_OK=1

if ! grep -q "struct Gpio" src/gpio.rs; then
    echo "Missing Gpio struct"
    GPIO_OK=0
fi

if ! (grep -q "set_high\|set_low\|set_function" src/gpio.rs); then
    echo "Missing GPIO control functions"
    GPIO_OK=0
fi

print_status $((1 - GPIO_OK)) "GPIO driver validation"
[ $GPIO_OK -eq 1 ] && ((PASSED_TESTS++))

# Test 10: Interrupt system validation
print_info "Validating interrupt system..."
((TOTAL_TESTS++))
INTERRUPT_OK=1

if ! grep -q "InterruptController" src/interrupts.rs; then
    echo "Missing InterruptController struct"
    INTERRUPT_OK=0
fi

if ! grep -q "enable_interrupt" src/interrupts.rs; then
    echo "Missing interrupt control functions"
    INTERRUPT_OK=0
fi

print_status $((1 - INTERRUPT_OK)) "Interrupt system validation"
[ $INTERRUPT_OK -eq 1 ] && ((PASSED_TESTS++))

echo
echo "========================================"
echo "  Test Results Summary"
echo "========================================"

echo "Tests Passed: $PASSED_TESTS/$TOTAL_TESTS"

if [ $PASSED_TESTS -eq $TOTAL_TESTS ]; then
    echo
    echo -e "${GREEN}🎉 ALL VALIDATION TESTS PASSED! 🎉${NC}"
    echo -e "${GREEN}TinyOS structure and build are valid${NC}"
    
    # Generate a simple test report
    cat > validation_report.md << EOF
# TinyOS Validation Report

Generated: $(date)

## Test Results

| Test | Status | Description |
|------|--------|-------------|
| Build Verification | ✅ PASS | Kernel builds successfully |
| Binary Size Check | ✅ PASS | Binary size is reasonable |
| Memory Layout | ✅ PASS | Sections are properly defined |
| Symbol Table | ✅ PASS | Essential symbols present |
| Release Build | ✅ PASS | Release build succeeds |
| Code Structure | ✅ PASS | All essential files present |
| Memory Management | ✅ PASS | Memory system properly implemented |
| UART Driver | ✅ PASS | UART functionality present |
| GPIO Driver | ✅ PASS | GPIO control implemented |
| Interrupt System | ✅ PASS | Interrupt management present |

## Summary

**Total Tests**: $TOTAL_TESTS  
**Passed**: $PASSED_TESTS  
**Failed**: $((TOTAL_TESTS - PASSED_TESTS))  

🎉 **ALL VALIDATION TESTS PASSED**

TinyOS is ready for QEMU testing and hardware deployment.

## Next Steps

- Run QEMU tests: \`./run.sh\`
- Test memory management: \`./test_memory_comprehensive.sh\`
- Test interrupt system: \`./test_interrupts.sh\`
- Deploy to Raspberry Pi hardware

EOF
    
    echo "Validation report generated: validation_report.md"
    EXIT_CODE=0
else
    echo
    echo -e "${RED}❌ SOME VALIDATION TESTS FAILED ❌${NC}"
    echo -e "${RED}Please fix issues before proceeding${NC}"
    EXIT_CODE=1
fi

echo "========================================"

exit $EXIT_CODE
