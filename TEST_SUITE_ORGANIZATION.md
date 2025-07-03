# TinyOS Test Suite Organization

This document describes the organized test suite structure for TinyOS, consolidating and deduplicating test scripts by OS functionality.

## Test Suite Structure

The test suite is now organized into three main functional areas:

### 1. Memory Management Suite (`test_memory_suite.sh`)
**Purpose**: Comprehensive testing of memory allocation, deallocation, and management.

**Test Categories**:
- Basic allocation/deallocation tests
- Memory protection and boundary testing
- Fragmentation analysis and defragmentation
- Stress testing with multiple allocations
- Corruption detection and validation
- Performance benchmarks for memory operations

**Modes**:
- `interactive`: Manual test execution with user prompts
- `automated`: Expect-based automated testing
- `quick`: Fast subset of critical tests

### 2. Interrupt Management Suite (`test_interrupt_suite.sh`)
**Purpose**: Testing interrupt handling, priority management, and controller functionality.

**Test Categories**:
- Interrupt handler registration and execution
- Priority level testing
- Nested interrupt scenarios
- Enable/disable functionality
- Performance analysis of interrupt latency
- Edge cases and error conditions

**Modes**:
- `interactive`: Manual test execution with user prompts
- `automated`: Expect-based automated testing
- `quick`: Fast subset of critical tests

### 3. Hardware/Driver Suite (`test_hardware_suite.sh`)
**Purpose**: Testing hardware abstraction layer and device drivers.

**Test Categories**:
- GPIO pin control and LED management
- UART communication and serial I/O
- Timer functionality and timing accuracy
- System initialization sequences
- Hardware diagnostics and health checks
- Performance benchmarks for I/O operations

**Modes**:
- `interactive`: Manual test execution with user prompts
- `automated`: Expect-based automated testing
- `quick`: Fast subset of critical tests

## Test Runners

### Primary Test Runner (`run_tests.sh`)
The main entry point for all testing activities.

**Usage**:
```bash
./run_tests.sh [OPTIONS] [TEST_TYPE]

# Examples:
./run_tests.sh                           # Run all tests
./run_tests.sh unit                      # Run unit tests only
./run_tests.sh integration --mode quick  # Run integration tests in quick mode
```

**Test Types**:
- `unit`: Rust unit tests for core components
- `integration`: Organized integration test suites
- `all`: Both unit and integration tests (default)

**Options**:
- `-m, --mode MODE`: Integration test mode (interactive, automated, quick)
- `-v, --verbose`: Enable verbose output
- `-h, --help`: Show help message

### Unified Test Suite Runner (`run_test_suites.sh`)
Orchestrates the execution of all integration test suites.

**Usage**:
```bash
./run_test_suites.sh [OPTIONS] [SUITE]

# Examples:
./run_test_suites.sh                     # Run all suites interactively
./run_test_suites.sh memory              # Run memory tests only
./run_test_suites.sh --mode automated all # Run all suites in automated mode
```

**Test Suites**:
- `memory`: Memory management tests
- `interrupt`: Interrupt management tests
- `hardware`: Hardware/driver tests
- `all`: All test suites (default)

## Archived Test Scripts

The following redundant test scripts have been moved to `archived_tests/`:

- `test_memory.sh` → Consolidated into `test_memory_suite.sh`
- `test_memory_comprehensive.sh` → Consolidated into `test_memory_suite.sh`
- `test_memory_automated.sh` → Consolidated into `test_memory_suite.sh`
- `test_memory_simple.sh` → Consolidated into `test_memory_suite.sh`
- `test_enhanced_memory.sh` → Consolidated into `test_memory_suite.sh`
- `test_interrupts.sh` → Consolidated into `test_interrupt_suite.sh`
- `test_interrupts_automated.sh` → Consolidated into `test_interrupt_suite.sh`
- `test_interrupts_simple.sh` → Consolidated into `test_interrupt_suite.sh`

## Test Execution Workflow

### Quick Test Run
```bash
# Fast validation of critical functionality
./run_tests.sh integration --mode quick
```

### Full Interactive Testing
```bash
# Complete test suite with interactive prompts
./run_tests.sh all
```

### Automated CI/CD Testing
```bash
# Non-interactive automated testing for CI/CD
./run_tests.sh all --mode automated
```

### Specific Functionality Testing
```bash
# Test only memory management
./run_test_suites.sh memory --mode automated

# Test only interrupt handling
./run_test_suites.sh interrupt --mode quick

# Test only hardware/drivers
./run_test_suites.sh hardware
```

## Test Result Reporting

### Test Reports
Both test runners generate comprehensive test reports:
- `test_report.md`: Overall test results and summary
- Individual suite reports with detailed pass/fail information

### Test Outputs
- Colorized console output for easy result identification
- Verbose mode for detailed debugging information
- Structured logging for automated analysis

## Dependencies

### Required Tools
- `expect`: For automated testing (install with `brew install expect` on macOS)
- `timeout`: Command timeout utility (usually pre-installed)
- `cargo`: Rust build system for unit tests

### Optional Tools
- `qemu-system-aarch64`: For hardware emulation testing
- `gdb-multiarch`: For debugging test failures

## Best Practices

### Running Tests
1. Always run tests from the project root directory
2. Ensure the project builds successfully before running integration tests
3. Use `quick` mode for rapid feedback during development
4. Use `automated` mode for CI/CD pipelines
5. Use `interactive` mode for detailed debugging

### Test Development
1. Add new tests to the appropriate functional suite
2. Maintain both interactive and automated test paths
3. Include performance benchmarks for critical operations
4. Test edge cases and error conditions
5. Update documentation when adding new test categories

### Debugging Test Failures
1. Run tests in verbose mode (`-v, --verbose`)
2. Use interactive mode to step through test execution
3. Check individual component tests before running full suites
4. Review test reports for detailed failure information

## Migration Guide

If you were using the old test scripts, here's the migration path:

| Old Script | New Equivalent |
|------------|----------------|
| `./test_memory.sh` | `./run_test_suites.sh memory` |
| `./test_interrupts.sh` | `./run_test_suites.sh interrupt` |
| `./test_memory_automated.sh` | `./run_test_suites.sh memory --mode automated` |
| `./test_interrupts_automated.sh` | `./run_test_suites.sh interrupt --mode automated` |
| Multiple script execution | `./run_test_suites.sh all` |

The new test suites provide all the functionality of the original scripts while eliminating duplication and providing a more organized structure.
