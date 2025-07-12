# AI Assistant Context for TinyOS

## Project Overview

TinyOS is a bare-metal ARM64 operating system for Raspberry Pi 4/5, implemented in Rust with `#![no_std]`. The project has evolved through multiple phases and is currently in a mature state with enterprise-grade CI/CD, zero warnings, and comprehensive testing.

## Current Project State

- **Status**: Phase 3 Complete (CI/CD Pipeline & Automation)
- **Code Quality**: 0 compiler warnings, 117 clippy warnings (non-blocking)
- **Testing**: 7 automated test suites, 100% pass rate
- **CI/CD**: 4 GitHub Actions workflows, Docker-based development
- **Architecture**: Thread-safe, modular design with advanced memory management

## Development Workflow

### Standard Commands

- `make setup` - Initial Docker environment setup
- `make build` - Build the kernel
- `make test` - Run test suite
- `make lint` - Run clippy linting (permissive)
- `make lint-strict` - Run clippy with warnings as errors
- `make dev-cycle` - Quick build-test iteration

### File Organization Principles

- **Core OS**: `src/` - Main kernel components
- **Shell**: `src/shell/` - Interactive command interface
- **Memory**: `src/memory/` - Memory management subsystem
- **Drivers**: `src/drivers/` - Hardware abstraction layer
- **Testing**: `src/testing/` - Test framework integration
- **Documentation**: `docs/` - All documentation
- **CI/CD**: `.github/workflows/` - GitHub Actions

## Common Modification Patterns

### When editing shell commands

- Usually involves `src/shell/commands/*.rs`
- May need `src/shell/mod.rs` for new commands
- Often paired with memory/driver changes

### When fixing clippy warnings

- Focus on modern Rust patterns
- Use `strip_prefix()` instead of manual string manipulation
- Use `enumerate()` instead of manual indexing
- Use `is_ascii_digit()` instead of manual range checks
- Avoid `clone()` on `Copy` types

### When updating CI/CD

- Modify `.github/workflows/*.yml`
- Test with `make lint` vs `make lint-strict`
- Consider Docker environment changes

## Architecture Patterns

### Memory Management

- **Thread-safe**: Uses `spin::Mutex` instead of `static mut`
- **Modular**: Separate managers for allocation, protection, COW
- **Advanced**: Page tables, ASLR, stack protection, defragmentation

### Error Handling

- **Result-based**: Extensive use of `Result<T, E>` patterns
- **Custom errors**: Domain-specific error types
- **Graceful fallbacks**: System continues operation when possible

### Shell Architecture

- **Command pattern**: Each command as separate module
- **State management**: Centralized system state access
- **User interaction**: Real-time feedback and help system

## Technical Terminology

- **MMU**: Memory Management Unit (ARM64 hardware)
- **COW**: Copy-on-Write memory sharing
- **ASLR**: Address Space Layout Randomization
- **CFI**: Control Flow Integrity
- **ESR**: Exception Syndrome Register (ARM64)
- **TTBR**: Translation Table Base Register
- **MAIR**: Memory Attribute Indirection Register

## Testing Strategies

### Unit Tests

- Component-level testing in `src/testing/`
- Memory manager unit tests
- Shell command validation

### Integration Tests

- Cross-component interaction testing
- System-level validation
- Hardware abstraction testing

### CI/CD Testing

- Docker-based environment testing
- Multi-workflow validation
- Code quality enforcement

## Common Debugging Scenarios

### Clippy Warnings

- **Modern patterns**: Replace manual implementations with stdlib equivalents
- **Performance**: Use iterator patterns instead of manual loops
- **Safety**: Avoid unnecessary `clone()` operations

### Memory Issues

- **Allocation**: Check memory manager statistics
- **Protection**: Verify page table entries
- **COW**: Validate copy-on-write behavior

### Shell Issues

- **Commands**: Verify command registration in `mod.rs`
- **Parsing**: Check argument parsing logic
- **State**: Validate system state access

## Development Priorities

1. **Code Quality**: Maintain zero warnings, address clippy suggestions
2. **Testing**: Expand test coverage, add integration tests
3. **Documentation**: Keep docs current with code changes
4. **Performance**: Optimize critical paths, memory usage
5. **Features**: Add new shell commands, memory management features

## File Change Patterns

### High-frequency changes

- `src/shell/commands/*.rs` - Command implementations
- `src/memory/*.rs` - Memory management
- `.github/workflows/*.yml` - CI/CD adjustments

### Paired changes

- Shell commands + memory managers
- Driver updates + shell commands
- Test framework + implementation code

## AI Assistant Guidelines

### When making changes

1. **Read context first**: Check existing code patterns
2. **Follow conventions**: Use established naming and structure
3. **Test thoroughly**: Run `make test` and `make lint`
4. **Document changes**: Update relevant documentation
5. **Consider dependencies**: Check for related components

### When fixing issues

1. **Understand scope**: Is this a single component or system-wide?
2. **Check patterns**: Look for similar implementations
3. **Validate fixes**: Ensure tests still pass
4. **Consider side effects**: Check for unintended consequences

This context file should help AI assistants understand the project structure, common patterns, and development workflow for more effective assistance.
