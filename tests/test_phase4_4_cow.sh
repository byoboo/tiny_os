#!/bin/bash

# Phase 4.4 COW Management Testing Script
# Tests the Copy-on-Write (COW) implementation in QEMU

echo "=========================================="
echo "Phase 4.4 COW Management Testing"
echo "=========================================="

# Change to the correct directory
cd /home/byoboo/projects/Tinyland/tiny_os

# Build the project
echo "Building TinyOS..."
cargo build --release

if [ $? -ne 0 ]; then
    echo "Build failed!"
    exit 1
fi

# Copy the kernel image
echo "Copying kernel image..."
cp target/aarch64-unknown-none/release/tiny_os kernel8.img

# Create a test configuration
echo "Creating test configuration..."

# Test script for COW management
TEST_SCRIPT="
# Phase 4.4 COW Management Test Script
# COW Status
~1
# COW Statistics
~2
# COW Test Suite
~6
# Create COW Mapping
~3
# COW Status after mapping
~1
# Force COW Protection
~4
# COW Statistics after protection
~2
# Remove COW Protection
~5
# Final COW Status
~1
# System Health Check
c
# Exit
"

# Create temporary test file
echo "$TEST_SCRIPT" > test_input.txt

echo "Starting QEMU test..."
timeout 60 qemu-system-aarch64 \
    -machine raspi3b \
    -cpu cortex-a72 \
    -kernel kernel8.img \
    -serial stdio \
    -nographic \
    -monitor none \
    -d guest_errors \
    < test_input.txt > test_output.txt 2>&1

# Check if QEMU ran successfully
if [ $? -eq 124 ]; then
    echo "QEMU test completed (timeout reached)"
else
    echo "QEMU test finished"
fi

# Analyze test output
echo "Analyzing test results..."

# Check for COW-related output
if grep -q "COW Status" test_output.txt; then
    echo "✓ COW Status command executed"
else
    echo "✗ COW Status command not found"
fi

if grep -q "COW Statistics" test_output.txt; then
    echo "✓ COW Statistics command executed"
else
    echo "✗ COW Statistics command not found"
fi

if grep -q "COW Test Suite" test_output.txt; then
    echo "✓ COW Test Suite command executed"
else
    echo "✗ COW Test Suite command not found"
fi

if grep -q "COW manager" test_output.txt; then
    echo "✓ COW manager functionality detected"
else
    echo "✗ COW manager functionality not detected"
fi

# Check for successful boot
if grep -q "TinyOS Ready" test_output.txt; then
    echo "✓ TinyOS booted successfully"
else
    echo "✗ TinyOS boot failed"
fi

# Check for shell functionality
if grep -q "Command Reference" test_output.txt; then
    echo "✓ Shell help system working"
else
    echo "✗ Shell help system not working"
fi

# Check for COW commands in help
if grep -q "COW Management" test_output.txt; then
    echo "✓ COW commands listed in help"
else
    echo "✗ COW commands not listed in help"
fi

# Check for test passes/fails
PASSED_TESTS=$(grep -c "PASS" test_output.txt)
FAILED_TESTS=$(grep -c "FAIL" test_output.txt)

if [ $PASSED_TESTS -gt 0 ]; then
    echo "✓ COW tests passed: $PASSED_TESTS"
else
    echo "✗ No COW tests passed"
fi

if [ $FAILED_TESTS -gt 0 ]; then
    echo "⚠ COW tests failed: $FAILED_TESTS"
fi

# Display summary
echo ""
echo "=========================================="
echo "Test Summary:"
echo "=========================================="
echo "Total output lines: $(wc -l < test_output.txt)"
echo "COW Status checks: $(grep -c "COW Status" test_output.txt)"
echo "COW Statistics checks: $(grep -c "COW Statistics" test_output.txt)"
echo "COW Test executions: $(grep -c "COW Test" test_output.txt)"
echo "Test passes: $PASSED_TESTS"
echo "Test failures: $FAILED_TESTS"

# Show key test output
echo ""
echo "=========================================="
echo "Key Test Output:"
echo "=========================================="
echo "Boot messages:"
grep -A 5 "TinyOS Ready" test_output.txt || echo "No boot messages found"

echo ""
echo "COW Status output:"
grep -A 10 "COW Status" test_output.txt || echo "No COW status output found"

echo ""
echo "COW Test results:"
grep -B 2 -A 5 "Test Summary" test_output.txt || echo "No COW test summary found"

# Clean up
rm -f test_input.txt

echo ""
echo "=========================================="
echo "Phase 4.4 COW Test Complete"
echo "=========================================="
echo "Test output saved to: test_output.txt"
echo "Review the output for detailed results"
echo "=========================================="
