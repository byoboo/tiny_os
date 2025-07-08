#!/usr/bin/env bash

# Shell Test Runner for TinyOS Phase 1
# Validates the refactored shell system functionality

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

echo "🧪 TinyOS Phase 1 Shell Test Suite"
echo "=================================="
echo "Testing refactored shell command system..."
echo

# Function to run tests with proper error handling
run_test_suite() {
    local test_name="$1"
    local test_command="$2"
    
    echo "📋 Running $test_name..."
    if eval "$test_command"; then
        echo "✅ $test_name: PASSED"
        return 0
    else
        echo "❌ $test_name: FAILED"
        return 1
    fi
}

# Change to project root
cd "$PROJECT_ROOT"

echo "🔧 Verifying project builds..."
if ! cargo check --quiet; then
    echo "❌ Project build check failed"
    exit 1
fi
echo "✅ Project builds successfully"
echo

# Initialize test results tracking
TOTAL_TESTS=0
PASSED_TESTS=0
FAILED_TESTS=()

# Test 1: Unit Tests for Shell Commands
TOTAL_TESTS=$((TOTAL_TESTS + 1))
if run_test_suite "Shell Unit Tests" "cargo test --test shell_tests --quiet"; then
    PASSED_TESTS=$((PASSED_TESTS + 1))
else
    FAILED_TESTS+=("Shell Unit Tests")
fi
echo

# Test 2: Integration Tests for Shell System
TOTAL_TESTS=$((TOTAL_TESTS + 1))
if run_test_suite "Shell Integration Tests" "cargo test --test shell_integration_tests --quiet"; then
    PASSED_TESTS=$((PASSED_TESTS + 1))
else
    FAILED_TESTS+=("Shell Integration Tests")
fi
echo

# Test 3: Compilation Tests (verify all modules compile)
TOTAL_TESTS=$((TOTAL_TESTS + 1))
if run_test_suite "Module Compilation Test" "cargo build --quiet"; then
    PASSED_TESTS=$((PASSED_TESTS + 1))
else
    FAILED_TESTS+=("Module Compilation Test")
fi
echo

# Test 4: Library Tests (existing tests still pass)
TOTAL_TESTS=$((TOTAL_TESTS + 1))
if run_test_suite "Existing Library Tests" "cargo test --lib --quiet"; then
    PASSED_TESTS=$((PASSED_TESTS + 1))
else
    FAILED_TESTS+=("Existing Library Tests")
fi
echo

# Test 5: Documentation Tests
TOTAL_TESTS=$((TOTAL_TESTS + 1))
if run_test_suite "Documentation Tests" "cargo test --doc --quiet"; then
    PASSED_TESTS=$((PASSED_TESTS + 1))
else
    FAILED_TESTS+=("Documentation Tests")
fi
echo

# Test 6: Module Structure Validation
TOTAL_TESTS=$((TOTAL_TESTS + 1))
echo "📋 Running Module Structure Validation..."
STRUCTURE_VALID=true

# Check required shell module files exist
REQUIRED_FILES=(
    "src/shell/mod.rs"
    "src/shell/commands/mod.rs"
    "src/shell/commands/system.rs"
    "src/shell/commands/hardware.rs"
    "src/shell/commands/memory.rs"
    "src/shell/commands/filesystem.rs"
)

for file in "${REQUIRED_FILES[@]}"; do
    if [[ ! -f "$file" ]]; then
        echo "❌ Missing required file: $file"
        STRUCTURE_VALID=false
    fi
done

# Check that no test files exist in src/ (they should be in tests/)
if [[ -d "src/shell/tests" ]]; then
    echo "❌ Test files found in src/shell/tests (should be in tests/ directory)"
    STRUCTURE_VALID=false
fi

# Check that main.rs is properly cleaned up (should be much smaller now)
MAIN_RS_LINES=$(wc -l < src/main.rs)
if [[ $MAIN_RS_LINES -gt 150 ]]; then
    echo "⚠️  main.rs has $MAIN_RS_LINES lines (expected < 150 after refactor)"
fi

if $STRUCTURE_VALID; then
    echo "✅ Module Structure Validation: PASSED"
    PASSED_TESTS=$((PASSED_TESTS + 1))
else
    echo "❌ Module Structure Validation: FAILED"
    FAILED_TESTS+=("Module Structure Validation")
fi
echo

# Test 7: Command Handler Coverage Test
TOTAL_TESTS=$((TOTAL_TESTS + 1))
echo "📋 Running Command Handler Coverage Test..."

# Verify all expected command handlers exist by checking exports
COVERAGE_VALID=true

# Check system commands
if ! grep -q "pub fn handle_help" src/shell/commands/system.rs; then
    echo "❌ Missing handle_help in system commands"
    COVERAGE_VALID=false
fi

if ! grep -q "pub fn handle_time" src/shell/commands/system.rs; then
    echo "❌ Missing handle_time in system commands"
    COVERAGE_VALID=false
fi

# Check hardware commands  
if ! grep -q "pub fn handle_led_on" src/shell/commands/hardware.rs; then
    echo "❌ Missing handle_led_on in hardware commands"
    COVERAGE_VALID=false
fi

# Check memory commands
if ! grep -q "pub fn handle_memory_stats" src/shell/commands/memory.rs; then
    echo "❌ Missing handle_memory_stats in memory commands"
    COVERAGE_VALID=false
fi

# Check filesystem commands
if ! grep -q "pub fn handle_directory_listing" src/shell/commands/filesystem.rs; then
    echo "❌ Missing handle_directory_listing in filesystem commands"
    COVERAGE_VALID=false
fi

if $COVERAGE_VALID; then
    echo "✅ Command Handler Coverage: PASSED"
    PASSED_TESTS=$((PASSED_TESTS + 1))
else
    echo "❌ Command Handler Coverage: FAILED"
    FAILED_TESTS+=("Command Handler Coverage")
fi
echo

# Print final results
echo "📊 Test Results Summary"
echo "======================"
echo "Total Tests: $TOTAL_TESTS"
echo "Passed: $PASSED_TESTS"
echo "Failed: $((TOTAL_TESTS - PASSED_TESTS))"
echo

if [[ ${#FAILED_TESTS[@]} -eq 0 ]]; then
    echo "🎉 All tests passed! Phase 1 refactor is successful."
    echo
    echo "✅ Shell system has been successfully refactored"
    echo "✅ All original functionality is preserved"
    echo "✅ Code is now modular and maintainable"
    echo "✅ Ready for Phase 2 (Driver Organization)"
    exit 0
else
    echo "❌ Some tests failed:"
    for test in "${FAILED_TESTS[@]}"; do
        echo "   - $test"
    done
    echo
    echo "Please fix the failing tests before proceeding to Phase 2."
    exit 1
fi
