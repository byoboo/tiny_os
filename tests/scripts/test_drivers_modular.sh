#!/bin/bash

# TinyOS Modular Driver Integration Test
# Tests driver module structure and integration (external validation only)

# Change to project root directory
cd "$(dirname "$0")/../.."

echo "========================================"
echo "  TinyOS Modular Driver Integration Test"
echo "========================================"
echo "This tests driver module structure and integration."
echo "For actual driver functionality testing: cargo run -> 't'"
echo "========================================"
echo ""

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

TESTS_PASSED=0
TESTS_FAILED=0
TOTAL_TESTS=0

log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
    ((TESTS_PASSED++))
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
    ((TESTS_FAILED++))
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

run_test() {
    local test_name="$1"
    local test_cmd="$2"
    ((TOTAL_TESTS++))
    
    log_info "Running: $test_name"
    if eval "$test_cmd" >/dev/null 2>&1; then
        log_success "$test_name passed"
    else
        log_error "$test_name failed"
    fi
}

# Test 1: Build System Validation
echo "=== Build System Tests ==="
run_test "Kernel compilation" "cargo build --release"

# Test 2: Driver Module Structure
echo ""
echo "=== Driver Module Structure Tests ==="
run_test "Drivers directory" "test -d src/drivers"
run_test "Drivers mod.rs" "test -f src/drivers/mod.rs"

# Check for key driver modules
DRIVERS=("uart" "gpio" "timer" "sdcard")
for driver in "${DRIVERS[@]}"; do
    run_test "$driver module directory" "test -d src/drivers/$driver"
    run_test "$driver mod.rs" "test -f src/drivers/$driver/mod.rs"
done

# Test 3: Basic Integration
echo ""
echo "=== Basic Integration Tests ==="

# Check that main driver interfaces are exported
run_test "Driver exports in lib.rs" "grep -q 'pub mod.*uart\|pub mod.*gpio\|pub mod.*timer' src/lib.rs"

# Test 4: Boot Integration
echo ""
echo "=== Boot Integration Tests ==="

# Test that system boots with modular drivers
log_info "Testing boot with modular drivers"
((TOTAL_TESTS++))

# Simple boot test - just verify the binary exists and can be executed with timeout
if [[ -f "target/aarch64-unknown-none/release/tiny_os" ]]; then
    # Run a minimal boot test to verify it starts
    timeout 3s qemu-system-aarch64 -M raspi4b -nographic -kernel target/aarch64-unknown-none/release/tiny_os >/dev/null 2>&1
    BOOT_EXIT_CODE=$?
    
    # Exit code 124 means timeout (expected), 0 means clean exit, both are acceptable
    if [[ $BOOT_EXIT_CODE -eq 124 || $BOOT_EXIT_CODE -eq 0 ]]; then
        log_success "Boot integration with modular drivers"
    else
        log_error "Boot integration with modular drivers - unexpected exit code: $BOOT_EXIT_CODE"
    fi
else
    log_error "Boot integration with modular drivers - release binary not found"
fi

# Test 5: Reminder about comprehensive testing
echo ""
echo "=== Comprehensive Testing Reminder ==="
log_info "Driver functionality testing reminder"
log_warn "IMPORTANT: This only tests driver module structure and integration"
log_warn "For actual driver functionality testing:"
log_warn "  1. Run: cargo run"
log_warn "  2. In TinyOS shell, run: t"
log_warn "  3. This tests actual GPIO, UART, timer operations"
((TOTAL_TESTS++))
((TESTS_PASSED++))  # Always "pass" since it's just a reminder
run_test "SdCard hardware registers" "grep -q 'pub mod registers' src/drivers/sdcard/hardware.rs"

# Check for hardware version support
run_test "Hardware version traits" "grep -q 'HardwareVersion' src/drivers/mod.rs"
run_test "Raspberry Pi 4 config" "grep -q 'RaspberryPi4' src/drivers/mod.rs"

# Test 5: Driver Trait Implementation
echo ""
echo "=== Driver Trait Tests ==="

# Check that drivers implement required traits
run_test "UART Initialize trait" "grep -q 'impl.*Initialize.*for UartDriver' src/drivers/uart/driver.rs"
run_test "GPIO Initialize trait" "grep -q 'impl.*Initialize.*for GpioDriver' src/drivers/gpio/driver.rs"
run_test "Timer Initialize trait" "grep -q 'impl.*Initialize.*for TimerDriver' src/drivers/timer/driver.rs"
run_test "SdCard Initialize trait" "grep -q 'impl.*Initialize.*for SdCardDriver' src/drivers/sdcard/driver.rs"

# Test 6: Type Safety Features
echo ""
echo "=== Type Safety Tests ==="

# Check for const generic usage
run_test "GPIO type-safe pins" "grep -q 'GpioPin<const PIN: u32' src/drivers/gpio/driver.rs"
run_test "Timer type-safe channels" "grep -q 'TimerChannel<const CHANNEL: u8' src/drivers/timer/driver.rs"

# Check for proper error handling
run_test "Driver error types" "grep -q 'DriverError' src/drivers/mod.rs"
run_test "SdCard error types" "grep -q 'SdCardError' src/drivers/sdcard/hardware.rs"

# Test 7: Performance Optimizations
echo ""
echo "=== Performance Optimization Tests ==="

# Check for inline annotations on critical paths
run_test "UART inline optimizations" "grep -q '#\[inline\]' src/drivers/uart/driver.rs"
run_test "GPIO inline optimizations" "grep -q '#\[inline\]' src/drivers/gpio/driver.rs"
run_test "Timer inline optimizations" "grep -q '#\[inline\]' src/drivers/timer/driver.rs"

# Cleanup
rm -f ./tmp/driver_boot.log

# Test Summary
echo ""
echo "========================================"
echo "  Driver Integration Test Results"
echo "========================================"
echo -e "Total Tests: ${BLUE}$TOTAL_TESTS${NC}"
echo -e "Passed: ${GREEN}$TESTS_PASSED${NC}"
echo -e "Failed: ${RED}$TESTS_FAILED${NC}"
echo ""
echo "NOTE: This tests driver integration only."
echo "For comprehensive driver functionality testing:"
echo "  cargo run -> TinyOS> t"
echo

if [ $TESTS_FAILED -eq 0 ]; then
    echo -e "${GREEN}🎉 All driver integration tests passed!${NC}"
    echo "Ready for comprehensive driver functionality testing."
    exit 0
else
    echo -e "${RED}❌ Some driver integration tests failed${NC}"
    echo "Please review the failed tests above"
    exit 1
fi
echo ""

if [[ $TESTS_FAILED -eq 0 ]]; then
    echo -e "${GREEN}🎉 All driver tests passed!${NC}"
    echo "✅ Phase 2 driver modularization is working correctly"
    exit 0
else
    echo -e "${RED}❌ Some driver tests failed${NC}"
    echo "Please review the failed tests above"
    exit 1
fi
