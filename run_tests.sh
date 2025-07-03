#!/bin/bash

# TinyOS Test Suite Runner
# This script provides a unified interface to run unit tests and organized integration test suites

echo "========================================"
echo "  TinyOS Test Suite Runner"
echo "========================================"
echo

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
PURPLE='\033[0;35m'
NC='\033[0m' # No Color

# Function to print colored output
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

show_usage() {
    echo -e "${CYAN}TinyOS Test Suite Runner${NC}"
    echo
    echo -e "${YELLOW}Usage:${NC} $0 [OPTIONS] [TEST_TYPE]"
    echo
    echo -e "${YELLOW}Test Types:${NC}"
    echo "  unit        Run Rust unit tests only"
    echo "  integration Run integration test suites only"
    echo "  all         Run both unit and integration tests (default)"
    echo
    echo -e "${YELLOW}Options:${NC}"
    echo "  -m, --mode MODE     Integration test mode: interactive, automated, quick (default: interactive)"
    echo "  -v, --verbose       Enable verbose output"
    echo "  -h, --help         Show this help message"
    echo
    echo -e "${YELLOW}Examples:${NC}"
    echo "  $0                           # Run all tests"
    echo "  $0 unit                      # Run unit tests only"
    echo "  $0 integration --mode quick  # Run integration tests in quick mode"
}

# Parse command line arguments
TEST_TYPE="all"
INTEGRATION_MODE="interactive"
VERBOSE="false"

while [[ $# -gt 0 ]]; do
    case $1 in
        -m|--mode)
            INTEGRATION_MODE="$2"
            if [[ ! "$INTEGRATION_MODE" =~ ^(interactive|automated|quick)$ ]]; then
                echo -e "${RED}Error: Invalid mode '$INTEGRATION_MODE'. Use: interactive, automated, or quick${NC}"
                exit 1
            fi
            shift 2
            ;;
        -v|--verbose)
            VERBOSE="true"
            shift
            ;;
        -h|--help)
            show_usage
            exit 0
            ;;
        unit|integration|all)
            TEST_TYPE="$1"
            shift
            ;;
        *)
            echo -e "${RED}Error: Unknown option '$1'${NC}"
            show_usage
            exit 1
            ;;
    esac
done

# Test configuration
CARGO_TEST_FLAGS="--lib --verbose"
TEST_TIMEOUT=300  # 5 minutes

print_info "Starting TinyOS test suite (Type: $TEST_TYPE, Integration Mode: $INTEGRATION_MODE)..."
echo

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ] || [ ! -d "src" ]; then
    echo -e "${RED}Error: Must be run from TinyOS project root${NC}"
    exit 1
fi
# Variables for test results
UNIT_TESTS_RESULT=0
INTEGRATION_TESTS_RESULT=0

# Run unit tests if requested
if [[ "$TEST_TYPE" == "unit" || "$TEST_TYPE" == "all" ]]; then
    # Clean previous test artifacts
    print_info "Cleaning previous build artifacts..."
    cargo clean --quiet
    print_status $? "Clean previous artifacts"

    # Build the library for testing
    print_info "Building TinyOS library for testing..."
    cargo build --lib
    BUILD_RESULT=$?
    print_status $BUILD_RESULT "Build TinyOS library"

    if [ $BUILD_RESULT -ne 0 ]; then
        echo -e "${RED}Build failed, cannot proceed with tests${NC}"
        exit 1
    fi

    echo
    echo "----------------------------------------"
    echo "  Unit Tests"
    echo "----------------------------------------"

    # Run unit tests
    print_info "Running unit tests..."

    # Memory tests
    print_info "Testing memory management..."
    timeout $TEST_TIMEOUT cargo test $CARGO_TEST_FLAGS memory_tests::
    MEMORY_RESULT=$?
    print_status $MEMORY_RESULT "Memory management tests"

    # UART tests
    print_info "Testing UART functionality..."
    timeout $TEST_TIMEOUT cargo test $CARGO_TEST_FLAGS uart_tests::
    UART_RESULT=$?
    print_status $UART_RESULT "UART functionality tests"

    # GPIO tests
    print_info "Testing GPIO control..."
    timeout $TEST_TIMEOUT cargo test $CARGO_TEST_FLAGS gpio_tests::
    GPIO_RESULT=$?
    print_status $GPIO_RESULT "GPIO control tests"

    # Timer tests
    print_info "Testing timer functionality..."
    timeout $TEST_TIMEOUT cargo test $CARGO_TEST_FLAGS timer_tests::
    TIMER_RESULT=$?
    print_status $TIMER_RESULT "Timer functionality tests"

    # Interrupt tests
    print_info "Testing interrupt management..."
    timeout $TEST_TIMEOUT cargo test $CARGO_TEST_FLAGS interrupt_tests::
    INTERRUPT_RESULT=$?
    print_status $INTERRUPT_RESULT "Interrupt management tests"

    # Calculate unit test result
    if [[ $MEMORY_RESULT -eq 0 && $UART_RESULT -eq 0 && $GPIO_RESULT -eq 0 && $TIMER_RESULT -eq 0 && $INTERRUPT_RESULT -eq 0 ]]; then
        UNIT_TESTS_RESULT=0
    else
        UNIT_TESTS_RESULT=1
    fi
fi

# Run integration tests if requested
if [[ "$TEST_TYPE" == "integration" || "$TEST_TYPE" == "all" ]]; then
    echo
    echo "----------------------------------------"
    echo "  Integration Test Suites"
    echo "----------------------------------------"

    # Run the unified test suite runner
    print_info "Running organized integration test suites..."
    
    SUITE_RUNNER="$(dirname "$0")/run_test_suites.sh"
    if [[ ! -f "$SUITE_RUNNER" ]]; then
        echo -e "${RED}Error: Test suite runner not found: $SUITE_RUNNER${NC}"
        INTEGRATION_TESTS_RESULT=1
    else
        # Build integration test command
        INTEGRATION_CMD="$SUITE_RUNNER all --mode $INTEGRATION_MODE"
        if [[ "$VERBOSE" == "true" ]]; then
            INTEGRATION_CMD="$INTEGRATION_CMD --verbose"
        fi
        
        print_info "Executing: $INTEGRATION_CMD"
        if $INTEGRATION_CMD; then
            INTEGRATION_TESTS_RESULT=0
            print_status 0 "Integration test suites"
        else
            INTEGRATION_TESTS_RESULT=1
            print_status 1 "Integration test suites"
        fi
    fi
fi

echo
echo "----------------------------------------"
echo "  Test Summary"
echo "----------------------------------------"

# Run all tests together
print_info "Running complete test suite..."
timeout $TEST_TIMEOUT cargo test $CARGO_TEST_FLAGS
ALL_TESTS_RESULT=$?
print_status $ALL_TESTS_RESULT "Complete test suite"

echo
echo "========================================"
echo "  Test Results Summary"
echo "========================================"

# Calculate results
UNIT_TESTS_PASSED=0
INTEGRATION_TESTS_PASSED=0
TOTAL_TESTS=7

# Count unit test results
[ $MEMORY_RESULT -eq 0 ] && ((UNIT_TESTS_PASSED++))
[ $UART_RESULT -eq 0 ] && ((UNIT_TESTS_PASSED++))
[ $GPIO_RESULT -eq 0 ] && ((UNIT_TESTS_PASSED++))
[ $TIMER_RESULT -eq 0 ] && ((UNIT_TESTS_PASSED++))
[ $INTERRUPT_RESULT -eq 0 ] && ((UNIT_TESTS_PASSED++))

# Count integration test results
[ $INTEGRATION_RESULT -eq 0 ] && ((INTEGRATION_TESTS_PASSED++))
[ $PERFORMANCE_RESULT -eq 0 ] && ((INTEGRATION_TESTS_PASSED++))

echo "Unit Tests:        $UNIT_TESTS_PASSED/5 passed"
echo "Integration Tests: $INTEGRATION_TESTS_PASSED/2 passed"
echo "Total:             $(($UNIT_TESTS_PASSED + $INTEGRATION_TESTS_PASSED))/$TOTAL_TESTS passed"

# Calculate overall results
if [[ "$TEST_TYPE" == "unit" ]]; then
    ALL_TESTS_RESULT=$UNIT_TESTS_RESULT
    print_info "Unit tests result: $([ $UNIT_TESTS_RESULT -eq 0 ] && echo "PASS" || echo "FAIL")"
elif [[ "$TEST_TYPE" == "integration" ]]; then
    ALL_TESTS_RESULT=$INTEGRATION_TESTS_RESULT
    print_info "Integration tests result: $([ $INTEGRATION_TESTS_RESULT -eq 0 ] && echo "PASS" || echo "FAIL")"
else
    # Both unit and integration tests
    if [[ $UNIT_TESTS_RESULT -eq 0 && $INTEGRATION_TESTS_RESULT -eq 0 ]]; then
        ALL_TESTS_RESULT=0
    else
        ALL_TESTS_RESULT=1
    fi
    print_info "Unit tests result: $([ $UNIT_TESTS_RESULT -eq 0 ] && echo "PASS" || echo "FAIL")"
    print_info "Integration tests result: $([ $INTEGRATION_TESTS_RESULT -eq 0 ] && echo "PASS" || echo "FAIL")"
fi

# Overall result
if [ $ALL_TESTS_RESULT -eq 0 ]; then
    echo
    echo -e "${GREEN}🎉 ALL TESTS PASSED! 🎉${NC}"
    echo -e "${GREEN}TinyOS is ready for deployment${NC}"
    EXIT_CODE=0
else
    echo
    echo -e "${RED}❌ SOME TESTS FAILED ❌${NC}"
    echo -e "${RED}Please review failed tests before deployment${NC}"
    EXIT_CODE=1
fi

echo
echo "Test suite completed in $(date)"
echo "========================================"

# Generate test report
print_info "Generating test report..."
cat > test_report.md << EOF
# TinyOS Test Report

Generated: $(date)

## Test Configuration
- Test Type: $TEST_TYPE
- Integration Mode: $INTEGRATION_MODE
- Verbose Output: $VERBOSE

## Test Results Summary

| Test Category | Status | Details |
|---------------|--------|---------|
| Unit Tests | $([ $UNIT_TESTS_RESULT -eq 0 ] && echo "✅ PASS" || echo "❌ FAIL") | Rust unit tests for core components |
| Integration Tests | $([ $INTEGRATION_TESTS_RESULT -eq 0 ] && echo "✅ PASS" || echo "❌ FAIL") | Organized test suites for OS functionality |

## Overall Status

$([ $ALL_TESTS_RESULT -eq 0 ] && echo "🎉 **ALL TESTS PASSED**" || echo "❌ **SOME TESTS FAILED**")

## Test Suite Organization
- **Memory Management Suite**: Allocation, protection, fragmentation tests
- **Interrupt Management Suite**: Handler, priority, nested interrupt tests  
- **Hardware/Driver Suite**: GPIO, UART, Timer functionality tests

For detailed test suite information, run: \`./run_test_suites.sh --list\`
EOF

print_status 0 "Test report generated: test_report.md"

exit $EXIT_CODE
**Total**: $(($UNIT_TESTS_PASSED + $INTEGRATION_TESTS_PASSED))/$TOTAL_TESTS passed

## Test Coverage

- ✅ Memory allocation and deallocation
- ✅ Memory corruption detection
- ✅ UART input/output operations
- ✅ GPIO pin control and functions
- ✅ Timer functionality and comparisons
- ✅ Interrupt enable/disable/triggering
- ✅ System boot sequence simulation
- ✅ Shell command processing
- ✅ System health checks
- ✅ Stress testing scenarios
- ✅ Performance benchmarking

## Next Steps

$([ $ALL_TESTS_RESULT -eq 0 ] && echo "- Deploy to real hardware for validation" || echo "- Fix failing tests before deployment")
- Run tests on actual Raspberry Pi hardware
- Implement additional test scenarios
- Add continuous integration pipeline

EOF

print_status 0 "Generated test_report.md"

exit $EXIT_CODE
