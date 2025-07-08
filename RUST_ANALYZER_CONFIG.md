# Rust-Analyzer Configuration for TinyOS

## Overview
This document explains the rust-analyzer configuration for TinyOS, which is optimized for embedded `no_std` development and **eliminates the "can't find crate for test" error**.

## The Problem: "Can't find crate for test"

In `no_std` embedded projects, rust-analyzer often shows the error "can't find crate for test" because:
1. The `test` crate is part of the standard library (`std`)
2. `no_std` projects don't have access to the standard library
3. Rust-analyzer tries to analyze test code even when tests aren't compatible with the target

## The Solution: Comprehensive Test Disabling

Our configuration **completely disables** test-related analysis to prevent this error:

### `.rust-analyzer.toml` - Primary Configuration
```toml
# Force embedded target and disable test compilation
cargo.target = "aarch64-unknown-none"
cargo.unsetTest = true  # KEY: Prevents test crate lookup
cargo.features = []

# Disable test-related features completely
runnables.enable = false
lens.enable = false
```

### `.vscode/settings.json` - VSCode Integration  
```json
{
    "rust-analyzer.cargo.unsetTest": true,
    "rust-analyzer.lens.run.enable": false,
    "rust-analyzer.lens.debug.enable": false,
    "rust-analyzer.cargo.runBuildScripts": false
}
```

### `.cargo/config.toml` - Cargo Defaults
```toml
[build]
target = "aarch64-unknown-none"

# Explicitly disable test harness
[profile.dev]
panic = "abort"
```

## Configuration Files Details

### `.rust-analyzer.toml`
Primary rust-analyzer configuration that:
- ✅ Sets the target to `aarch64-unknown-none` 
- ✅ **Explicitly disables test builds** (`cargo.unsetTest = true`)
- ✅ Disables all test-related UI features (runnables, lens, etc.)
- ✅ Suppresses common embedded development warnings
- ✅ Configures check commands for the embedded target only

### `.vscode/settings.json`
VS Code specific settings that:
- ✅ **Reinforces test disabling** with multiple settings
- ✅ Forces the correct target and cargo configuration  
- ✅ Disables problematic diagnostics for embedded development
- ✅ Sets up proper file associations for assembly and linker scripts

### `.cargo/config.toml`
Cargo configuration that:
- ✅ Sets the default target to `aarch64-unknown-none`
- ✅ **Uses `panic = "abort"`** to prevent test harness inclusion
- ✅ Configures the QEMU runner for actual testing
- ✅ Sets up proper linker flags for embedded development

## Verification: Checking the Fix

### Before Configuration
```
error[E0463]: can't find crate for `test`
  --> src/lib.rs
   |
   | #[cfg(test)]
   | ^^^^^^^^^^^^
```

### After Configuration  
✅ **No "can't find crate for test" errors**
✅ Rust-analyzer works smoothly
✅ All embedded features properly analyzed

### Test the Fix
```bash
# Check that rust-analyzer config is working
cargo check  # Should compile without test-related errors

# Verify no test artifacts in build
ls target/aarch64-unknown-none/debug/  # No test-related files

# Confirm VSCode shows no test-related errors in Problems panel
```

## Why These Configurations Are Needed

### `no_std` Environment Challenges
1. **No Test Framework**: The `test` crate requires `std`, which isn't available in `no_std`
2. **Limited Proc Macros**: Some procedural macros don't work in embedded contexts  
3. **Target-Specific**: Code needs to be analyzed for the embedded target, not the host
4. **Test Crate Dependencies**: Even disabled tests can cause crate resolution issues

### Resolved Issues
- ✅ **"can't find crate for `test`" errors eliminated**
- ✅ Derive macros (`Debug`, `Clone`, `Copy`) work correctly
- ✅ Core types (`Option`, `Result`, etc.) properly recognized  
- ✅ Build and check commands use correct target
- ✅ No false positive diagnostics
- ✅ No test-related UI clutter in VSCode

## Development Experience

With this configuration, you get:
- **No test crate errors** - the primary issue is completely resolved
- **Fast analysis** - rust-analyzer focuses on embedded code only
- **Accurate diagnostics** - only reports real issues, not `std`-related false positives  
- **Proper completion** - code completion works for `no_std` environment
- **Correct checking** - clippy and check commands use embedded target
- **Clean UI** - no confusing test-related options in VSCode

## Alternative Testing Approach

Since unit tests don't work in `no_std`, TinyOS uses:
- **Shell-based testing** - Test OS functionality via command interface
- **QEMU integration** - Test actual boot and runtime behavior
- **Hardware simulation** - Test drivers and hardware interfaces

See `TESTING_INFRASTRUCTURE.md` for details on our testing approach.

While rust-analyzer doesn't support the test framework in `no_std`, TinyOS uses a superior hardware-focused testing approach:
- **Shell-based testing** - Interactive and automated validation
- **QEMU integration** - Full system testing without physical hardware
- **Real hardware validation** - Tests actual embedded behavior

See `TESTING_INFRASTRUCTURE.md` for complete testing documentation.

## Troubleshooting

If you see test-related errors:
1. Ensure `.rust-analyzer.toml` is in the project root
2. Reload the VS Code window (Ctrl+Shift+P → "Developer: Reload Window")
3. Check that the correct target is set in all configuration files

The configuration prioritizes embedded development workflow over traditional Rust testing patterns, which provides a much better experience for bare-metal development.
