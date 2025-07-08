#!/bin/bash

echo "========================================"
echo "  TinyOS Modular Driver Test Suite"
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
run_test "Library crate compilation" "cargo check --lib"
run_test "Binary crate compilation" "cargo check --bin tiny_os"
run_test "Release build" "cargo build --release"

# Test 2: Driver Module Structure
echo ""
echo "=== Driver Module Structure Tests ==="

DRIVERS=("uart" "gpio" "timer" "sdcard")
for driver in "${DRIVERS[@]}"; do
    run_test "$driver module directory" "test -d src/drivers/$driver"
    run_test "$driver mod.rs" "test -f src/drivers/$driver/mod.rs"
    run_test "$driver hardware.rs" "test -f src/drivers/$driver/hardware.rs"
    run_test "$driver driver.rs" "test -f src/drivers/$driver/driver.rs"
done

# Test 3: API Exports and Compatibility
echo ""
echo "=== API Compatibility Tests ==="

# Check that all expected types are exported
run_test "UART type export" "grep -q 'pub type Uart' src/drivers/uart/driver.rs"
run_test "GPIO type export" "grep -q 'pub type Gpio' src/drivers/gpio/driver.rs"
run_test "Timer type export" "grep -q 'pub type SystemTimer' src/drivers/timer/driver.rs"
run_test "SdCard type export" "grep -q 'pub type SdCard' src/drivers/sdcard/driver.rs"

# Check backward compatibility re-exports
run_test "UART compatibility re-export" "grep -q 'pub mod uart' src/lib.rs"
run_test "GPIO compatibility re-export" "grep -q 'pub mod gpio' src/lib.rs"
run_test "Timer compatibility re-export" "grep -q 'pub mod timer' src/lib.rs"
run_test "SdCard compatibility re-export" "grep -q 'pub mod sdcard' src/lib.rs"

# Test 4: Hardware Abstraction Layer
echo ""
echo "=== Hardware Abstraction Tests ==="

# Check that hardware modules have proper register definitions
run_test "UART hardware registers" "grep -q 'pub mod registers' src/drivers/uart/hardware.rs"
run_test "GPIO hardware registers" "grep -q 'pub mod registers' src/drivers/gpio/hardware.rs"
run_test "Timer hardware registers" "grep -q 'pub mod registers' src/drivers/timer/hardware.rs"
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

# Test 8: Legacy Driver Archive
echo ""
echo "=== Legacy Driver Archive Tests ==="

run_test "Legacy drivers directory" "test -d src/legacy_drivers"
run_test "Legacy UART archived" "test -f src/legacy_drivers/uart.rs"
run_test "Legacy GPIO archived" "test -f src/legacy_drivers/gpio.rs"
run_test "Legacy Timer archived" "test -f src/legacy_drivers/timer.rs"
run_test "Legacy SdCard archived" "test -f src/legacy_drivers/sdcard.rs"

# Test 9: Integration Tests
echo ""
echo "=== Integration Tests ==="

# Test that the kernel boots with new drivers
run_test "QEMU boot test" "./tests/test_qemu_boot.sh >/dev/null 2>&1"

# Test that binary size is reasonable
BINARY_SIZE=$(stat -c%s target/aarch64-unknown-none/release/tiny_os 2>/dev/null || echo "0")
if [[ $BINARY_SIZE -gt 0 && $BINARY_SIZE -lt 2000000 ]]; then
    log_success "Binary size reasonable ($BINARY_SIZE bytes)"
    ((TESTS_PASSED++))
else
    log_error "Binary size issue ($BINARY_SIZE bytes)"
    ((TESTS_FAILED++))
fi
((TOTAL_TESTS++))

# Test 10: Documentation Tests
echo ""
echo "=== Documentation Tests ==="

run_test "Driver module documentation" "grep -q '//!' src/drivers/mod.rs"
run_test "UART driver documentation" "grep -q '//!' src/drivers/uart/driver.rs"
run_test "GPIO driver documentation" "grep -q '//!' src/drivers/gpio/driver.rs"
run_test "Timer driver documentation" "grep -q '//!' src/drivers/timer/driver.rs"
run_test "SdCard driver documentation" "grep -q '//!' src/drivers/sdcard/driver.rs"

# Test Summary
echo ""
echo "========================================"
echo "  Driver Test Results Summary"
echo "========================================"
echo -e "Total Tests: ${BLUE}$TOTAL_TESTS${NC}"
echo -e "Passed: ${GREEN}$TESTS_PASSED${NC}"
echo -e "Failed: ${RED}$TESTS_FAILED${NC}"
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
