#!/bin/bash

# Test Script Fix Verification
echo "=== TinyOS Test Script Fix Verification ==="
echo

# Check if test scripts exist in correct location
echo "Checking test scripts in tests/scripts/..."
if [ -d "tests/scripts" ]; then
    echo "✓ tests/scripts directory exists"
    echo "✓ Found $(ls tests/scripts/test_*.sh | wc -l) test scripts"
    echo
    echo "Sample test scripts:"
    ls tests/scripts/test_*.sh | head -5
else
    echo "✗ tests/scripts directory not found"
fi

echo
echo "=== Testing fixed script ==="
echo "Running: ./test_tinyos.sh --list"
echo

# Test the main script functionality
./test_tinyos.sh --list

echo
echo "=== Testing memory feature ==="
echo "Running: ./test_tinyos.sh memory --verbose"
echo

# Test running a specific feature
./test_tinyos.sh memory --verbose
