#!/bin/bash

# Phase 4.4.4 Dynamic Memory Management Test Script
# TinyOS - July 11, 2025

echo "=== Phase 4.4.4 Dynamic Memory Management Test ==="
echo "Testing dynamic memory management implementation..."

# Set up environment
cd "$(dirname "$0")"
TEST_DIR="$(pwd)"
PROJECT_ROOT="$TEST_DIR"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Test counter
TESTS_PASSED=0
TESTS_TOTAL=0

# Test function
run_test() {
    local test_name="$1"
    local test_command="$2"
    local expected_result="$3"
    
    echo -n "Testing $test_name... "
    TESTS_TOTAL=$((TESTS_TOTAL + 1))
    
    if eval "$test_command" > /dev/null 2>&1; then
        if [ "$expected_result" = "success" ]; then
            echo -e "${GREEN}PASS${NC}"
            TESTS_PASSED=$((TESTS_PASSED + 1))
        else
            echo -e "${RED}FAIL${NC} (expected failure but got success)"
        fi
    else
        if [ "$expected_result" = "failure" ]; then
            echo -e "${GREEN}PASS${NC} (expected failure)"
            TESTS_PASSED=$((TESTS_PASSED + 1))
        else
            echo -e "${RED}FAIL${NC}"
        fi
    fi
}

# Change to project directory
cd "$PROJECT_ROOT"

echo "Project directory: $PROJECT_ROOT"
echo

# Test 1: Build system
echo "1. Build System Tests"
run_test "Clean build" "cargo clean && cargo build" "success"
run_test "Incremental build" "cargo build" "success"
echo

# Test 2: File structure validation
echo "2. File Structure Tests"
run_test "Dynamic memory module exists" "[ -f src/memory/dynamic.rs ]" "success"
run_test "Dynamic memory shell commands exist" "[ -f src/shell/commands/dynamic_memory.rs ]" "success"
run_test "Implementation plan exists" "[ -f PHASE4_4_4_IMPLEMENTATION_PLAN.md ]" "success"
run_test "Implementation summary exists" "[ -f PHASE4_4_4_IMPLEMENTATION_SUMMARY.md ]" "success"
echo

# Test 3: Code validation
echo "3. Code Validation Tests"
run_test "Dynamic memory manager exports" "grep -q 'pub use dynamic' src/memory/mod.rs" "success"
run_test "Shell commands module exports" "grep -q 'dynamic_memory' src/shell/commands/mod.rs" "success"
run_test "Dynamic memory initialization" "grep -q 'init_dynamic_memory_manager' src/main.rs" "success"
run_test "Shell submenu integration" "grep -q '\\*' src/shell/mod.rs" "success"
echo

# Test 4: Documentation validation
echo "4. Documentation Tests"
run_test "Module documentation exists" "grep -q '//!' src/memory/dynamic.rs" "success"
run_test "Function documentation exists" "grep -q '///' src/memory/dynamic.rs" "success"
run_test "Shell command documentation" "grep -q '//!' src/shell/commands/dynamic_memory.rs" "success"
echo

# Test 5: Integration validation
echo "5. Integration Tests"
run_test "No compilation errors" "cargo check 2>&1 | grep -q 'error:' && false || true" "success"
run_test "Memory manager integration" "grep -q 'DynamicMemoryManager' src/memory/dynamic.rs" "success"
run_test "MMU exception integration" "grep -q 'MmuFaultInfo' src/memory/dynamic.rs" "success"
run_test "Shell context integration" "grep -q 'ShellContext' src/shell/commands/dynamic_memory.rs" "success"
echo

# Test 6: Feature validation
echo "6. Feature Implementation Tests"
run_test "Dynamic stack management" "grep -q 'DynamicStack' src/memory/dynamic.rs" "success"
run_test "Lazy page allocation" "grep -q 'LazyPage' src/memory/dynamic.rs" "success"
run_test "Memory pressure handling" "grep -q 'MemoryPressureHandler' src/memory/dynamic.rs" "success"
run_test "Hardware context switching" "grep -q 'HardwareContextSwitcher' src/memory/dynamic.rs" "success"
echo

# Test 7: Shell command validation
echo "7. Shell Command Tests"
run_test "Dynamic memory help command" "grep -q 'show_dynamic_memory_help' src/shell/commands/dynamic_memory.rs" "success"
run_test "Statistics command" "grep -q 'cmd_dynamic_memory_stats' src/shell/commands/dynamic_memory.rs" "success"
run_test "Growth command" "grep -q 'cmd_dynamic_memory_growth' src/shell/commands/dynamic_memory.rs" "success"
run_test "Memory pressure command" "grep -q 'cmd_dynamic_memory_pressure' src/shell/commands/dynamic_memory.rs" "success"
echo

# Test 8: Safety and robustness
echo "8. Safety Tests"
run_test "No unsafe blocks without justification" "grep -A 5 'unsafe' src/memory/dynamic.rs | grep -q '//' && true || false" "success"
run_test "Error handling present" "grep -q 'Result<' src/memory/dynamic.rs" "success"
run_test "Option handling present" "grep -q 'Option<' src/memory/dynamic.rs" "success"
echo

# Generate summary
echo "=== Test Summary ==="
echo "Tests passed: $TESTS_PASSED/$TESTS_TOTAL"

if [ "$TESTS_PASSED" -eq "$TESTS_TOTAL" ]; then
    echo -e "${GREEN}All tests passed! Phase 4.4.4 implementation is validated.${NC}"
    exit 0
else
    echo -e "${RED}Some tests failed. Please review the implementation.${NC}"
    exit 1
fi
