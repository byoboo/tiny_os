#!/bin/bash

# TinyOS Unified Test Runner
# Simple and robust version

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
PURPLE='\033[0;35m'
NC='\033[0m' # No Color

# Script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
TESTS_DIR="${SCRIPT_DIR}/tests"

# Test results tracking
TOTAL_TESTS=0
PASSED_TESTS=0
FAILED_TESTS=0

# Configuration
VERBOSE=false
DIAGNOSTIC=true  # Show diagnostic output by default
MODE="automated"
INTERACTIVE=false  # Use automated tests by default

# Functions
print_header() {
    echo -e "${CYAN}========================================${NC}"
    echo -e "${CYAN}    TinyOS Unified Test Runner${NC}"
    echo -e "${CYAN}    Organized by OS Features${NC}"
    echo -e "${CYAN}========================================${NC}"
    echo
}

print_status() {
    if [ $1 -eq 0 ]; then
        echo -e "${GREEN}✓ PASS${NC} - $2"
        ((PASSED_TESTS++))
    else
        echo -e "${RED}✗ FAIL${NC} - $2"
        ((FAILED_TESTS++))
    fi
    ((TOTAL_TESTS++))
}

print_info() {
    echo -e "${BLUE}ℹ INFO${NC} - $1"
}

print_warning() {
    echo -e "${YELLOW}⚠ WARN${NC} - $1"
}

print_feature_header() {
    echo
    echo -e "${PURPLE}═══════════════════════════════════════${NC}"
    echo -e "${PURPLE}  $1${NC}"
    echo -e "${PURPLE}═══════════════════════════════════════${NC}"
    echo
}

show_usage() {
    echo -e "${CYAN}TinyOS Unified Test Runner${NC}"
    echo
    echo -e "${YELLOW}Usage:${NC} $0 [OPTIONS] [FEATURES]"
    echo
    echo -e "${YELLOW}Features (OS Components):${NC}"
    echo "  boot        Boot system tests and validation"
    echo "  memory      Memory management and allocation tests"
    echo "  interrupts  Interrupt handling and priority tests"
    echo "  hardware    Hardware abstraction and driver tests"
    echo "  unit        Rust unit tests"
    echo "  all         Run all features (default)"
    echo
    echo -e "${YELLOW}Options:${NC}"
    echo "  -v, --verbose       Enable verbose output"
    echo "  -d, --diagnostic    Show diagnostic output on failures (default for non-verbose)"
    echo "  -i, --interactive   Use interactive test suites (requires expect)"
    echo "  -h, --help         Show this help message"
    echo "  --list             List available test features"
    echo "  --validate-only    Run only basic validation tests"
    echo
    echo -e "${YELLOW}Examples:${NC}"
    echo "  $0                           # Run all automated tests (default)"
    echo "  $0 memory                    # Run automated memory tests only"
    echo "  $0 --interactive             # Run interactive test suites (requires expect)"
    echo "  $0 --verbose                 # Run all tests with full verbose output"
    echo "  $0 --validate-only           # Run basic validation only"
    echo
    echo -e "${YELLOW}Notes:${NC}"
    echo "  - Automated tests are the default and require no external dependencies"
    echo "  - Interactive tests require the 'expect' tool and --interactive flag"
    echo "  - Use --verbose for full output, or --diagnostic for error details only"
}

list_features() {
    echo -e "${CYAN}Available TinyOS Test Features:${NC}"
    echo
    echo -e "  ${GREEN}boot${NC}: Boot System Tests"
    echo -e "  ${GREEN}memory${NC}: Memory Management Tests"
    echo -e "  ${GREEN}unit${NC}: Rust Unit Tests"
    echo -e "  ${GREEN}interrupts${NC}: Interrupt Management Tests"
    echo -e "  ${GREEN}hardware${NC}: Hardware/Driver Tests"
    echo
    echo -e "${BLUE}Use '$0 <feature>' to run specific tests${NC}"
}

run_boot_tests() {
    print_feature_header "Boot System Tests"
    
    # QEMU boot test
    if [[ -f "${TESTS_DIR}/test_qemu_boot.sh" ]]; then
        print_info "Running QEMU boot validation test"
        if $VERBOSE; then
            timeout 30s bash "${TESTS_DIR}/test_qemu_boot.sh"
            exit_code=$?
        else
            timeout 30s bash "${TESTS_DIR}/test_qemu_boot.sh" > /dev/null 2>&1
            exit_code=$?
        fi
        print_status $exit_code "QEMU boot test"
    else
        print_warning "Boot test script not found: tests/test_qemu_boot.sh"
    fi
    
    # Basic validation
    if [[ -f "${TESTS_DIR}/validate_tinyos.sh" ]]; then
        print_info "Running basic system validation"
        if $VERBOSE; then
            bash "${TESTS_DIR}/validate_tinyos.sh"
            exit_code=$?
        else
            # Capture error output for diagnosis
            error_output=$(bash "${TESTS_DIR}/validate_tinyos.sh" 2>&1)
            exit_code=$?
            if [ $exit_code -ne 0 ] && $DIAGNOSTIC; then
                print_warning "System validation failed. Last error output:"
                echo "$error_output" | tail -5 | sed 's/^/  /'
            fi
        fi
        print_status $exit_code "System validation"
    else
        print_warning "Validation test script not found: tests/validate_tinyos.sh"
    fi
}

run_memory_tests() {
    print_feature_header "Memory Management Tests"
    
    if $INTERACTIVE; then
        # Use interactive test suite when explicitly requested
        if [[ -f "${TESTS_DIR}/test_memory_suite.sh" ]]; then
            print_info "Running interactive memory test suite (requires expect)"
            if $VERBOSE; then
                timeout 60s bash "${TESTS_DIR}/test_memory_suite.sh" --automated
                exit_code=$?
            else
                error_output=$(timeout 60s bash "${TESTS_DIR}/test_memory_suite.sh" --automated 2>&1)
                exit_code=$?
                if [ $exit_code -ne 0 ] && $DIAGNOSTIC; then
                    print_warning "Interactive memory tests failed. Last error output:"
                    echo "$error_output" | tail -5 | sed 's/^/  /'
                fi
            fi
            print_status $exit_code "Memory test suite (interactive)"
        else
            print_warning "Interactive memory test suite not found: tests/test_memory_suite.sh"
        fi
    else
        # Use automated test suite by default
        if [[ -f "${TESTS_DIR}/test_memory_automated.sh" ]]; then
            print_info "Running automated memory test suite"
            if $VERBOSE; then
                bash "${TESTS_DIR}/test_memory_automated.sh"
                exit_code=$?
            else
                error_output=$(bash "${TESTS_DIR}/test_memory_automated.sh" 2>&1)
                exit_code=$?
                if [ $exit_code -ne 0 ] && $DIAGNOSTIC; then
                    print_warning "Memory tests failed. Last error output:"
                    echo "$error_output" | tail -5 | sed 's/^/  /'
                fi
            fi
            print_status $exit_code "Memory test suite (automated)"
        else
            print_warning "Automated memory test suite not found: tests/test_memory_automated.sh"
        fi
    fi
}

run_interrupt_tests() {
    print_feature_header "Interrupt Management Tests"
    
    if $INTERACTIVE; then
        # Use interactive test suite when explicitly requested
        if [[ -f "${TESTS_DIR}/test_interrupt_suite.sh" ]]; then
            print_info "Running interactive interrupt test suite (requires expect)"
            if $VERBOSE; then
                timeout 60s bash "${TESTS_DIR}/test_interrupt_suite.sh" --automated
                exit_code=$?
            else
                error_output=$(timeout 60s bash "${TESTS_DIR}/test_interrupt_suite.sh" --automated 2>&1)
                exit_code=$?
                if [ $exit_code -ne 0 ] && $DIAGNOSTIC; then
                    print_warning "Interactive interrupt tests failed. Last error output:"
                    echo "$error_output" | tail -5 | sed 's/^/  /'
                fi
            fi
            print_status $exit_code "Interrupt test suite (interactive)"
        else
            print_warning "Interactive interrupt test suite not found: tests/test_interrupt_suite.sh"
        fi
    else
        # Use automated test suite by default
        if [[ -f "${TESTS_DIR}/test_interrupt_automated.sh" ]]; then
            print_info "Running automated interrupt test suite"
            if $VERBOSE; then
                bash "${TESTS_DIR}/test_interrupt_automated.sh"
                exit_code=$?
            else
                error_output=$(bash "${TESTS_DIR}/test_interrupt_automated.sh" 2>&1)
                exit_code=$?
                if [ $exit_code -ne 0 ] && $DIAGNOSTIC; then
                    print_warning "Interrupt tests failed. Last error output:"
                    echo "$error_output" | tail -5 | sed 's/^/  /'
                fi
            fi
            print_status $exit_code "Interrupt test suite (automated)"
        else
            print_warning "Automated interrupt test suite not found: tests/test_interrupt_automated.sh"
        fi
    fi
}

run_hardware_tests() {
    print_feature_header "Hardware/Driver Tests"
    
    if $INTERACTIVE; then
        # Use interactive test suite when explicitly requested
        if [[ -f "${TESTS_DIR}/test_hardware_suite.sh" ]]; then
            print_info "Running interactive hardware test suite (requires expect)"
            if $VERBOSE; then
                timeout 60s bash "${TESTS_DIR}/test_hardware_suite.sh" --automated
                exit_code=$?
            else
                error_output=$(timeout 60s bash "${TESTS_DIR}/test_hardware_suite.sh" --automated 2>&1)
                exit_code=$?
                if [ $exit_code -ne 0 ] && $DIAGNOSTIC; then
                    print_warning "Interactive hardware tests failed. Last error output:"
                    echo "$error_output" | tail -5 | sed 's/^/  /'
                fi
            fi
            print_status $exit_code "Hardware test suite (interactive)"
        else
            print_warning "Interactive hardware test suite not found: tests/test_hardware_suite.sh"
        fi
    else
        # Use automated test suite by default
        if [[ -f "${TESTS_DIR}/test_hardware_automated.sh" ]]; then
            print_info "Running automated hardware test suite"
            if $VERBOSE; then
                bash "${TESTS_DIR}/test_hardware_automated.sh"
                exit_code=$?
            else
                error_output=$(bash "${TESTS_DIR}/test_hardware_automated.sh" 2>&1)
                exit_code=$?
                if [ $exit_code -ne 0 ] && $DIAGNOSTIC; then
                    print_warning "Hardware tests failed. Last error output:"
                    echo "$error_output" | tail -5 | sed 's/^/  /'
                fi
            fi
            print_status $exit_code "Hardware test suite (automated)"
        else
            print_warning "Automated hardware test suite not found: tests/test_hardware_automated.sh"
        fi
    fi
}

run_unit_tests() {
    print_feature_header "Rust Unit Tests"
    
    print_info "Running Rust unit tests (host target)"
    cd "$SCRIPT_DIR"
    
    # Use host target for unit tests since the embedded target doesn't support std
    if $VERBOSE; then
        cargo test --target "$(rustc -vV | sed -n 's|host: ||p')" --lib
        exit_code=$?
    else
        cargo test --target "$(rustc -vV | sed -n 's|host: ||p')" --lib > /dev/null 2>&1
        exit_code=$?
    fi
    
    if [ $exit_code -ne 0 ] && ! $VERBOSE && $DIAGNOSTIC; then
        print_warning "Unit tests failed. Run with --verbose for details."
        print_info "Note: Unit tests run on host target, not embedded target"
    fi
    
    print_status $exit_code "Rust unit tests"
}

run_validation_only() {
    print_feature_header "Basic Validation Tests"
    
    print_info "Running TinyOS validation tests only"
    if [[ -f "${TESTS_DIR}/validate_tinyos.sh" ]]; then
        bash "${TESTS_DIR}/validate_tinyos.sh"
        exit_code=$?
        print_status $exit_code "Basic validation"
    else
        print_warning "Validation test script not found: tests/validate_tinyos.sh"
    fi
}

print_summary() {
    echo
    echo -e "${CYAN}========================================${NC}"
    echo -e "${CYAN}           Test Summary${NC}"
    echo -e "${CYAN}========================================${NC}"
    echo -e "Total Tests:  ${TOTAL_TESTS}"
    echo -e "Passed:       ${GREEN}${PASSED_TESTS}${NC}"
    echo -e "Failed:       ${RED}${FAILED_TESTS}${NC}"
    
    if [ $FAILED_TESTS -eq 0 ]; then
        echo -e "${GREEN}🎉 All tests passed!${NC}"
        exit 0
    else
        echo -e "${RED}❌ Some tests failed${NC}"
        exit 1
    fi
}

# Parse command line arguments
FEATURES="all"
VALIDATE_ONLY=false
MODE="automated"  # Default to automated for better CI/CD compatibility

while [[ $# -gt 0 ]]; do
    case $1 in
        -v|--verbose)
            VERBOSE=true
            DIAGNOSTIC=true
            shift
            ;;
        -d|--diagnostic)
            DIAGNOSTIC=true
            shift
            ;;
        --no-diagnostic)
            DIAGNOSTIC=false
            shift
            ;;
        -i|--interactive)
            INTERACTIVE=true
            shift
            ;;
        -h|--help)
            show_usage
            exit 0
            ;;
        --list)
            list_features
            exit 0
            ;;
        --validate-only)
            VALIDATE_ONLY=true
            shift
            ;;
        boot|memory|interrupts|hardware|unit|all)
            if [[ "$FEATURES" == "all" ]]; then
                FEATURES="$1"
            else
                FEATURES="$FEATURES $1"
            fi
            shift
            ;;
        *)
            echo -e "${RED}Unknown option: $1${NC}"
            show_usage
            exit 1
            ;;
    esac
done

# Validate mode
if [[ ! "$MODE" =~ ^(interactive|automated|quick)$ ]]; then
    echo -e "${RED}Invalid mode: $MODE${NC}"
    echo -e "Valid modes: interactive, automated, quick"
    exit 1
fi

# Main execution
print_header

if $VALIDATE_ONLY; then
    run_validation_only
    print_summary
    exit $?
fi

print_info "Mode: automated"
if $INTERACTIVE; then
    print_info "Using interactive test suites (requires expect)"
else
    print_info "Using automated test suites (no external dependencies)"
fi
if $VERBOSE; then
    print_info "Verbose output enabled"
elif $DIAGNOSTIC; then
    print_info "Diagnostic output enabled (errors shown)"
else
    print_info "Minimal output mode"
fi
print_info "Features: $FEATURES"
echo

# Run tests based on features
if [[ "$FEATURES" == "all" ]]; then
    run_boot_tests
    run_unit_tests
    run_memory_tests
    run_interrupt_tests
    run_hardware_tests
else
    for feature in $FEATURES; do
        case $feature in
            boot)
                run_boot_tests
                ;;
            memory)
                run_memory_tests
                ;;
            interrupts)
                run_interrupt_tests
                ;;
            hardware)
                run_hardware_tests
                ;;
            unit)
                run_unit_tests
                ;;
            *)
                print_warning "Unknown feature: $feature"
                ;;
        esac
    done
fi

print_summary
