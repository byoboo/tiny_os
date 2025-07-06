# TinyOS Test Suite Documentation

## Overview

TinyOS now has a unified, feature-organized test suite that provides comprehensive testing across all OS components. The testing is organized by core OS features rather than scattered across multiple scripts.

## Unified Test Runner: `test_tinyos.sh`

The main test runner provides a single interface for all TinyOS testing needs.

### Quick Start

```bash
# Run all tests interactively
./test_tinyos.sh

# Run specific OS feature tests
./test_tinyos.sh memory
./test_tinyos.sh interrupts hardware

# Run in different modes
./test_tinyos.sh --mode automated all
./test_tinyos.sh --mode quick boot memory

# Quick validation only
./test_tinyos.sh --validate-only
```

## Test Organization by OS Features

### 1. **Boot System Tests** (`boot`)
- **Purpose**: Validates system initialization and boot sequence
- **Components**:
  - QEMU boot validation (`test_qemu_boot.sh`)
  - Basic system validation (`validate_tinyos.sh`)
- **Coverage**: System startup, kernel loading, initial state

### 2. **Memory Management Tests** (`memory`)
- **Purpose**: Comprehensive memory system testing
- **Components**: 
  - Memory allocation/deallocation
  - Memory protection and boundaries
  - Fragmentation analysis
  - Performance benchmarks
- **Coverage**: All memory management subsystems

### 3. **Interrupt Management Tests** (`interrupts`)
- **Purpose**: Interrupt handling and controller testing
- **Components**:
  - Handler registration and execution
  - Priority level management
  - Nested interrupt scenarios
  - Performance analysis
- **Coverage**: Complete interrupt subsystem

### 4. **Hardware/Driver Tests** (`hardware`)
- **Purpose**: Hardware abstraction layer testing
- **Components**:
  - GPIO and LED control
  - UART communication
  - Timer functionality
  - System diagnostics
- **Coverage**: All hardware interfaces

### 5. **Unit Tests** (`unit`)
- **Purpose**: Rust unit tests for core components
- **Components**: All `#[test]` annotated functions
- **Coverage**: Individual function and module testing

## Test Modes

### Interactive Mode (default)
- Manual test execution with user prompts
- Real-time feedback and control
- Best for development and debugging

### Automated Mode
- Expect-based automated testing
- No user interaction required
- Best for CI/CD pipelines

### Quick Mode
- Fast subset of critical tests
- Essential functionality verification
- Best for rapid validation

## Usage Examples

### Development Workflow
```bash
# Quick validation during development
./test_tinyos.sh --validate-only

# Test specific feature you're working on
./test_tinyos.sh memory --mode quick

# Full testing before commit
./test_tinyos.sh --mode automated all
```

### CI/CD Integration
```bash
# Automated pipeline testing
./test_tinyos.sh --mode automated all --verbose

# Quick smoke tests
./test_tinyos.sh --mode quick boot unit
```

### Debugging and Development
```bash
# Verbose output for debugging
./test_tinyos.sh memory --verbose

# Interactive testing for manual verification
./test_tinyos.sh interrupts --mode interactive
```

## Test Suite Files

### Active Test Suites
- `test_tinyos.sh` - Unified test runner (main interface)
- `test_memory_suite.sh` - Memory management tests
- `test_interrupt_suite.sh` - Interrupt management tests  
- `test_hardware_suite.sh` - Hardware/driver tests
- `test_qemu_boot.sh` - Boot validation tests
- `validate_tinyos.sh` - Basic system validation

### Archived Tests
All legacy test scripts have been moved to `archived_tests/`:
- `run_tests.sh` - Legacy test runner
- `run_test_suites.sh` - Legacy suite runner
- `test_interactive.sh` - Legacy interactive tests
- Various `test_memory_*.sh` and `test_interrupts_*.sh` files

### Configuration and Verification
- `verify_test_organization.sh` - Verifies test organization
- `TEST_SUITE_ORGANIZATION.md` - Detailed organization documentation

## Test Results and Reporting

The unified test runner provides:
- Color-coded output with status indicators
- Comprehensive test summary with pass/fail counts
- Verbose mode for detailed debugging output
- Integration with existing test frameworks

## Migration from Legacy Tests

If you were using the old test scripts:

| Old Command | New Command |
|-------------|-------------|
| `./run_tests.sh` | `./test_tinyos.sh` |
| `./run_test_suites.sh memory` | `./test_tinyos.sh memory` |
| `./test_memory_suite.sh --mode automated` | `./test_tinyos.sh memory --mode automated` |
| `./validate_tinyos.sh` | `./test_tinyos.sh --validate-only` |

## Adding New Tests

To add tests for new OS features:

1. **For new OS components**: Add to the appropriate existing suite or create a new feature category
2. **For unit tests**: Add `#[test]` functions to Rust source files
3. **For integration tests**: Extend the relevant suite in `test_*_suite.sh`

## Best Practices

1. **Use feature-specific testing**: Target the specific OS component you're testing
2. **Choose appropriate mode**: Interactive for development, automated for CI/CD
3. **Start with validation**: Use `--validate-only` for quick health checks
4. **Enable verbose output**: Use `--verbose` when debugging test failures
5. **Test incrementally**: Test individual features during development, all features before commits
