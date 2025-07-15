# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build and Development Commands

### Primary Development Workflow (Docker-based)
```bash
# Setup (one-time)
make setup          # Build Docker development environment

# Core development cycle
make build          # Build kernel and extract binary
make test           # Run comprehensive test suite
make dev-cycle      # Build + test in one command

# Hardware deployment
make build-pi       # Create kernel8.img for Raspberry Pi

# Testing specific components
./test_tinyos.sh memory        # Memory management tests
./test_tinyos.sh interrupts    # Interrupt handling tests
./test_tinyos.sh hardware      # Hardware/driver tests
./test_tinyos.sh boot          # Boot system tests
```

### Code Quality and Maintenance
```bash
make format         # Format Rust code
make lint          # Run clippy linter
make clean         # Clean build artifacts
make dev-shell     # Enter interactive development shell
```

### Legacy Commands (host-based, requires local Rust/QEMU)
```bash
cargo build --release --target aarch64-unknown-none
./build.sh         # Create kernel8.img
./test_tinyos.sh   # Run tests on host
```

## System Architecture

TinyOS is a sophisticated bare-metal ARM64 operating system with a modular, layered architecture:

### Core Architecture Layers
1. **Interactive Shell** (`src/shell/`) - 40+ commands organized by functionality
2. **Process Management** (`src/process/`) - Scheduling, context switching, privilege control
3. **Advanced Memory Management** (`src/memory/`) - MMU, COW, page tables, protection
4. **Exception & Interrupt System** (`src/exceptions/`, `src/interrupts.rs`) - Comprehensive ARM64 exception handling
5. **Hardware Abstraction** (`src/drivers/`) - Modular driver architecture
6. **Filesystem Support** (`src/filesystem/`) - FAT32 implementation

### Key Technical Components

#### Memory Management (`src/memory/`)
- **MMU (`mmu/`)**: ARM64 page table management, virtual memory mapping
- **COW (`cow/`)**: Copy-on-Write memory sharing with reference counting
- **User Space (`user_space/`)**: Per-process page table isolation
- **Protection (`protection/`)**: Memory protection, stack guards, ASLR
- **Dynamic (`dynamic/`)**: Dynamic allocation, lazy loading, pressure handling

#### Exception System (`src/exceptions/`)
- **Deferred Processing (`deferred/`)**: Work queues, soft IRQs, statistics
- **ESR Decoding**: Comprehensive ARM64 exception syndrome analysis
- **Memory Faults**: Page fault handling and recovery
- **Syscall Interface**: System call handling framework

#### Process Management (`src/process/`)
- **Scheduler (`scheduler/`)**: Round-robin, priority-based scheduling
- **Context Management**: Save/restore operations, EL0/EL1 switching
- **Privilege Control**: Secure user/kernel mode transitions

#### Hardware Drivers (`src/drivers/`)
All drivers follow a modular architecture with hardware abstraction:
- **GPIO (`gpio/`)**: BCM2835 register access, LED control
- **UART (`uart/`)**: PL011 hardware support
- **Timer (`timer/`)**: Microsecond precision timing
- **SD Card (`sdcard/`)**: EMMC interface

#### Advanced Driver Modules (Week 4-6 Refactored)
Professional modular architecture achieved through Project Baseline Initiative:
- **Performance (`performance/`)**: Benchmarking, power management, thermal control, metrics
- **Network (`network/`)**: Ethernet, WiFi, USB 3.0, protocols with unified controller
- **Security (`security/`)**: TrustZone, real-time scheduling, hardening, threat detection

### Boot Process
1. **Assembly Boot** (`src/boot.s`) - Initial ARM64 setup
2. **Kernel Main** (`src/main.rs`) - Component initialization
3. **System Initialization** - MMU, exceptions, drivers, shell

## Development Patterns

### Module Organization
- Each major component has its own module directory
- Hardware abstraction through traits (`src/drivers/traits.rs`)
- Modular testing framework (`src/testing/`)
- Legacy compatibility layers maintained

### Error Handling
- No-std environment with custom error types
- Comprehensive exception handling with ESR decoding
- Memory fault analysis and recovery mechanisms

### Testing Strategy
- **External Integration Tests**: `tests/scripts/` - Shell script automation
- **Internal Kernel Tests**: `src/testing/` - No-std testing framework
- **Test Categories**: Boot, memory, interrupts, hardware, filesystem
- **Unified Test Runner**: `./test_tinyos.sh` with feature selection

## Shell Command Organization

The interactive shell provides organized command groups:
- **Memory Management**: `0`-`6`, `+`(COW), `-`(User Space), `@`(Protection), `*`(Dynamic)
- **Exception Testing**: `7`-`9`, `!` (Synchronous), `#`-`%` (IRQ)
- **Process Control**: `[`, `\\`, `]`, `&` (Process management)
- **System Control**: `(`, `)`, `{`, `}` (MMU), `<`, `>`, `?` (Virtual memory)
- **Hardware Testing**: `a`-`f` (Filesystem), `g`-`o` (Drivers)
- **Advanced Features**: `4` (Performance), `5` (Network), `6` (Security)

## Project Structure Notes

- **Target**: ARM64 (`aarch64-unknown-none`) for Raspberry Pi 4/5
- **Features**: Conditional compilation with `raspi3` feature for Pi 3 support
- **No-std**: Embedded environment with custom allocators and panic handlers
- **Docker Environment**: Complete containerized development workflow
- **CI/CD**: GitHub Actions with automated testing and quality gates

## Project Baseline Initiative Achievement

### Major Refactoring Completed
The Project Baseline Initiative has successfully transformed week-specific prototypes into professional modular architecture:

- **Code Modernization**: 4,000+ lines refactored from week-specific files to organized modules
- **Architecture Transformation**: Week 4-6 features → `drivers/performance/`, `drivers/network/`, `drivers/security/`
- **no_std Compatibility**: All modules verified for embedded environment compatibility
- **Zero Regressions**: 100% build success rate with comprehensive testing

### Modular Architecture Structure
```
src/drivers/
├── performance/         # Week 4 → Performance benchmarking and management
│   ├── benchmarks.rs    # Comprehensive performance testing
│   ├── power.rs         # Power management and CPU/GPU scaling
│   ├── thermal.rs       # Thermal monitoring and control
│   └── metrics.rs       # System-wide performance metrics
├── network/             # Week 5 → Network and I/O management
│   ├── ethernet.rs      # Gigabit Ethernet with DMA
│   ├── wifi.rs          # WiFi 6 with WPA3 security
│   ├── protocols.rs     # USB 3.0, SPI, I2C protocols
│   └── controller.rs    # Unified network controller
└── security/            # Week 6 → Security and real-time systems
    ├── trustzone.rs     # ARM TrustZone secure/non-secure worlds
    ├── realtime.rs      # Microsecond-precision scheduling
    ├── hardening.rs     # System hardening and exploit mitigation
    └── controller.rs    # Integrated security controller
```

### Quality Achievements
- **Build Quality**: Zero compilation errors with modern Rust compatibility
- **Testing**: 25+ no_std compatible unit tests across all modules
- **Documentation**: Professional module-level documentation
- **Maintainability**: Clear separation of concerns and modular design

## Important Implementation Details

### Memory Addresses
- Hardware addresses are Pi-specific (BCM2835/BCM2711)
- Virtual memory management with 4KB page granularity
- Guard pages for stack overflow protection

### Exception Handling
- ARM64 exception vectors with comprehensive ESR decoding
- Nested interrupt support with priority management
- Deferred processing for performance optimization

### Performance Characteristics
- Boot time: ~2 seconds in QEMU, ~5 seconds on hardware
- Exception latency: < 50 CPU cycles
- Memory allocation: < 1ms latency
- Context switch: < 200 CPU cycles

## Testing and Validation

### Critical Test Files
- `test_tinyos.sh` - Unified test runner
- `tests/scripts/test_*_automated.sh` - Automated test suites
- `tests/scripts/validate_tinyos.sh` - Basic validation

### Expected Test Behavior
- All external integration tests should pass (100% pass rate)
- Unit tests may fail due to no-std limitations (expected)
- Interactive tests require `expect` tool and `--interactive` flag

## Code Quality Requirements

- Zero compiler warnings policy
- Clippy linting with strict mode available
- Rust formatting with `rustfmt.toml` configuration
- Professional documentation standards