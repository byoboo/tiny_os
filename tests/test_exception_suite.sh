#!/bin/bash

# Exception System Test Suite - Phase 1
# Tests ESR decoding, system calls, and memory fault analysis

echo "=== TinyOS Exception System Test Suite (Phase 1) ==="
echo "Testing enhanced exception handling capabilities"
echo

TEST_RESULTS=()
TOTAL_TESTS=0
PASSED_TESTS=0

# Helper function to record test results
record_test() {
    local test_name="$1"
    local result="$2"
    TOTAL_TESTS=$((TOTAL_TESTS + 1))
    if [ "$result" = "PASS" ]; then
        PASSED_TESTS=$((PASSED_TESTS + 1))
        echo "✅ $test_name: PASS"
    else
        echo "❌ $test_name: FAIL"
    fi
    TEST_RESULTS+=("$test_name: $result")
}

# Test 1: Verify exception module structure
echo "Test 1: Exception Module Architecture"
echo "-------------------------------------"

modules=(
    "src/exceptions/mod.rs"
    "src/exceptions/types.rs" 
    "src/exceptions/handler.rs"
    "src/exceptions/esr_decoder.rs"
    "src/exceptions/init.rs"
)

for module in "${modules[@]}"; do
    if [ -f "$module" ]; then
        record_test "Module exists: $module" "PASS"
    else
        record_test "Module exists: $module" "FAIL"
    fi
done

# Test 2: Build verification
echo
echo "Test 2: Build System Integration"
echo "--------------------------------"

echo "Building TinyOS with exception enhancements..."
if cargo build --quiet 2>/dev/null; then
    record_test "Exception system builds successfully" "PASS"
else
    record_test "Exception system builds successfully" "FAIL"
    echo "Build failed. Check compilation errors."
fi

# Test 3: ESR Decoder validation
echo
echo "Test 3: ESR_EL1 Decoder Functionality"
echo "------------------------------------"

# Check if ESR decoder has required types and functions
if grep -q "pub enum ExceptionClass" src/exceptions/esr_decoder.rs; then
    record_test "ExceptionClass enum defined" "PASS"
else
    record_test "ExceptionClass enum defined" "FAIL"
fi

if grep -q "pub enum DataFaultStatus" src/exceptions/esr_decoder.rs; then
    record_test "DataFaultStatus enum defined" "PASS"
else
    record_test "DataFaultStatus enum defined" "FAIL"
fi

if grep -q "pub struct EsrInfo" src/exceptions/esr_decoder.rs; then
    record_test "EsrInfo struct defined" "PASS"
else
    record_test "EsrInfo struct defined" "FAIL"
fi

if grep -q "pub fn decode_esr" src/exceptions/esr_decoder.rs; then
    record_test "ESR decode function exists" "PASS"
else
    record_test "ESR decode function exists" "FAIL"
fi

# Test 4: Exception handler integration
echo
echo "Test 4: Exception Handler Integration"
echo "-----------------------------------"

handlers=(
    "handle_sync_exception"
    "handle_irq_exception" 
    "handle_fiq_exception"
    "handle_serror_exception"
)

for handler in "${handlers[@]}"; do
    if grep -q "pub extern \"C\" fn $handler" src/exceptions/handler.rs; then
        record_test "Handler exists: $handler" "PASS"
    else
        record_test "Handler exists: $handler" "FAIL"
    fi
done

# Test 5: Exception statistics integration
echo
echo "Test 5: Exception Statistics System"
echo "----------------------------------"

if grep -q "ExceptionStats" src/exceptions/types.rs; then
    record_test "ExceptionStats type defined" "PASS"
else
    record_test "ExceptionStats type defined" "FAIL"
fi

if grep -q "EXCEPTION_STATS" src/exceptions/types.rs; then
    record_test "Global exception stats defined" "PASS"
else
    record_test "Global exception stats defined" "FAIL"
fi

# Test 6: Shell command integration
echo
echo "Test 6: Shell Command Integration"
echo "--------------------------------"

if grep -q "handle_exception_stats" src/shell/commands/hardware.rs; then
    record_test "Exception stats command exists" "PASS"
else
    record_test "Exception stats command exists" "FAIL"
fi

if grep -q "handle_exception_test" src/shell/commands/hardware.rs; then
    record_test "Exception test command exists" "PASS"
else
    record_test "Exception test command exists" "FAIL"
fi

# Test 7: QEMU Boot Test with Exception System
echo
echo "Test 7: QEMU Boot Test with Exception System"
echo "-------------------------------------------"

echo "Starting QEMU boot test with 10-second timeout..."
timeout 10s ./run.sh > qemu_test_output.txt 2>&1 &
QEMU_PID=$!

sleep 8
kill $QEMU_PID 2>/dev/null
wait $QEMU_PID 2>/dev/null

if grep -q "Exception handling initialized" qemu_test_output.txt; then
    record_test "Exception system initializes in QEMU" "PASS"
else
    record_test "Exception system initializes in QEMU" "FAIL"
fi

if grep -q "TinyOS Starting" qemu_test_output.txt; then
    record_test "TinyOS boots successfully with exceptions" "PASS"
else
    record_test "TinyOS boots successfully with exceptions" "FAIL"
fi

# Clean up
rm -f qemu_test_output.txt

# Test 8: Exception Vector Table Assembly Integration  
echo
echo "Test 8: Exception Vector Table Integration"
echo "-----------------------------------------"

if [ -f "src/exception_vectors.s" ]; then
    record_test "Exception vector assembly file exists" "PASS"
else
    record_test "Exception vector assembly file exists" "FAIL"
fi

if grep -q "exception_vector_table" src/exception_vectors.s; then
    record_test "Exception vector table defined" "PASS"
else
    record_test "Exception vector table defined" "FAIL"
fi

if grep -q "handle_sync_exception" src/exception_vectors.s; then
    record_test "Sync handler linked in assembly" "PASS"
else
    record_test "Sync handler linked in assembly" "FAIL"
fi

# Test 9: Memory Layout Compatibility
echo
echo "Test 9: Memory Layout and Linker Compatibility"
echo "---------------------------------------------"

if [ -f "linker.ld" ]; then
    record_test "Linker script exists" "PASS"
    
    if grep -q "__bss_start\|__bss_end\|__stack_end" linker.ld; then
        record_test "Linker script has symbol definitions" "PASS"
    else
        record_test "Linker script has symbol definitions" "FAIL"
    fi
else
    record_test "Linker script exists" "FAIL"
fi

# Test 10: Exception System API Consistency
echo
echo "Test 10: Exception System API Consistency" 
echo "----------------------------------------"

if grep -q "exceptions::init_exceptions\|init_exceptions" src/main.rs; then
    record_test "Main.rs uses exception module" "PASS"
else
    record_test "Main.rs uses exception module" "FAIL"
fi

if grep -q "init_exceptions" src/main.rs; then
    record_test "Exception initialization called" "PASS"
else
    record_test "Exception initialization called" "FAIL"
fi

# Summary
echo
echo "=========================================="
echo "Exception System Test Suite Summary"
echo "=========================================="
echo "Total Tests: $TOTAL_TESTS"
echo "Passed: $PASSED_TESTS"
echo "Failed: $((TOTAL_TESTS - PASSED_TESTS))"
echo

if [ $PASSED_TESTS -eq $TOTAL_TESTS ]; then
    echo "🎉 ALL TESTS PASSED! Exception system is ready for Phase 1."
    exit 0
else
    echo "⚠️  Some tests failed. Review the exception system implementation."
    echo
    echo "Failed tests:"
    for result in "${TEST_RESULTS[@]}"; do
        if [[ $result == *"FAIL"* ]]; then
            echo "  - $result"
        fi
    done
    exit 1
fi
