# TinyOS - Raspberry Pi Operating System

A sophisticated bare-metal operating system designed to run on Raspberry Pi 4 and 5, developed in Rust. TinyOS features comprehensive memory management, interrupt handling, and an interactive shell interface.

## Table of Contents

- [Features Overview](#features-overview)
- [Prerequisites](#prerequisites)
- [Building and Running](#building-and-running)
- [Project Structure](#project-structure)
- [Memory Layout](#memory-layout)
- [Current Features](#current-features)
- [Comprehensive Testing Suite](#comprehensive-testing-suite)
- [Complete Shell Command Reference](#complete-shell-command-reference)
- [Interactive Shell Interface](#interactive-shell-interface)
- [Advanced Features Detail](#advanced-features-detail)
- [Development and Testing](#development-and-testing)
- [Real Hardware Deployment](#real-hardware-deployment)
- [Debugging and Development](#debugging-and-development)
- [Technical Implementation Details](#technical-implementation-details)
- [Future Development Roadmap](#future-development-roadmap)

## Features Overview

TinyOS has evolved from a simple "Hello, World!" kernel into a fully functional operating system with:

- ✅ **Bare-metal ARM64 kernel** with custom boot process
- ✅ **Interactive shell** with real-time command processing
- ✅ **Comprehensive memory management** with bitmap allocation
- ✅ **Interrupt management system** with ARM GIC simulation
- ✅ **Hardware drivers** for UART, GPIO, and System Timer
- ✅ **Diagnostic and testing suite** with health checks
- ✅ **QEMU development environment** with real hardware deployment ready

## Prerequisites

### Development Environment
- **Rust**: Install from [rustup.rs](https://rustup.rs/)
- **QEMU**: For testing and development
  ```bash
  # macOS
  brew install qemu
  
  # Ubuntu/Debian
  sudo apt install qemu-system-arm
  
  # Arch Linux
  sudo pacman -S qemu-arch-extra
  ```

### Rust Toolchain Setup
```bash
# Add the AArch64 target for cross-compilation
rustup target add aarch64-unknown-none-softfloat
```

## Building and Running

### Development (QEMU)
```bash
# Easy way - use the run script
./run.sh

# Manual way
cargo build
qemu-system-aarch64 -M raspi4b -kernel target/aarch64-unknown-none/debug/tiny_os -serial stdio -display none

# Or build and run in one step (if cargo runner is configured properly)
cargo run
```

**Note**: Press `Ctrl+A` then `X` to exit QEMU.

### For Real Hardware (Raspberry Pi 4/5)
1. Build the kernel: `cargo build --release`
2. Copy `target/aarch64-unknown-none/release/tiny_os` to SD card as `kernel8.img`
3. Ensure you have the Raspberry Pi firmware files on the SD card
4. Boot the Pi

## Project Structure

```
├── src/
│   ├── main.rs           # Main kernel and interactive shell
│   ├── boot.s            # Assembly boot code and initialization  
│   ├── uart.rs           # PL011 UART driver
│   ├── gpio.rs           # GPIO and LED control driver
│   ├── timer.rs          # BCM2835 System Timer driver
│   ├── memory.rs         # Bitmap-based memory manager
│   ├── interrupts.rs     # ARM GIC interrupt controller
│   └── tests/            # Rust unit tests
├── .cargo/
│   └── config.toml       # Cargo configuration for cross-compilation
├── linker.ld             # Custom linker script for memory layout
├── aarch64-raspi.json    # Custom target specification
├── build.sh              # Build script
├── run.sh                # QEMU execution script
├── test_tinyos.sh        # Unified test runner (main testing interface)
├── test_*_suite.sh       # Feature-specific test suites
├── archived_tests/       # Legacy test scripts (archived)
├── TESTING.md            # Comprehensive testing documentation
└── *.md                  # Project documentation
```

## Memory Layout

- **Kernel Space**: 0x80000 - 0x100000 (512KB)
- **Heap Space**: 0x100000 - 0x500000 (4MB managed heap)
- **Memory Manager**: Bitmap-based allocation with 64-byte blocks
- **Total Blocks**: 65,536 blocks for fine-grained allocation
- **Stack**: Grows downward from kernel load address

## Current Features

### Core System
- [x] **Bare-metal ARM64 boot process** with custom assembly initialization
- [x] **Interactive shell interface** with real-time command processing
- [x] **Multi-core aware** initialization (CPU 0 active, others parked)
- [x] **QEMU Raspberry Pi 4B emulation** with real hardware deployment ready
- [x] **Comprehensive panic handling** with UART error reporting

### Hardware Drivers
- [x] **PL011 UART driver** with full input/output capabilities
- [x] **GPIO control system** with LED management and pin configuration
- [x] **BCM2835 System Timer** with microsecond precision delays
- [x] **ARM GIC interrupt controller** simulation with real hardware hooks

### Memory Management
- [x] **Bitmap-based heap allocator** with 64-byte block granularity
- [x] **4MB managed heap** with 65,536 addressable blocks
- [x] **Memory corruption detection** with canary values and integrity checking
- [x] **Fragmentation analysis** with real-time monitoring
- [x] **Multi-block allocation** support for contiguous memory
- [x] **Memory statistics** and comprehensive diagnostic reporting

### Interrupt System
- [x] **ARM GIC simulation** for QEMU development
- [x] **Timer, UART, and GPIO interrupts** with individual enable/disable
- [x] **Interrupt statistics tracking** with real-time monitoring
- [x] **Comprehensive interrupt testing** with simulation framework

## Comprehensive Testing Suite

TinyOS features a unified, feature-organized test suite that provides comprehensive testing across all OS components. The testing is organized by core OS features for better maintainability and clarity.

### Unified Test Runner: `test_tinyos.sh`

The main test interface provides organized testing by OS features:

```bash
# Run all tests interactively
./test_tinyos.sh

# Test specific OS features
./test_tinyos.sh memory           # Memory management tests
./test_tinyos.sh interrupts       # Interrupt handling tests  
./test_tinyos.sh hardware         # Hardware/driver tests
./test_tinyos.sh boot             # Boot and validation tests
./test_tinyos.sh unit             # Rust unit tests

# Different test modes
./test_tinyos.sh --mode automated all    # Automated CI/CD testing
./test_tinyos.sh --mode quick memory     # Quick validation
./test_tinyos.sh --validate-only         # Basic health check only
```

### Test Organization by OS Features

#### 1. **Boot System Tests** (`boot`)
- QEMU boot validation and system initialization
- Basic system validation and health checks
- Kernel loading and startup verification

#### 2. **Memory Management Tests** (`memory`)
- Allocation/deallocation with data integrity validation
- Stress testing with fragmentation scenarios
- Boundary testing and alignment validation
- Memory protection and corruption detection
- Performance benchmarks and usage analysis

#### 3. **Interrupt Management Tests** (`interrupts`)
- Handler registration and execution testing
- Priority level and nested interrupt scenarios
- Controller validation (GIC distributor/CPU interface)
- Performance analysis and latency testing
- Enable/disable functionality validation

#### 4. **Hardware/Driver Tests** (`hardware`)
- GPIO control and LED management
- UART communication and serial I/O
- Timer functionality and timing accuracy
- System diagnostics and hardware health checks

#### 5. **Unit Tests** (`unit`)
- Rust unit tests for all core components
- Individual function and module testing

### Interactive Shell Testing

Through the interactive shell, you can also run tests directly:
- `x` - Basic memory test  
- `z` - Comprehensive memory test suite
- `j` - Complete interrupt test suite
- `c` - System health check (all systems)
- `i` - View interrupt status and statistics

### Legacy Test Access
All previous test scripts have been consolidated and are available through the unified runner. See `TESTING.md` for detailed migration information.

## Development and Testing

### Quick Development Workflow
```bash
# Quick validation during development
./test_tinyos.sh --validate-only

# Test specific feature you're working on  
./test_tinyos.sh memory --mode quick

# Full testing before commit
./test_tinyos.sh --mode automated all
```

### QEMU Testing Environment
The kernel runs on QEMU's Raspberry Pi 4 emulation for fast development:
```bash
# Build and run in QEMU
./run.sh

# Run unified test suite
./test_tinyos.sh

# Quick boot validation
./test_tinyos.sh boot --mode quick
```

## Future Development Roadmap

### Near-term Goals
1. **Exception Vectors**: Implement proper ARM64 exception handling
2. **Real Hardware Testing**: Validate on actual Raspberry Pi 4/5
3. **Improved GPIO Control**: Enhanced hardware control capabilities
4. **Device Driver Framework**: Standardized driver interface

### Long-term Goals
1. **Virtual Memory Management**: MMU configuration and page tables
2. **Process/Task Management**: Multi-tasking and process isolation
3. **File System**: Basic FAT32 or custom filesystem support
4. **Network Stack**: Basic TCP/IP implementation
5. **Multi-core Support**: SMP (Symmetric Multi-Processing)
6. **User Space**: Kernel/user mode separation

## QEMU Testing

The kernel is configured to run on QEMU's Raspberry Pi 4 emulation. This allows for:
- Fast development cycles
- Easy debugging
- Consistent testing environment

## Real Hardware Deployment

To deploy TinyOS on actual Raspberry Pi 4/5 hardware:

### SD Card Setup
1. Format SD card with FAT32 partition
2. Download official Raspberry Pi firmware files:
   - `bootcode.bin` (Pi 4 only)
   - `start4.elf` and `start4cd.elf` 
   - `fixup4.dat` and `fixup4cd.dat`
3. Copy firmware files to SD card root
4. Build release version: `cargo build --release`
5. Copy `target/aarch64-unknown-none/release/tiny_os` to SD card as `kernel8.img`

### Optional Configuration (`config.txt`)
```ini
# Enable 64-bit mode
arm_64bit=1

# Set GPU memory split (16MB minimum for GPU)
gpu_mem=16

# Disable rainbow splash screen
disable_splash=1

# Enable UART for debugging
enable_uart=1
uart_2ndstage=1
```

### Hardware Features Ready
- ✅ **UART Console**: Full bidirectional communication
- ✅ **GPIO Control**: LED and pin management  
- ✅ **System Timer**: Microsecond precision timing
- ✅ **Memory Management**: 4MB heap with bitmap allocation
- ✅ **Interrupt Support**: ARM GIC ready for real hardware

## Debugging and Development

### GDB Debugging with QEMU
TinyOS supports full GDB debugging through QEMU's integrated GDB server:

```bash
# Terminal 1: Start QEMU with GDB server (paused)
qemu-system-aarch64 -M raspi4b -kernel target/aarch64-unknown-none/debug/tiny_os \
  -serial stdio -display none -s -S

# Terminal 2: Connect GDB
gdb-multiarch target/aarch64-unknown-none/debug/tiny_os
(gdb) target remote localhost:1234
(gdb) break main
(gdb) continue
```

### Debugging Features Available
- **Breakpoints**: Set breakpoints in kernel code
- **Memory inspection**: Examine heap, stack, and kernel memory
- **Register analysis**: View ARM64 CPU registers
- **Step debugging**: Step through kernel execution
- **Variable inspection**: Examine kernel variables and structures

### Development Workflow
1. **Code Changes**: Edit Rust source files
2. **Build**: `cargo build` for debug builds
3. **Test in QEMU**: `./run.sh` for quick testing
4. **Debug if needed**: Use GDB for detailed analysis
5. **Comprehensive Test**: Run test scripts for validation
6. **Deploy**: Build release version for hardware

### Performance Analysis
- **Memory usage tracking**: Real-time heap utilization monitoring
- **Interrupt statistics**: Performance metrics for interrupt handling
- **System timing**: Microsecond precision timing measurements
- **Resource monitoring**: CPU and memory resource usage

## Interactive Shell Interface

TinyOS features a comprehensive interactive shell that provides real-time access to all system functionality:

### System Commands
- **h/H** - Display comprehensive help menu with all available commands
- **t/T** - Show current system time and uptime in seconds
- **s/S** - Display detailed system information (OS version, hardware specs, memory stats)
- **c/C** - Run comprehensive system health check with all subsystem tests
- **d/D** - Display hardware diagnostics and system information

### Hardware Control
- **1** - Turn LED ON (GPIO 42)
- **0** - Turn LED OFF (GPIO 42)  
- **l/L** - Toggle LED state (ON↔OFF)

### Memory Management Commands
- **m/M** - Display detailed memory statistics and layout information
- **a/A** - Allocate a memory block and display address
- **f/F** - Free the last allocated memory block
- **x/X** - Run basic memory allocation/deallocation test
- **z/Z** - Run comprehensive memory test suite (all 5 tests)
- **g/G** - Run memory corruption check and bitmap validation
- **r/R** - Perform memory defragmentation and report results

### Interrupt Management Commands
- **i/I** - Show interrupt controller status and statistics
- **e/E** - Enable all major interrupt sources (Timer, UART, GPIO)
- **j/J** - Run comprehensive interrupt test with simulation

### Command Input Features
- **Case-insensitive**: Both uppercase and lowercase commands work
- **Real-time feedback**: Immediate command execution and response
- **Error handling**: Invalid commands show helpful error messages
- **Timestamping**: All commands show execution time

### Example Shell Session
```
TinyOS v0.1.0 - Raspberry Pi Kernel
Type 'h' for help or any command to interact with TinyOS
----------------------------------------
You typed: 'h' (ASCII 104) at [5.230s]

=== TinyOS Command Reference ===
System Commands:
  h/H - Show this help menu
  t/T - Show current system time
  s/S - Show system information
  c/C - Run system health check
  d/D - Display hardware diagnostics
Hardware Control:
  1 - Turn LED ON
  0 - Turn LED OFF
  l/L - Toggle LED state
Memory Management:
  m/M - Display memory statistics
  a/A - Allocate memory block
  f/F - Free last allocated block
  x/X - Run basic memory test
  z/Z - Run comprehensive memory test suite
  g/G - Run memory corruption check
  r/R - Perform memory defragmentation
Interrupt Management:
  i/I - Show interrupt status
  e/E - Enable all interrupts
  j/J - Run interrupt test

You typed: 'm' (ASCII 109) at [8.445s]

=== Memory Statistics ===
Heap Layout:
  Start Address: 0x100000
  End Address: 0x500000
  Total Size: 4194304 bytes
Block Information:
  Block Size: 64 bytes
  Total Blocks: 65536
  Used Blocks: 0
  Free Blocks: 65536
Memory Usage:
  Used: 0 bytes
  Free: 4194304 bytes
  Usage: 0%
  Largest Free Block: 4193664 bytes
Advanced Info:
  Fragmentation: 0%
  Corruption Check: ✓ CLEAN
========================
```

## Advanced Features Detail

### Memory Management Deep Dive

#### Architecture Specifications
- **Heap Range**: 0x100000 - 0x500000 (4MB total)
- **Block Granularity**: 64 bytes for optimal ARM64 alignment
- **Allocation Method**: Bitmap-based with O(n) allocation, O(1) deallocation
- **Protection**: Canary values and corruption detection
- **Fragmentation**: Real-time analysis and defragmentation support

#### Memory Statistics Example
```
=== Memory Statistics ===
Heap Layout:
  Start Address: 0x100000
  End Address: 0x500000
  Total Size: 4194304 bytes
Block Information:
  Block Size: 64 bytes
  Total Blocks: 65536
  Used Blocks: 0
  Free Blocks: 65536
Memory Usage:
  Used: 0 bytes
  Free: 4194304 bytes
  Usage: 0%
  Largest Free Block: 4193664 bytes
Advanced Info:
  Fragmentation: 0%
  Corruption Check: ✓ CLEAN
========================
```

### Interrupt System Deep Dive

#### ARM GIC Configuration
- **GIC Distributor**: 0xFF841000 (hardware-ready base address)
- **GIC CPU Interface**: 0xFF842000 (ready for multi-core)
- **QEMU Simulation**: Full testing without hardware dependencies
- **Interrupt Sources**: 256 total, with 3 major sources implemented

#### Supported Interrupts
1. **Timer (IRQ 64)**: BCM2835 System Timer with microsecond precision
2. **UART (IRQ 153)**: PL011 serial communication interrupts
3. **GPIO (IRQ 129)**: General-purpose I/O pin change interrupts

#### Interrupt Status Example
```
=== Interrupt Status ===
Controller State:
  Enabled Interrupts: 0x7

Interrupt Sources:
  Timer (IRQ 64): ENABLED (15 interrupts)
  UART (IRQ 153): ENABLED (8 interrupts)
  GPIO (IRQ 129): ENABLED (3 interrupts)

Statistics:
  Total Interrupts: 26
========================
```

## Technical Implementation Details

### Boot Process
1. **Assembly Boot (`boot.s`)**: ARM64 CPU initialization, stack setup, other CPUs parked
2. **Rust Kernel Entry (`main.rs`)**: Hardware initialization, memory setup, shell start
3. **Driver Initialization**: UART, GPIO, Timer, Memory Manager, Interrupt Controller
4. **Interactive Shell**: Real-time command processing loop

### Hardware Abstraction
- **Memory-mapped I/O**: Direct hardware register manipulation
- **ARM64 Assembly**: Low-level CPU and hardware control
- **Cross-compilation**: Rust targeting `aarch64-unknown-none-softfloat`
- **Custom Linker Script**: Precise memory layout control for bare-metal environment

### Key Technologies
- **Language**: Rust (no_std, embedded-friendly)
- **Target**: ARM64 (AArch64) for Raspberry Pi 4/5
- **Boot**: Custom assembly with ARM64 exception level handling
- **Memory**: Bitmap-based allocator with corruption protection
- **I/O**: Memory-mapped hardware register access
- **Testing**: QEMU emulation with hardware validation ready

### Code Organization
```
src/
├── main.rs         # Kernel entry, shell, command processing
├── boot.s          # Assembly boot code and CPU initialization  
├── uart.rs         # PL011 UART driver (input/output)
├── gpio.rs         # GPIO control and LED management
├── timer.rs        # BCM2835 system timer (microsecond precision)
├── memory.rs       # Bitmap memory manager (4MB heap)
└── interrupts.rs   # ARM GIC interrupt controller
```

### Build System
- **Cargo**: Rust package manager with custom target
- **Cross-compilation**: AArch64 toolchain integration
- **Custom Target**: `aarch64-unknown-none-softfloat` for bare-metal
- **Linker Script**: Custom memory layout for Raspberry Pi
- **Test Scripts**: Automated testing and validation
