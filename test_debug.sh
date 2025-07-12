#!/bin/bash

echo "=== Testing TinyOS Test Script Fix ==="
echo

# Test if the scripts exist in the correct location
TESTS_DIR="/home/byoboo/projects/Tinyland/tiny_os/tests/scripts"

echo "Checking test scripts in: $TESTS_DIR"
echo

if [ -d "$TESTS_DIR" ]; then
    echo "✓ Tests directory exists"
    echo "Available test scripts:"
    ls -1 "$TESTS_DIR"/*.sh | head -5
else
    echo "✗ Tests directory not found"
fi

echo
echo "=== Testing main script ==="
echo "Running: bash test_tinyos.sh --list"

# Try to run the main script
if bash test_tinyos.sh --list; then
    echo "✓ Script executed successfully"
else
    echo "✗ Script failed with exit code $?"
fi
