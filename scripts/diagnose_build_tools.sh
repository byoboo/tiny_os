#!/bin/bash

# TinyOS Build Tool Diagnostic Script
# Helps diagnose and fix objcopy tool issues

echo "=== TinyOS Build Tool Diagnostic ==="
echo

# Check Rust installation
echo "1. Checking Rust installation..."
if command -v rustc >/dev/null 2>&1; then
    echo "✅ Rust is installed: $(rustc --version)"
    RUST_SYSROOT=$(rustc --print sysroot)
    echo "   Rust sysroot: $RUST_SYSROOT"
else
    echo "❌ Rust is not installed"
    echo "   Please install Rust: https://rustup.rs/"
    exit 1
fi

echo

# Check rustup
echo "2. Checking rustup..."
if command -v rustup >/dev/null 2>&1; then
    echo "✅ rustup is available: $(rustup --version | head -1)"
    echo "   Active toolchain: $(rustup show active-toolchain)"
else
    echo "❌ rustup is not available"
    exit 1
fi

echo

# Check llvm-tools-preview component
echo "3. Checking llvm-tools-preview component..."
if rustup component list --installed | grep -q "llvm-tools-preview"; then
    echo "✅ llvm-tools-preview is installed"
else
    echo "⚠️  llvm-tools-preview is not installed"
    echo "   Installing now..."
    rustup component add llvm-tools-preview
    if [ $? -eq 0 ]; then
        echo "✅ llvm-tools-preview installed successfully"
    else
        echo "❌ Failed to install llvm-tools-preview"
        exit 1
    fi
fi

echo

# Check for objcopy tools
echo "4. Checking for objcopy tools..."

# Check rust-objcopy
if command -v rust-objcopy >/dev/null 2>&1; then
    echo "✅ rust-objcopy is available: $(which rust-objcopy)"
else
    echo "⚠️  rust-objcopy is not in PATH"
fi

# Check llvm-objcopy
if command -v llvm-objcopy >/dev/null 2>&1; then
    echo "✅ llvm-objcopy is available: $(which llvm-objcopy)"
else
    echo "⚠️  llvm-objcopy is not in PATH"
fi

# Check system objcopy
if command -v objcopy >/dev/null 2>&1; then
    echo "✅ objcopy is available: $(which objcopy)"
else
    echo "⚠️  objcopy is not in PATH"
fi

echo

# Check in Rust toolchain directory
echo "5. Checking Rust toolchain directory for objcopy..."
RUST_TOOLCHAIN=$(rustc --print sysroot)

# Check different possible locations
OBJCOPY_LOCATIONS=(
    "$RUST_TOOLCHAIN/lib/rustlib/x86_64-apple-darwin/bin/llvm-objcopy"
    "$RUST_TOOLCHAIN/lib/rustlib/aarch64-apple-darwin/bin/llvm-objcopy"
    "$RUST_TOOLCHAIN/lib/rustlib/x86_64-unknown-linux-gnu/bin/llvm-objcopy"
    "$RUST_TOOLCHAIN/lib/rustlib/aarch64-unknown-linux-gnu/bin/llvm-objcopy"
)

FOUND_OBJCOPY=""
for location in "${OBJCOPY_LOCATIONS[@]}"; do
    if [ -f "$location" ]; then
        echo "✅ Found objcopy at: $location"
        FOUND_OBJCOPY="$location"
        break
    fi
done

if [ -z "$FOUND_OBJCOPY" ]; then
    echo "❌ No objcopy found in Rust toolchain directory"
    echo "   Searched locations:"
    for location in "${OBJCOPY_LOCATIONS[@]}"; do
        echo "     $location"
    done
else
    echo "✅ objcopy is available in toolchain"
fi

echo

# Test objcopy functionality
echo "6. Testing objcopy functionality..."

# Create a simple test file
echo "Creating test file..."
echo -e "#include <stdio.h>\nint main() { return 0; }" > test.c

# Try to compile it (if we have a compiler)
if command -v clang >/dev/null 2>&1; then
    clang -o test test.c 2>/dev/null
    if [ -f test ]; then
        echo "✅ Test executable created"
        
        # Try to use objcopy on it
        if [ -n "$FOUND_OBJCOPY" ]; then
            "$FOUND_OBJCOPY" --help >/dev/null 2>&1
            if [ $? -eq 0 ]; then
                echo "✅ objcopy tool is functional"
            else
                echo "❌ objcopy tool is not working properly"
            fi
        fi
        
        # Clean up
        rm -f test test.c
    else
        echo "⚠️  Could not create test executable"
        rm -f test.c
    fi
else
    echo "⚠️  No C compiler available for testing"
    rm -f test.c
fi

echo

# Recommendations
echo "7. Recommendations..."

if [ -n "$FOUND_OBJCOPY" ]; then
    echo "✅ Your system should work with the updated build.sh script"
    echo "   The script will automatically find and use: $FOUND_OBJCOPY"
else
    echo "❌ objcopy tool not found. Try these solutions:"
    echo
    echo "   Option 1: Reinstall llvm-tools-preview"
    echo "     rustup component remove llvm-tools-preview"
    echo "     rustup component add llvm-tools-preview"
    echo
    echo "   Option 2: Install LLVM tools system-wide"
    echo "     macOS: brew install llvm"
    echo "     Ubuntu/Debian: sudo apt-get install llvm"
    echo
    echo "   Option 3: Use alternative toolchain"
    echo "     rustup toolchain install nightly"
    echo "     rustup component add llvm-tools-preview --toolchain nightly"
fi

echo
echo "=== Diagnostic Complete ==="
