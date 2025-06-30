# TinyOS - Raspberry Pi Operating System

A minimal operating system designed to run on Raspberry Pi 4 and 5, developed in Rust.

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
│   ├── main.rs        # Main kernel code
│   └── boot.s         # Assembly boot code
├── .cargo/
│   └── config.toml    # Cargo configuration
├── linker.ld          # Linker script for memory layout
├── aarch64-raspi.json # Custom target specification
└── build.sh           # Build script
```

## Memory Layout

- **Load Address**: 0x80000 (512KB from start of RAM)
- **Stack**: Grows downward from load address
- **BSS**: Cleared on boot

## Current Features

- [x] Bare-metal boot process
- [x] Basic UART output
- [x] Multi-core aware (only CPU 0 active)
- [x] QEMU Raspberry Pi 4B emulation support
- [x] Working kernel that boots and prints messages
- [ ] GPIO control and proper UART initialization
- [ ] Interrupt handling
- [ ] Memory management
- [ ] Process scheduling
- [ ] Filesystem support

## Next Steps

1. **GPIO and Hardware Control**: Implement proper GPIO initialization and control
2. **Interrupt Handling**: Set up exception vectors and interrupt handling
3. **Memory Management**: Implement virtual memory and heap allocation
4. **Device Drivers**: UART, I2C, SPI, etc.
5. **Process Management**: Basic task scheduling and process isolation

## QEMU Testing

The kernel is configured to run on QEMU's Raspberry Pi 4 emulation. This allows for:
- Fast development cycles
- Easy debugging
- Consistent testing environment

## Real Hardware Deployment

To run on actual Raspberry Pi hardware:
1. Copy `kernel8.img` to the SD card boot partition
2. Ensure these files are also present:
   - `bootcode.bin` (Pi 4 only)
   - `start4.elf`
   - `fixup4.dat`
   - `config.txt` (optional, for configuration)

## Debugging

Use GDB with QEMU for debugging:
```bash
# Terminal 1: Start QEMU with GDB server
qemu-system-aarch64 -M raspi4 -kernel kernel8.img -serial stdio -display none -s -S

# Terminal 2: Connect GDB
gdb-multiarch target/aarch64-raspi/debug/tiny_os
(gdb) target remote localhost:1234
(gdb) continue
```
