# VSCode & Development Environment Setup for TinyOS

This document explains the complete development environment setup for TinyOS, including VSCode configuration, rust-analyzer setup, and solutions for common `no_std` embedded development issues.

## The Problem: no_std Development Challenges

**Why this configuration is critical**: Developing embedded `no_std` projects presents unique challenges:

### 1. Global Cargo Configuration Interference
- Global settings in `~/.cargo/config.toml` can interfere with project-specific settings
- Global configs often assume std targets
- They might set different default targets or incompatible rustflags
- rust-analyzer picks up these global settings, causing confusion

### 2. "Can't find crate for test" Error
In `no_std` embedded projects, rust-analyzer often shows this error because:
- The `test` crate is part of the standard library (`std`)
- `no_std` projects don't have access to the standard library
- Rust-analyzer tries to analyze test code even when tests aren't compatible with the target

### 3. Target Architecture Complexity
- Need explicit targeting for `aarch64-unknown-none`
- Require custom linker scripts and memory layouts
- Standard library features must be completely disabled

## The Solution: Complete Project Isolation

We've configured this project to explicitly override any global cargo settings and ensure complete isolation while eliminating common `no_std` errors.

## Configuration Files

### `.rust-analyzer.toml` - Primary Rust-Analyzer Configuration
```toml
# Force embedded target and disable test compilation
cargo.target = "aarch64-unknown-none"
cargo.unsetTest = true  # KEY: Prevents test crate lookup error
cargo.features = []

# Disable test-related features completely
runnables.enable = false
lens.enable = false
```

### `.vscode/settings.json` - VSCode Integration
- Configures rust-analyzer for the `aarch64-unknown-none` target
- **Completely disables test-related analysis** to prevent errors
- Forces rust-analyzer to use project-specific cargo config
- Sets explicit environment variables to override global settings
- Configures file associations for assembly and linker scripts

Key settings:
```json
{
    "rust-analyzer.cargo.target": "aarch64-unknown-none",
    "rust-analyzer.cargo.unsetTest": true,
    "rust-analyzer.lens.run.enable": false,
    "rust-analyzer.lens.debug.enable": false,
    "rust-analyzer.cargo.runBuildScripts": false
}
```

### `.cargo/config.toml` - Cargo Project Configuration
- Sets default target to `aarch64-unknown-none`
- Specifies custom linker script
- Configures target-specific rustflags
- Overrides any global cargo configuration

### `.vscode/tasks.json` - Development Tasks
- Build tasks for debug and release modes
- QEMU testing task for hardware simulation
- Validation and testing tasks
- Clippy linting task

### `.vscode/launch.json`
- Configured for QEMU debugging
- Runs the project in QEMU for testing

### `.vscode/extensions.json`
- Recommends useful extensions for embedded Rust development

## Additional Configuration Files

### `rust-toolchain.toml`
- Specifies nightly Rust toolchain (recommended for embedded development)
- Includes required targets and components for bare-metal development
- Note: Project can also build with stable Rust if needed

### `rustfmt.toml`
- Code formatting configuration
- Ensures consistent style across the project

### `.cargo/config.toml`
- Sets default target to `aarch64-unknown-none`
- Configures linker settings
- Sets up QEMU runner for testing
- **Explicitly overrides global cargo configuration** to prevent interference
- Forces the correct target even if global settings differ

### `.env`
- Workspace-specific environment variables
- Ensures project isolation from global cargo settings
- Prevents inheritance of problematic global configurations

## Troubleshooting Common Issues

### "Can't find crate for test" Error
**Status**: ✅ RESOLVED
- **Root cause**: rust-analyzer trying to analyze test code in `no_std` environment
- **Solution**: Comprehensive test disabling via `cargo.unsetTest = true` and related settings
- **Result**: Error completely eliminated

### Global Cargo Configuration Interference  
**Status**: ✅ RESOLVED
- **Root cause**: Global `~/.cargo/config.toml` overriding project settings
- **Solution**: Project-level configuration with explicit overrides
- **Result**: Complete project isolation achieved

### Target Architecture Issues
**Status**: ✅ RESOLVED
- **Root cause**: Conflicting target specifications
- **Solution**: Explicit `aarch64-unknown-none` targeting in all config files
- **Result**: Consistent targeting across all tools

### Build Script Errors
**Status**: ✅ RESOLVED
- **Root cause**: Build scripts trying to access std library
- **Solution**: `cargo.runBuildScripts = false` in rust-analyzer config
- **Result**: Clean analysis without build script interference

## Development Workflow

### Quick Start
1. Open the project in VSCode
2. Install recommended extensions when prompted
3. Configuration will be automatically applied
4. Use `Ctrl+Shift+P` → "Rust Analyzer: Restart Server" if needed

### Building and Testing
- Use `Ctrl+Shift+P` → "Tasks: Run Task" → "Build (Debug)" or "Build (Release)"
- For QEMU testing: "Run QEMU Test"
- For validation: "Run TinyOS Tests"

### Debugging
- The project is configured for QEMU debugging
- Use F5 to start debugging session
- Breakpoints and step debugging work in QEMU environment

## Benefits of This Configuration

- **Zero setup friction**: Works immediately after cloning
- **Global config immunity**: Unaffected by user's global cargo settings  
- **Error-free analysis**: No spurious rust-analyzer errors
- **Optimized for embedded**: All tools configured for bare-metal development
- **Team consistency**: Same environment for all developers
- **CI/CD ready**: Configuration works in automated environments

## File Overview

| File | Purpose | Key Settings |
|------|---------|--------------|
| `.rust-analyzer.toml` | Primary rust-analyzer config | `cargo.unsetTest = true` |
| `.vscode/settings.json` | VSCode integration | Target override, test disabling |
| `.cargo/config.toml` | Cargo project config | Default target, linker script |
| `.vscode/tasks.json` | Build and test tasks | QEMU integration, validation |
| `rust-toolchain.toml` | Toolchain specification | Nightly with required components |

This configuration ensures a smooth development experience for embedded Rust development with TinyOS.
