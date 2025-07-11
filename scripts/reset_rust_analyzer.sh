#!/usr/bin/env bash

# Complete rust-analyzer reset for TinyOS
# This script completely resets rust-analyzer state and forces it to use no_std configuration

set -e

echo "🔄 Performing complete rust-analyzer reset for TinyOS..."
echo

# 1. Kill any running rust-analyzer processes
echo "1. Stopping rust-analyzer processes..."
pkill -f rust-analyzer || true
sleep 2

# 1.5. Ensure rust-analyzer is installed in the toolchain
echo "1.5. Ensuring rust-analyzer is installed..."
if ! rustup component list --installed --toolchain nightly | grep -q rust-analyzer; then
    echo "Installing rust-analyzer component..."
    rustup component add rust-analyzer --toolchain nightly
else
    echo "rust-analyzer component already installed"
fi

# 2. Clear all rust-analyzer caches
echo "2. Clearing rust-analyzer caches..."
rm -rf target/rust-analyzer 2>/dev/null || true
rm -rf target/.rustc_info.json 2>/dev/null || true
rm -rf target/debug/.fingerprint 2>/dev/null || true
rm -rf target/aarch64-unknown-none/debug/.fingerprint 2>/dev/null || true

# 3. Clear any VSCode cache related to rust-analyzer
echo "3. Clearing VSCode caches..."
rm -rf .vscode/.ropeproject 2>/dev/null || true

# 4. Verify critical configuration files are correct
echo "4. Verifying configuration files..."

# Check .rust-analyzer.toml has the critical setting
if ! grep -q "cargo.unsetTest = true" .rust-analyzer.toml; then
    echo "❌ cargo.unsetTest = true missing from .rust-analyzer.toml"
    exit 1
fi

# Check VSCode settings has the critical setting
if ! grep -q '"rust-analyzer.cargo.unsetTest": true' .vscode/settings.json; then
    echo "❌ rust-analyzer.cargo.unsetTest missing from VSCode settings"
    exit 1
fi

# Check no test tasks in VSCode
if grep -q '"cargo", "test"' .vscode/tasks.json; then
    echo "❌ Found cargo test task in .vscode/tasks.json - this can cause test crate issues"
    exit 1
fi

# 5. Force rebuild with correct target
echo "5. Forcing rebuild with embedded target..."
cargo clean
CARGO_BUILD_TARGET=aarch64-unknown-none cargo check --target aarch64-unknown-none

# 6. Verify no test-related build artifacts
echo "6. Verifying no test artifacts..."
if ls target/aarch64-unknown-none/debug/ 2>/dev/null | grep -i test; then
    echo "⚠️  Found test artifacts - this may indicate test compilation"
else
    echo "✅ No test artifacts found"
fi

echo
echo "🎉 Rust-analyzer reset complete!"
echo
echo "Next steps:"
echo "  1. If using VSCode, restart it completely (close all windows)"
echo "  2. Reopen the project"
echo "  3. Wait for rust-analyzer to fully initialize (check bottom status bar)"
echo "  4. If you still see 'can't find crate for test', run:"
echo "     Ctrl+Shift+P -> 'Rust Analyzer: Restart Server'"
echo
echo "The error should now be completely resolved."
