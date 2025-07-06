#!/bin/bash

# TinyOS Unified Test Suite Runner
# Provides a unified interface to run all organized test suites

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Test suites
MEMORY_SUITE="${SCRIPT_DIR}/test_memory_suite.sh"
INTERRUPT_SUITE="${SCRIPT_DIR}/test_interrupt_suite.sh"
HARDWARE_SUITE="${SCRIPT_DIR}/test_hardware_suite.sh"

# Display usage information
show_usage() {
    echo -e "${CYAN}TinyOS Unified Test Suite Runner${NC}"
    echo
    echo -e "${YELLOW}Usage:${NC} $0 [OPTIONS] [SUITE]"
    echo
    echo -e "${YELLOW}Test Suites:${NC}"
    echo "  memory      Run memory management tests"
    echo "  interrupt   Run interrupt management tests"
    echo "  hardware    Run hardware/driver tests"
    echo "  all         Run all test suites (default)"
    echo
    echo -e "${YELLOW}Options:${NC}"
    echo "  -m, --mode MODE     Test mode: interactive, automated, quick (default: interactive)"
    echo "  -v, --verbose       Enable verbose output"
    echo "  -h, --help         Show this help message"
    echo "  --list             List available test suites"
    echo
    echo -e "${YELLOW}Examples:${NC}"
    echo "  $0                           # Run all tests interactively"
    echo "  $0 memory                    # Run memory tests interactively"
    echo "  $0 --mode automated all      # Run all tests in automated mode"
    echo "  $0 --mode quick interrupt    # Run interrupt tests in quick mode"
}

# List available test suites
list_suites() {
    echo -e "${CYAN}Available Test Suites:${NC}"
    echo
    echo -e "${GREEN}Memory Management Suite:${NC}"
    echo "  - Allocation/deallocation tests"
    echo "  - Memory protection tests"
    echo "  - Fragmentation analysis"
    echo "  - Performance benchmarks"
    echo
    echo -e "${GREEN}Interrupt Management Suite:${NC}"
    echo "  - Interrupt handling tests"
    echo "  - Priority testing"
    echo "  - Nested interrupt tests"
    echo "  - Performance analysis"
    echo
    echo -e "${GREEN}Hardware/Driver Suite:${NC}"
    echo "  - GPIO functionality"
    echo "  - UART communication"
    echo "  - Timer operations"
    echo "  - Hardware abstraction layer"
}

# Run a specific test suite
run_suite() {
    local suite_name="$1"
    local suite_script="$2"
    local mode="$3"
    local verbose="$4"
    
    echo -e "${PURPLE}=== Running ${suite_name} Test Suite ===${NC}"
    echo
    
    if [[ ! -f "$suite_script" ]]; then
        echo -e "${RED}Error: Test suite script not found: $suite_script${NC}"
        return 1
    fi
    
    # Make script executable
    chmod +x "$suite_script"
    
    # Build command
    local cmd="$suite_script"
    if [[ "$mode" != "interactive" ]]; then
        cmd="$cmd --mode $mode"
    fi
    if [[ "$verbose" == "true" ]]; then
        cmd="$cmd --verbose"
    fi
    
    # Run the test suite
    echo -e "${BLUE}Executing: $cmd${NC}"
    echo
    
    if $cmd; then
        echo -e "${GREEN}✓ ${suite_name} tests completed successfully${NC}"
        return 0
    else
        echo -e "${RED}✗ ${suite_name} tests failed${NC}"
        return 1
    fi
}

# Main execution
main() {
    local mode="interactive"
    local verbose="false"
    local suite="all"
    local failed_suites=()
    local total_suites=0
    local passed_suites=0
    
    # Parse command line arguments
    while [[ $# -gt 0 ]]; do
        case $1 in
            -m|--mode)
                mode="$2"
                if [[ ! "$mode" =~ ^(interactive|automated|quick)$ ]]; then
                    echo -e "${RED}Error: Invalid mode '$mode'. Use: interactive, automated, or quick${NC}"
                    exit 1
                fi
                shift 2
                ;;
            -v|--verbose)
                verbose="true"
                shift
                ;;
            -h|--help)
                show_usage
                exit 0
                ;;
            --list)
                list_suites
                exit 0
                ;;
            memory|interrupt|hardware|all)
                suite="$1"
                shift
                ;;
            *)
                echo -e "${RED}Error: Unknown option '$1'${NC}"
                show_usage
                exit 1
                ;;
        esac
    done
    
    echo -e "${CYAN}TinyOS Unified Test Suite Runner${NC}"
    echo -e "${YELLOW}Mode: $mode${NC}"
    echo -e "${YELLOW}Verbose: $verbose${NC}"
    echo -e "${YELLOW}Suite: $suite${NC}"
    echo
    
    # Run specified test suite(s)
    case "$suite" in
        memory)
            total_suites=1
            if run_suite "Memory Management" "$MEMORY_SUITE" "$mode" "$verbose"; then
                ((passed_suites++))
            else
                failed_suites+=("Memory Management")
            fi
            ;;
        interrupt)
            total_suites=1
            if run_suite "Interrupt Management" "$INTERRUPT_SUITE" "$mode" "$verbose"; then
                ((passed_suites++))
            else
                failed_suites+=("Interrupt Management")
            fi
            ;;
        hardware)
            total_suites=1
            if run_suite "Hardware/Driver" "$HARDWARE_SUITE" "$mode" "$verbose"; then
                ((passed_suites++))
            else
                failed_suites+=("Hardware/Driver")
            fi
            ;;
        all)
            total_suites=3
            
            echo -e "${PURPLE}=== Running All Test Suites ===${NC}"
            echo
            
            # Memory Management Suite
            echo -e "${CYAN}[1/3] Memory Management Suite${NC}"
            if run_suite "Memory Management" "$MEMORY_SUITE" "$mode" "$verbose"; then
                ((passed_suites++))
            else
                failed_suites+=("Memory Management")
            fi
            echo
            
            # Interrupt Management Suite
            echo -e "${CYAN}[2/3] Interrupt Management Suite${NC}"
            if run_suite "Interrupt Management" "$INTERRUPT_SUITE" "$mode" "$verbose"; then
                ((passed_suites++))
            else
                failed_suites+=("Interrupt Management")
            fi
            echo
            
            # Hardware/Driver Suite
            echo -e "${CYAN}[3/3] Hardware/Driver Suite${NC}"
            if run_suite "Hardware/Driver" "$HARDWARE_SUITE" "$mode" "$verbose"; then
                ((passed_suites++))
            else
                failed_suites+=("Hardware/Driver")
            fi
            ;;
    esac
    
    # Summary
    echo
    echo -e "${PURPLE}=== Test Suite Summary ===${NC}"
    echo -e "${GREEN}Passed: $passed_suites/$total_suites${NC}"
    
    if [[ ${#failed_suites[@]} -gt 0 ]]; then
        echo -e "${RED}Failed: ${#failed_suites[@]}/$total_suites${NC}"
        echo -e "${RED}Failed suites:${NC}"
        for suite in "${failed_suites[@]}"; do
            echo -e "  ${RED}✗ $suite${NC}"
        done
        exit 1
    else
        echo -e "${GREEN}All test suites passed successfully!${NC}"
        exit 0
    fi
}

# Run main function with all arguments
main "$@"
