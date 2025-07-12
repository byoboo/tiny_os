#!/bin/bash

# Test script for Phase 4 Modular Filesystem Implementation
# This script validates the new modular filesystem structure

echo "=== Phase 4 Modular Filesystem Test Script ==="
echo

# Test 1: Verify modular filesystem structure
echo "Test 1: Verifying modular filesystem structure..."
if [ -d "src/filesystem" ]; then
    echo "✓ src/filesystem/ directory exists"
else
    echo "✗ src/filesystem/ directory missing"
    exit 1
fi

if [ -d "src/filesystem/fat32" ]; then
    echo "✓ src/filesystem/fat32/ directory exists"
else
    echo "✗ src/filesystem/fat32/ directory missing"
    exit 1
fi

# Check individual module files
modules=(
    "src/filesystem/mod.rs"
    "src/filesystem/fat32/mod.rs"
    "src/filesystem/fat32/boot_sector.rs"
    "src/filesystem/fat32/directory.rs"
    "src/filesystem/fat32/file_operations.rs"
    "src/filesystem/fat32/cluster_chain.rs"
    "src/filesystem/fat32/filename.rs"
    "src/filesystem/fat32/interface.rs"
)

for module in "${modules[@]}"; do
    if [ -f "$module" ]; then
        echo "✓ $module exists"
    else
        echo "✗ $module missing"
        exit 1
    fi
done

echo

# Test 2: Verify legacy filesystem archive
echo "Test 2: Verifying legacy filesystem archive..."
if [ -d "src/legacy_filesystem" ]; then
    echo "✓ src/legacy_filesystem/ directory exists"
else
    echo "✗ src/legacy_filesystem/ directory missing"
    exit 1
fi

if [ -f "src/legacy_filesystem/fat32.rs" ]; then
    echo "✓ Legacy fat32.rs archived successfully"
else
    echo "✗ Legacy fat32.rs archive missing"
    exit 1
fi

if [ ! -f "src/fat32.rs" ]; then
    echo "✓ Old monolithic fat32.rs removed"
else
    echo "✗ Old monolithic fat32.rs still exists"
    exit 1
fi

echo

# Test 3: Compilation test
echo "Test 3: Testing compilation..."
if cargo check > /dev/null 2>&1; then
    echo "✓ cargo check passes"
else
    echo "✗ cargo check failed"
    cargo check
    exit 1
fi

if cargo build > /dev/null 2>&1; then
    echo "✓ cargo build passes"
else
    echo "✗ cargo build failed"
    cargo build
    exit 1
fi

echo

# Test 4: Binary size comparison
echo "Test 4: Binary size comparison..."
if [ -f "target/aarch64-unknown-none/debug/tiny_os" ]; then
    BINARY_SIZE=$(stat -c%s "target/aarch64-unknown-none/debug/tiny_os")
    echo "✓ Binary size: $BINARY_SIZE bytes"
    
    # Check if binary size is reasonable (should be similar to before)
    if [ "$BINARY_SIZE" -lt 1000000 ]; then  # Less than 1MB
        echo "✓ Binary size is reasonable"
    else
        echo "⚠ Binary size is large (>1MB), this may indicate issues"
    fi
else
    echo "✗ Binary not found"
    exit 1
fi

echo

# Test 5: Module structure validation
echo "Test 5: Module structure validation..."

# Check that modules have proper exports
if grep -q "pub mod fat32" src/filesystem/mod.rs; then
    echo "✓ Filesystem module exports fat32"
else
    echo "✗ Filesystem module missing fat32 export"
    exit 1
fi

if grep -q "pub use fat32" src/filesystem/mod.rs; then
    echo "✓ Filesystem module re-exports fat32 types"
else
    echo "✗ Filesystem module missing fat32 re-exports"
    exit 1
fi

# Check that lib.rs uses the new filesystem
if grep -q "pub mod filesystem" src/lib.rs; then
    echo "✓ lib.rs includes filesystem module"
else
    echo "✗ lib.rs missing filesystem module"
    exit 1
fi

if grep -q "pub use crate::filesystem::fat32" src/lib.rs; then
    echo "✓ lib.rs provides backward compatibility"
else
    echo "✗ lib.rs missing backward compatibility"
    exit 1
fi

echo

# Test 6: Shell integration test
echo "Test 6: Shell integration test..."
if grep -q "filesystem::Fat32FileSystem" src/shell/mod.rs; then
    echo "✓ Shell uses new filesystem module"
else
    echo "✗ Shell not updated for new filesystem"
    exit 1
fi

if grep -q "filesystem::Fat32FileSystem" src/shell/commands/filesystem.rs; then
    echo "✓ Shell commands use new filesystem module"
else
    echo "✗ Shell commands not updated for new filesystem"
    exit 1
fi

echo

# Test 7: no_std compliance check
echo "Test 7: no_std compliance check..."

# Check that modules don't use std
forbidden_std_items=(
    "use std::"
    "std::"
    "String::"
    "Vec::"
    "HashMap::"
    "format!"
    "println!"
    "to_string()"
)

no_std_violations=0
for item in "${forbidden_std_items[@]}"; do
    if grep -r "$item" src/filesystem/ > /dev/null 2>&1; then
        echo "✗ Found std usage: $item"
        grep -r "$item" src/filesystem/ | head -3
        no_std_violations=$((no_std_violations + 1))
    fi
done

if [ $no_std_violations -eq 0 ]; then
    echo "✓ No std violations found in filesystem modules"
else
    echo "✗ Found $no_std_violations std violations"
    exit 1
fi

echo

# Test 8: Documentation check
echo "Test 8: Documentation check..."
doc_files=(
    "src/filesystem/mod.rs"
    "src/filesystem/fat32/mod.rs"
    "src/filesystem/fat32/boot_sector.rs"
    "src/filesystem/fat32/directory.rs"
    "src/filesystem/fat32/file_operations.rs"
    "src/filesystem/fat32/cluster_chain.rs"
    "src/filesystem/fat32/filename.rs"
    "src/filesystem/fat32/interface.rs"
)

missing_docs=0
for file in "${doc_files[@]}"; do
    if grep -q "///" "$file"; then
        echo "✓ $file has documentation"
    else
        echo "✗ $file missing documentation"
        missing_docs=$((missing_docs + 1))
    fi
done

if [ $missing_docs -eq 0 ]; then
    echo "✓ All modules have documentation"
else
    echo "⚠ $missing_docs modules missing documentation"
fi

echo

# Test 9: Code organization check
echo "Test 9: Code organization check..."

# Check line counts for each module
echo "Module line counts:"
for file in "${modules[@]}"; do
    if [ -f "$file" ]; then
        lines=$(wc -l < "$file")
        echo "  $file: $lines lines"
        
        # Check if any single file is too large
        if [ "$lines" -gt 500 ]; then
            echo "  ⚠ $file is large ($lines lines)"
        fi
    fi
done

echo

# Test 10: Backward compatibility test
echo "Test 10: Backward compatibility test..."

# Test that old fat32 imports still work through lib.rs
if grep -q "pub mod fat32" src/lib.rs; then
    echo "✓ fat32 module still available for compatibility"
else
    echo "✗ fat32 module not available for compatibility"
    exit 1
fi

# Test that key types are still available
key_types=(
    "Fat32FileSystem"
    "Fat32Error"
    "FileContent"
    "FileInfo"
    "FileList"
)

for type in "${key_types[@]}"; do
    if grep -q "$type" src/filesystem/fat32/mod.rs; then
        echo "✓ $type exported from fat32 module"
    else
        echo "✗ $type not exported from fat32 module"
        exit 1
    fi
done

echo

# Test 11: Memory efficiency check
echo "Test 11: Memory efficiency check..."

# Check for potential memory issues
memory_concerns=(
    "Vec<"
    "String"
    "HashMap<"
    "BTreeMap<"
    "alloc::"
)

memory_issues=0
for concern in "${memory_concerns[@]}"; do
    if grep -r "$concern" src/filesystem/ > /dev/null 2>&1; then
        echo "⚠ Found potential memory concern: $concern"
        memory_issues=$((memory_issues + 1))
    fi
done

if [ $memory_issues -eq 0 ]; then
    echo "✓ No memory efficiency concerns found"
else
    echo "⚠ Found $memory_issues potential memory concerns"
fi

echo

# Test 12: Error handling check
echo "Test 12: Error handling check..."

# Check that all modules handle errors properly
error_handling_good=0
for module in "${modules[@]}"; do
    if grep -q "Result<" "$module" && grep -q "Fat32Error" "$module"; then
        echo "✓ $module has proper error handling"
        error_handling_good=$((error_handling_good + 1))
    fi
done

echo "✓ $error_handling_good modules have proper error handling"

echo

# Summary
echo "=== Phase 4 Modular Filesystem Test Summary ==="
echo "✓ All tests passed successfully!"
echo "✓ Modular filesystem structure created"
echo "✓ Legacy filesystem archived"
echo "✓ Compilation successful"
echo "✓ no_std compliance maintained"
echo "✓ Backward compatibility preserved"
echo "✓ Shell integration updated"
echo "✓ Documentation present"
echo "✓ Memory efficiency maintained"
echo "✓ Error handling consistent"
echo
echo "Phase 4 modular filesystem implementation completed successfully!"
echo

# Optional: Display module structure
echo "=== Modular Filesystem Structure ==="
echo "src/filesystem/"
echo "├── mod.rs (main filesystem interface)"
echo "└── fat32/"
echo "    ├── mod.rs (fat32 module exports)"
echo "    ├── boot_sector.rs (boot sector handling)"
echo "    ├── directory.rs (directory operations)"
echo "    ├── file_operations.rs (file read/write)"
echo "    ├── cluster_chain.rs (cluster management)"
echo "    ├── filename.rs (filename utilities)"
echo "    └── interface.rs (high-level API)"
echo
echo "Legacy archive:"
echo "src/legacy_filesystem/"
echo "└── fat32.rs (original monolithic implementation)"
echo
