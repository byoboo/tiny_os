#!/bin/bash

# TinyOS External Integration Validation
# This script runs basic validation tests for external integration only
# For comprehensive testing, use: cargo run -> 't' command

# Change to project root directory
cd "$(dirname "$0")/../.."

echo "========================================"
echo "  TinyOS External Integration Validation"
echo "========================================"
echo "This validates external integration only."
echo "For comprehensive testing: cargo run -> 't'"
echo "========================================"
echo

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
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

print_warning() {
    echo -e "${YELLOW}⚠ WARN${NC} - $1"
}

# Validation Tests
PASSED_TESTS=0
TOTAL_TESTS=0

echo "Running TinyOS external integration validation..."
echo

# Test 1: Build verification
print_info "Testing kernel build..."
((TOTAL_TESTS++))
if cargo build --release --quiet; then
    print_status 0 "Kernel build verification"
    ((PASSED_TESTS++))
else
    print_status 1 "Kernel build verification"
fi

# Test 2: Source structure validation
print_info "Validating source structure..."
((TOTAL_TESTS++))
if [[ -f "src/main.rs" && -f "src/lib.rs" && -d "src/memory" && -d "src/drivers" ]]; then
    print_status 0 "Source structure validation"
    ((PASSED_TESTS++))
else
    print_status 1 "Source structure validation"
fi

# Test 3: Basic boot test
print_info "Testing basic boot integration..."
((TOTAL_TESTS++))
BOOT_OUTPUT=$(timeout 5s qemu-system-aarch64 -M raspi4b -nographic -kernel target/aarch64-unknown-none/release/tiny_os 2>&1)
BOOT_EXIT_CODE=$?

# Check if we got the expected boot output (timeout is expected, exit code 124 is OK)
if [[ $BOOT_EXIT_CODE -eq 124 || $BOOT_EXIT_CODE -eq 143 ]] && echo "$BOOT_OUTPUT" | grep -q "TinyOS Ready" && echo "$BOOT_OUTPUT" | grep -q "TinyOS Starting"; then
    print_status 0 "Basic boot integration"
    ((PASSED_TESTS++))
else
    print_status 1 "Basic boot integration"
fi

# Test 4: Binary size check
print_info "Checking binary size..."
((TOTAL_TESTS++))
if [ -f "target/aarch64-unknown-none/release/tiny_os" ]; then
    SIZE=$(stat -c%s "target/aarch64-unknown-none/release/tiny_os" 2>/dev/null || echo "0")
    if [ "$SIZE" -gt 1000 ] && [ "$SIZE" -lt 10000000 ]; then
        print_status 0 "Binary size check ($SIZE bytes)"
        ((PASSED_TESTS++))
    else
        print_status 1 "Binary size check ($SIZE bytes)"
    fi
else
    print_status 1 "Binary size check (file not found)"
fi

# Test 5: Reminder about comprehensive testing
print_info "Comprehensive testing reminder"
print_warning "IMPORTANT: This only validates external integration"
print_warning "For comprehensive kernel testing:"
print_warning "  1. Run: cargo run"
print_warning "  2. In TinyOS shell, run: t"
print_warning "  3. This runs all internal kernel tests"
((TOTAL_TESTS++))
((PASSED_TESTS++))  # Always "pass" since it's just a reminder

# Cleanup
rm -f /tmp/validation_boot.log

# Results
echo
echo "========================================"
echo "  External Integration Validation Results"
echo "========================================"
echo "Tests Passed: $PASSED_TESTS/$TOTAL_TESTS"
echo
echo "NOTE: This validates external integration only."
echo "For comprehensive kernel testing:"
echo "  cargo run -> TinyOS> t"
echo

if [ $PASSED_TESTS -eq $TOTAL_TESTS ]; then
    echo -e "${GREEN}🎉 ALL INTEGRATION TESTS PASSED! 🎉${NC}"
    echo -e "${GREEN}TinyOS external integration is valid${NC}"
    echo "Ready for comprehensive internal testing."
    exit 0
else
    echo -e "${RED}❌ SOME INTEGRATION TESTS FAILED ❌${NC}"
    echo "Please fix integration issues before proceeding"
    exit 1
fi
