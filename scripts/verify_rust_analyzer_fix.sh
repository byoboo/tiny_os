#!/usr/bin/env bash

# Verification script for "can't find crate for test" fix
# This script verifies that rust-analyzer configuration is working correctly

set -e

echo "🔍 Verifying rust-analyzer configuration for TinyOS..."
echo

# Check configuration files exist
echo "✅ Checking configuration files:"
files=(".rust-analyzer.toml" ".vscode/settings.json" ".cargo/config.toml")
for file in "${files[@]}"; do
    if [[ -f "$file" ]]; then
        echo "  ✓ $file exists"
    else
        echo "  ❌ $file missing"
        exit 1
    fi
done
echo

# Check no test-related configurations in Cargo.toml
echo "✅ Checking Cargo.toml has no test configurations:"
if grep -q "\[profile\.test\]" Cargo.toml; then
    echo "  ❌ Found [profile.test] in Cargo.toml - this causes test crate issues"
    exit 1
else
    echo "  ✓ No [profile.test] found"
fi
echo

# Check rust-analyzer config has test disabling
echo "✅ Checking rust-analyzer test disabling:"
if grep -q "cargo.unsetTest = true" .rust-analyzer.toml; then
    echo "  ✓ Found cargo.unsetTest = true"
else
    echo "  ❌ cargo.unsetTest not found in .rust-analyzer.toml"
    exit 1
fi
echo

# Check build works without test-related errors
echo "✅ Checking build for test-related errors:"
if cargo check 2>&1 | grep -i "can't find crate for.*test"; then
    echo "  ❌ Found 'can't find crate for test' error"
    exit 1
else
    echo "  ✓ No 'can't find crate for test' errors found"
fi
echo

# Check that target directory doesn't have test artifacts
echo "✅ Checking for test artifacts:"
if ls target/aarch64-unknown-none/debug/ 2>/dev/null | grep -i test; then
    echo "  ⚠️  Found test artifacts (may indicate test compilation)"
else
    echo "  ✓ No test artifacts found"
fi
echo

# Verify key configuration values
echo "✅ Verifying key configuration values:"

# Check target is set correctly
if grep -q 'target = "aarch64-unknown-none"' .cargo/config.toml; then
    echo "  ✓ Cargo target set to aarch64-unknown-none"
else
    echo "  ❌ Cargo target not set correctly"
    exit 1
fi

# Check rust-analyzer target
if grep -q 'cargo.target = "aarch64-unknown-none"' .rust-analyzer.toml; then
    echo "  ✓ Rust-analyzer target set to aarch64-unknown-none"
else
    echo "  ❌ Rust-analyzer target not set correctly"
    exit 1
fi

# Check VSCode test disabling
if grep -q '"rust-analyzer.cargo.unsetTest": true' .vscode/settings.json; then
    echo "  ✓ VSCode test disabling configured"
else
    echo "  ❌ VSCode test disabling not configured"
    exit 1
fi
echo

echo "🎉 All rust-analyzer configuration checks passed!"
echo
echo "The 'can't find crate for test' error should be resolved."
echo "If you're still seeing the error:"
echo "  1. Restart VSCode completely"
echo "  2. Run: Ctrl+Shift+P -> 'Rust Analyzer: Restart Server'"
echo "  3. Clear rust-analyzer cache if needed"
echo
echo "Configuration summary:"
echo "  • Tests completely disabled in rust-analyzer"
echo "  • Target locked to aarch64-unknown-none"
echo "  • No test profiles in Cargo.toml"
echo "  • Shell-based testing used instead (see docs/TESTING_INFRASTRUCTURE.md)"
