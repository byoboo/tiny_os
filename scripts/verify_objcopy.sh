#!/bin/bash

# Test script to verify objcopy tools work correctly
# This should be run in CI environments to debug objcopy issues

echo "=== OBJCOPY TOOL VERIFICATION ==="
echo

# Check Rust toolchain
echo "1. Rust toolchain information:"
rustc --print sysroot
echo

# Check for llvm-tools-preview component
echo "2. Checking llvm-tools-preview component:"
rustup component list | grep llvm-tools || echo "llvm-tools-preview not found"
echo

# Ensure it's installed
echo "3. Installing llvm-tools-preview component:"
rustup component add llvm-tools-preview
echo

# Find LLVM objcopy tools
echo "4. Searching for LLVM objcopy tools:"
RUST_TOOLCHAIN=$(rustc --print sysroot)
echo "Rust toolchain: $RUST_TOOLCHAIN"

find "$RUST_TOOLCHAIN/lib/rustlib" -name "llvm-objcopy" -type f 2>/dev/null | while read -r path; do
    echo "  Found: $path"
    if [ -x "$path" ]; then
        echo "    Executable: YES"
        "$path" --version 2>/dev/null || echo "    Version check failed"
    else
        echo "    Executable: NO"
    fi
done

echo

# Check system-wide tools
echo "5. Checking system-wide LLVM tools:"
for tool in llvm-objcopy objcopy rust-objcopy; do
    if command -v "$tool" >/dev/null 2>&1; then
        echo "  ✓ $tool: $(which $tool)"
        "$tool" --version 2>/dev/null || echo "    Version check failed"
    else
        echo "  ✗ $tool: not found"
    fi
done

echo

# Test with a simple binary
echo "6. Testing objcopy with a simple binary:"
echo 'int main() { return 0; }' > test.c

if command -v gcc >/dev/null 2>&1; then
    gcc -o test test.c
    if [ -f test ]; then
        echo "  Created test binary"
        
        # Try different objcopy tools
        for tool in llvm-objcopy objcopy; do
            if command -v "$tool" >/dev/null 2>&1; then
                echo "  Testing $tool:"
                if "$tool" --help >/dev/null 2>&1; then
                    echo "    Help works: YES"
                    if "$tool" -O binary test test.bin 2>/dev/null; then
                        echo "    Binary extraction: YES"
                        rm -f test.bin
                    else
                        echo "    Binary extraction: FAILED"
                    fi
                else
                    echo "    Help works: NO"
                fi
            fi
        done
    else
        echo "  Could not create test binary"
    fi
else
    echo "  No gcc available for testing"
fi

# Cleanup
rm -f test test.c test.bin

echo
echo "=== VERIFICATION COMPLETE ==="
