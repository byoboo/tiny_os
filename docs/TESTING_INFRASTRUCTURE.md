# TinyOS Testing Infrastructure

## Overview
TinyOS uses a **dual-approach testing strategy** combining comprehensive shell-based validation with a complete `no_std` Rust testing framework for embedded development.

## Testing Philosophy

### Dual Testing Approach
- **Shell-driven testing** - Interactive and automated validation via command interface
- **Rust testing framework** - Complete `no_std` kernel testing with UART output
- **Real hardware simulation** - Tests actual embedded behavior, not mocked components  
- **`no_std` native** - Pure embedded environment testing
- **QEMU-enabled development** - Full testing without physical hardware required

### Why Dual Testing?
Traditional Rust unit tests require the standard library (`std`), which is incompatible with embedded `no_std` targets. Our dual approach provides superior validation by:
- **Shell testing**: Interactive debugging, real-world validation, automated CI/CD
- **Rust framework**: Comprehensive kernel unit testing, integrated test execution
- **Combined coverage**: Both system-level and component-level validation
- **No external dependencies**: All tests run in the kernel environment

> 📋 **Note**: The "can't find crate for test" error has been completely resolved through comprehensive rust-analyzer configuration and our custom `no_std` testing framework.

## Test Structure Organization

### Current Directory Structure
```
tests/
├── scripts/                    # 26 shell test scripts
│   ├── test_qemu_boot.sh      # QEMU boot validation
│   ├── test_memory_suite.sh   # Memory system testing
│   ├── test_process_phase3.sh # Process management tests
│   ├── test_hardware_suite.sh # Hardware driver tests
│   ├── validate_tinyos.sh     # Comprehensive validation
│   └── ... (21 more scripts)
└── (other test files)

src/testing/                   # Rust testing framework
├── mod.rs                     # TestRunner implementation
├── kernel_tests.rs            # Core kernel tests
├── mmu_tests.rs              # Memory management tests
├── process_tests.rs          # Process management tests
├── syscall_tests.rs          # System call tests
└── integration_tests.rs      # Cross-component tests
```

## Main Test Runner

### `test_tinyos.sh`
The unified test runner that organizes tests by OS features:

**Usage:**
```bash
./test_tinyos.sh                    # Run all tests
./test_tinyos.sh --verbose          # Run with detailed output
./test_tinyos.sh boot               # Run only boot tests
./test_tinyos.sh memory interrupts  # Run specific feature tests
./test_tinyos.sh --validate-only    # Run only validation tests
```

**Features Tested:**
- `boot` - Boot system tests and QEMU validation
- `memory` - Memory management via shell commands
- `interrupts` - Interrupt handling and simulation
- `hardware` - Hardware drivers (UART, GPIO, Timer)

## Test Components

### 1. Rust Testing Framework (`src/testing/`)

**Core Testing Infrastructure:**
- **`mod.rs`** - TestRunner implementation with UART-based output
- **`kernel_tests.rs`** - Core kernel functionality validation
- **`mmu_tests.rs`** - Memory management unit testing
- **`process_tests.rs`** - Process management validation
- **`syscall_tests.rs`** - System call interface testing
- **`integration_tests.rs`** - Cross-component testing

**Interactive Testing Commands:**
- **`t`** - Run comprehensive kernel tests through shell
- **Custom assertions** - `no_std` compatible test assertions
- **UART output** - Real-time test results and diagnostics

### 2. Shell Script Testing (`tests/scripts/`)

**Build & Boot Validation:**
- **Build validation** - Ensures clean compilation for `aarch64-unknown-none` target
- **`test_qemu_boot.sh`** - Validates TinyOS boots successfully in QEMU
- **`validate_tinyos.sh`** - Comprehensive code structure and build validation

**Hardware-Focused Automated Tests:**
- **`test_memory_automated.sh`** - Memory manager via shell commands
- **`test_interrupt_automated.sh`** - Interrupt controller validation
- **`test_hardware_automated.sh`** - Hardware driver validation (GPIO, UART, Timer)

**System Integration Tests:**
- **`test_memory_suite.sh`** - Comprehensive memory system testing
- **`test_process_phase3.sh`** - Process management and scheduling tests
- **`test_exception_comprehensive.sh`** - Exception handling validation
- **`test_filesystem_modular.sh`** - FAT32 filesystem testing

**Specialized Test Scripts:**
- **`test_comprehensive_integration.sh`** - Full system integration testing
- **`test_drivers_modular.sh`** - Modular driver testing
- **`test_memory_modular.sh`** - Modular memory management testing

### 3. CI/CD Integration

**GitHub Workflows:**
- **`ci.yml`** - Main CI pipeline with automated testing
- **`feature.yml`** - Feature branch testing
- **`pr.yml`** - Pull request validation

**All workflows updated with new paths:**
- Shell scripts referenced from `tests/scripts/` directory
- Automated execution of test suites
- QEMU-based boot validation
- Build verification for ARM64 target

## Test Categories

### ✅ Current Testing Status
- **✅ Build Validation** - Clean compilation for embedded target
- **✅ Boot Tests** - QEMU boot validation, system initialization  
- **✅ Rust Testing Framework** - Complete `no_std` kernel testing
- **✅ Memory Tests** - Shell-based memory system validation
- **✅ Interrupt Tests** - Hardware simulation and validation
- **✅ Hardware Tests** - UART, GPIO, Timer validation via shell commands
- **✅ Process Tests** - Process management and scheduling validation
- **✅ Exception Tests** - Comprehensive exception handling validation
- **✅ System Call Tests** - SVC interface and privilege testing
- **✅ Integration Tests** - Cross-component validation
- **✅ CI/CD Integration** - Automated testing with new directory structure

### 🏗️ Testing Architecture
- **Dual approach** - Both shell scripts and Rust framework
- **Automated by default** - CI/CD ready with QEMU support
- **Feature-organized** - Tests grouped by OS functionality
- **Hardware-realistic** - Tests match actual deployment scenarios
- **Development-friendly** - Interactive debugging and exploration
- **Comprehensive coverage** - 26 shell scripts + complete Rust framework
- **Organized structure** - Clean separation of test types

## Test Design Principles

### 1. **Automated by Default**
- All tests run without external dependencies by default
- Interactive tests are opt-in with `--interactive` flag
- No manual intervention required for CI/CD

### 2. **Feature-Organized**
- Tests are grouped by OS functionality (boot, memory, interrupts, etc.)
- Each feature can be tested independently
- Clear separation between different test types

### 3. **Realistic Expectations**
- Tests validate what the system actually does, not idealized behavior
- Boot tests check for actual initialization messages
- Memory tests verify manager initialization, not runtime heap reporting
- Interrupt tests validate controller setup, not runtime interrupt handling

### 4. **Progressive Complexity**
- Quick validation tests for basic functionality
- Comprehensive automated tests for feature validation
- Optional interactive tests for deep debugging

## Build and Deployment Scripts

### `build.sh`
- Creates production kernel build
- Generates `kernel8.img` for Raspberry Pi deployment
- Provides deployment instructions

### `run.sh`
- Quick QEMU execution script
- Configured for Raspberry Pi 4/5 (raspi4b model)
- Proper hardware base addresses

## Test Results Summary

### Current Status: ✅ ALL TESTS PASSING

**Testing Statistics:**
- **26 Shell Scripts**: All passing in `tests/scripts/`
- **Rust Testing Framework**: Complete `no_std` kernel testing
- **CI/CD Integration**: Automated testing with new directory structure
- **100% Success Rate**: All tests consistently passing
- **Comprehensive Coverage**: Boot, memory, processes, hardware, exceptions

### Individual Test Results
- ✅ **Boot system tests** - QEMU boot validation and system initialization
- ✅ **Rust kernel tests** - Complete `no_std` testing framework
- ✅ **Memory management tests** - All memory system components
- ✅ **Process management tests** - Scheduling and context switching
- ✅ **Exception handling tests** - Comprehensive exception validation
- ✅ **Hardware/driver tests** - UART, GPIO, Timer validation
- ✅ **Integration tests** - Cross-component validation
- ✅ **System call tests** - SVC interface and privilege testing

### Testing Commands

**Shell Script Testing:**
```bash
# Run all shell tests
./tests/scripts/validate_tinyos.sh

# Individual test categories
./tests/scripts/test_memory_suite.sh
./tests/scripts/test_process_phase3.sh
./tests/scripts/test_hardware_suite.sh
./tests/scripts/test_qemu_boot.sh
./tests/scripts/test_exception_comprehensive.sh
```

**Rust Framework Testing:**
```bash
# Interactive kernel testing (within TinyOS shell)
t  # Run comprehensive kernel tests

# Build and run with testing
cargo build && ./run.sh
```

**Legacy Test Runner:**
```bash
# Full test suite (legacy, redirects to scripts)
./test_tinyos.sh

# With verbose output for debugging
./test_tinyos.sh --verbose
```

## Integration with Development

### Continuous Integration Ready
- All tests are automated and require no user interaction
- Tests validate actual system behavior with dual approach
- Clear pass/fail results with detailed diagnostics
- Suitable for automated build pipelines
- Updated CI/CD workflows with new directory structure

### Hardware Compatibility
- Tests are designed for Raspberry Pi 4/5 compatibility
- UART base addresses, QEMU model, and memory layout all Pi 4/5 specific
- No Pi 3 legacy support to maintain focus
- Both shell and Rust tests work in QEMU and on hardware

### Future Enhancements
- Tests can be extended to validate new features
- Framework supports adding new feature categories
- Interactive tests available for advanced debugging
- Easy to add hardware-specific tests when needed
- Rust testing framework can be expanded with new test modules

---

*Last Updated: July 11, 2025 - Updated for Phase 5 completion with comprehensive dual testing approach. All 26 shell scripts reorganized in tests/scripts/ directory, complete Rust testing framework in src/testing/, and full CI/CD integration.*
