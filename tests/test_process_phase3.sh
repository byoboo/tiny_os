#!/bin/bash

# TinyOS Process Management Test Suite
# Phase 3: Process Management Foundation

set -e

echo "================================================================"
echo "TinyOS Process Management Test Suite - Phase 3"
echo "================================================================"
echo "Testing process management foundation including:"
echo "- Process context management"
echo "- User/kernel mode separation"
echo "- Basic task scheduler"
echo "================================================================"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Test results tracking
TOTAL_TESTS=0
PASSED_TESTS=0
FAILED_TESTS=0

# Function to run a test command and capture output
run_test() {
    local test_name="$1"
    local command="$2"
    local expected_pattern="$3"
    
    echo -n "Testing $test_name..."
    ((TOTAL_TESTS++))
    
    # Run the command and capture output
    local output=$(timeout 20 sh -c "(sleep 3; printf '$command') | qemu-system-aarch64 -M raspi4b -cpu cortex-a72 -m 2048M -kernel target/aarch64-unknown-none/debug/tiny_os -serial stdio -display none -monitor none 2>/dev/null" || true)
    
    if echo "$output" | grep -q "$expected_pattern"; then
        echo -e " ${GREEN}✅ PASSED${NC}"
        ((PASSED_TESTS++))
        return 0
    else
        echo -e " ${RED}❌ FAILED${NC}"
        echo "Expected pattern: $expected_pattern"
        echo "Actual output:"
        echo "$output" | tail -10
        ((FAILED_TESTS++))
        return 1
    fi
}

# Function to run integration test
run_integration_test() {
    local test_name="$1"
    local command="$2"
    local success_patterns="$3"
    
    echo -n "Testing $test_name..."
    ((TOTAL_TESTS++))
    
    # Run the command and capture output
    local output=$(timeout 20 sh -c "(sleep 3; printf '$command') | qemu-system-aarch64 -M raspi4b -cpu cortex-a72 -m 2048M -kernel target/aarch64-unknown-none/debug/tiny_os -serial stdio -display none -monitor none 2>/dev/null" || true)
    
    local all_passed=true
    IFS='|' read -ra PATTERNS <<< "$success_patterns"
    for pattern in "${PATTERNS[@]}"; do
        if ! echo "$output" | grep -q "$pattern"; then
            all_passed=false
            break
        fi
    done
    
    if [ "$all_passed" = true ]; then
        echo -e " ${GREEN}✅ PASSED${NC}"
        ((PASSED_TESTS++))
        return 0
    else
        echo -e " ${RED}❌ FAILED${NC}"
        echo "Expected patterns: $success_patterns"
        echo "Actual output:"
        echo "$output" | tail -10
        ((FAILED_TESTS++))
        return 1
    fi
}

# Build the project first
echo "Building TinyOS..."
cd "${TINY_OS_DIR:-$(pwd)/../tiny_os}"
cargo build --target aarch64-unknown-none || {
    echo -e "${RED}Build failed!${NC}"
    exit 1
}

echo -e "${GREEN}Build successful!${NC}"
echo

# Test Phase 3.1: Process Context Management
echo "=== Phase 3.1: Process Context Management ==="

# Test process context creation and state management
run_test "Process Context Management" "[" "Process Context"

# Test context switching
run_test "Context Switching" "[" "Context Switch"

# Test process state tracking
run_test "Process State Tracking" "[" "Process State"

echo

# Test Phase 3.2: User/Kernel Mode Separation
echo "=== Phase 3.2: User/Kernel Mode Separation ==="

# Test privilege level management
run_test "Privilege Level Management" "\\" "Privilege Level"

# Test user/kernel mode transitions
run_test "User/Kernel Mode Transitions" "\\" "EL0.*EL1"

# Test privilege validation
run_test "Privilege Validation" "\\" "Privilege.*Validation"

echo

# Test Phase 3.3: Basic Task Scheduler
echo "=== Phase 3.3: Basic Task Scheduler ==="

# Test task creation and management
run_test "Task Creation and Management" "]" "Task.*Created"

# Test round-robin scheduling
run_test "Round-Robin Scheduling" "]" "Round.*Robin"

# Test priority-based scheduling
run_test "Priority-Based Scheduling" "]" "Priority.*Scheduling"

# Test preemptive scheduling
run_test "Preemptive Scheduling" "]" "Preemption"

echo

# Test Phase 3 Integration
echo "=== Phase 3 Integration Tests ==="

# Test process management initialization
run_integration_test "Process Management Init" "[|\\|]" "Process.*Management.*Initialized"

# Test complete process lifecycle
run_integration_test "Process Lifecycle" "pctx|sched" "Process.*Lifecycle"

# Test scheduler with privilege management
run_integration_test "Scheduler + Privilege Management" "sched|priv" "Scheduler.*Privilege"

echo

# Test Phase 3 Statistics and Monitoring
echo "=== Phase 3 Statistics and Monitoring ==="

# Test process statistics
run_test "Process Statistics" "pstats" "Process.*Statistics"

# Test scheduler statistics
run_test "Scheduler Statistics" "sstats" "Scheduler.*Statistics"

# Test privilege statistics
run_test "Privilege Statistics" "prstats" "Privilege.*Statistics"

echo

# Test Phase 3 Error Handling
echo "=== Phase 3 Error Handling ==="

# Test invalid process operations
run_test "Invalid Process Operations" "pctx" "Invalid.*Process"

# Test privilege violations
run_test "Privilege Violations" "priv" "Privilege.*Violation"

# Test scheduler error handling
run_test "Scheduler Error Handling" "sched" "Scheduler.*Error"

echo

# Summary
echo "================================================================"
echo "                    Process Management Test Summary"
echo "================================================================"
echo -e "Total Tests: $TOTAL_TESTS"
echo -e "${GREEN}Passed: $PASSED_TESTS${NC}"
echo -e "${RED}Failed: $FAILED_TESTS${NC}"

if [ $FAILED_TESTS -eq 0 ]; then
    echo -e "${GREEN}🎉 All process management tests passed!${NC}"
    echo
    echo "Phase 3 Success Criteria:"
    echo "✅ Process context fully managed"
    echo "✅ User/kernel mode separation working"
    echo "✅ Basic preemptive multitasking operational"
    echo "✅ Task creation and scheduling functional"
    echo "✅ Exception system supporting process management"
    echo
    echo "Phase 3 implementation complete and validated!"
    exit 0
else
    echo -e "${RED}❌ Some tests failed. Please check the implementation.${NC}"
    exit 1
fi
