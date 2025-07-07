# TinyOS - Raspberry Pi Operating System

A sophisticated bare-metal operating system designed to run on Raspberry Pi 4 and 5, develo4. **Build release version:**
   ```bash
   cargo build --release --target aarch64-unknown-none
   ```

5. **Extract raw kernel binary:**
   ```bash
   # IMPORTANT: Extract only the .text section containing executable code
   # (The ELF places rodata first, but Pi firmware expects code at 0x80000)
   rust-objcopy -j .text -O binary target/aarch64-unknown-none/release/tiny_os kernel8.img
   ```

6. **Copy kernel to SD card:**
   ```bash
   cp kernel8.img /path/to/sdcard/
   ```st. TinyOS features comprehensive memory management, interrupt handling, and an interactive shell interface.

## Features

- ✅ **Bare-metal ARM64 kernel** with custom boot process
- ✅ **Interactive shell** with real-time command processing *(UART only)*
- ✅ **Comprehensive memory management** with bitmap allocation
- ✅ **Interrupt management system** with ARM GIC simulation
- ✅ **Exception vector table** with ARM64 exception handling
- ✅ **SD card driver** with EMMC interface and block I/O operations
- ✅ **Hardware drivers** for UART, GPIO, System Timer, and SD/EMMC
- ✅ **Diagnostic and testing suite** with health checks
- ✅ **QEMU development environment** with real hardware deployment ready
- ✅ **Memory protection** with corruption detection and canary values
- ✅ **Defragmentation support** and real-time memory analysis
- ✅ **Performance benchmarks** and comprehensive test suites
- ✅ **Cross-platform development** with automated testing
- 🔧 **Serial-based interface** - No HDMI/video output (by design)

## Quick Start

### Development Environment Setup

#### Prerequisites
- **Rust**: Install from [rustup.rs](https://rustup.rs/)
- **QEMU**: For testing and development

**Install QEMU:**
```bash
# macOS
brew install qemu

# Ubuntu/Debian  
sudo apt install qemu-system-arm

# Arch Linux
sudo pacman -S qemu-arch-extra
```

**Setup Rust Toolchain:**
```bash
# The project uses nightly by default (configured in rust-toolchain.toml)
# This provides the best embedded development experience
rustup install nightly

# Add the AArch64 target for cross-compilation
rustup target add aarch64-unknown-none

# Note: The project can also build with stable Rust if needed:
# rustup override set stable
```
```

#### Building and Running

**Development (QEMU):**
```bash
# Easy way - use the run script
./run.sh

# Manual way
cargo build
qemu-system-aarch64 -M raspi4b -kernel target/aarch64-unknown-none/debug/tiny_os -serial stdio -display none
```

**Note**: Press `Ctrl+A` then `X` to exit QEMU.

### Testing

**Run all tests:**
```bash
./test_tinyos.sh
```

**Quick validation:**
```bash
./test_tinyos.sh validate
```

**Test specific components:**
```bash
./test_tinyos.sh boot       # Boot and system validation tests
./test_tinyos.sh unit       # Rust unit tests
./test_tinyos.sh memory     # Memory management tests (automated)
./test_tinyos.sh interrupts # Interrupt handling tests (automated)
./test_tinyos.sh hardware   # Hardware/driver tests (automated)
```

**Advanced options:**
```bash
./test_tinyos.sh --verbose  # Show detailed output
./test_tinyos.sh --interactive  # Use interactive test suites (requires expect)
./test_tinyos.sh --help     # Show all available options
```

**Note**: Automated tests are the default and require no external dependencies. Interactive test suites (for manual testing) require the `expect` tool and the `--interactive` flag.

## Real Hardware Deployment

### SD Card Setup for Raspberry Pi 4/5

1. **Format SD card** with FAT32 partition

   **CRITICAL: Proper SD card formatting is essential for Pi 5 boot!**
   
   **Option A: Using Raspberry Pi Imager (Recommended):**
   ```bash
   # Download and use Raspberry Pi Imager
   # 1. Select "CHOOSE OS" → "Erase (Format as FAT32)"
   # 2. Select your SD card
   # 3. Write (this creates proper partition table)
   ```
   
   **Option B: Manual formatting (Linux/macOS):**
   ```bash
   # DANGER: Replace /dev/sdX with your actual SD card device!
   # Check with: lsblk or df -h
   
   # Unmount the SD card first
   sudo umount /dev/sdX*
   
   # Create new partition table and FAT32 partition
   sudo fdisk /dev/sdX
   # In fdisk: o (new partition table) → n (new partition) → p (primary) 
   # → 1 (partition number) → Enter → Enter → t (change type) → c (FAT32) → w (write)
   
   # Format as FAT32
   sudo mkfs.vfat -F 32 /dev/sdX1
   ```
   
   **Option C: Using diskutil (macOS):**
   ```bash
   # Find your SD card
   diskutil list
   
   # Unmount and format (replace diskX with your SD card)
   diskutil unmountDisk /dev/diskX
   sudo diskutil eraseDisk FAT32 BOOT MBR /dev/diskX
   ```

2. **Download Raspberry Pi firmware files from [rpi-firmware](https://github.com/raspberrypi/firmware/tree/master/boot):**
   
   **For Pi 4:**
   - `start4.elf`, `start4cd.elf`, `start4db.elf`, `start4x.elf`
   - `fixup4.dat`, `fixup4cd.dat`, `fixup4db.dat`, `fixup4x.dat`
   - `bootcode.bin` (Pi 4 only)
   
   **For Pi 5:**
   - `start_cd.elf` (debug), `start.elf` (standard), `start_x.elf` (extended)
   - `fixup_cd.dat`, `fixup.dat`, `fixup_x.dat`
   - `bcm2712-rpi-5-b.dtb` (device tree blob - **this is in the same boot directory!**)

   **Quick download for Pi 5:**
   ```bash
   # Download Pi 5 firmware files
   wget https://github.com/raspberrypi/firmware/raw/master/boot/start_cd.elf
   wget https://github.com/raspberrypi/firmware/raw/master/boot/fixup_cd.dat
   wget https://github.com/raspberrypi/firmware/raw/master/boot/bcm2712-rpi-5-b.dtb
   ```

3. **Copy firmware files** to SD card root

4. **What is the Device Tree Blob (DTB)?**
   
   The `bcm2712-rpi-5-b.dtb` file is the **Device Tree Blob** for Raspberry Pi 5. It describes the hardware layout to the firmware and kernel:
   - **Required for Pi 5**: Pi 5 has different hardware than older models
   - **Hardware description**: Tells the system about GPIO pins, peripherals, memory layout
   - **Firmware dependency**: The Pi firmware uses this to initialize hardware correctly
   
   **Without the DTB file, Pi 5 will not boot properly!**

6. **Create config.txt** (required for proper boot):
   ```ini
   # Enable 64-bit mode
   arm_64bit=1
   
   # Use appropriate start file based on memory split
   start_file=start_cd.elf
   fixup_file=fixup_cd.dat
   
   # Set GPU memory split (16MB minimum)
   gpu_mem=16
   
   # Enable UART for debugging
   enable_uart=1
   
   # Disable rainbow splash screen
   disable_splash=1
   
   # For Pi 5: specify device tree
   device_tree=bcm2712-rpi-5-b.dtb
   
   # Kernel loading address (important!)
   kernel_address=0x80000
   ```

7. **Build release version:**
   ```bash
   cargo build --release --target aarch64-unknown-none
   ```

8. **Extract raw kernel binary: (this is already part of build.sh)**
   ```bash
   # IMPORTANT: Extract only the .text section containing executable code
   # (The ELF places rodata first, but Pi firmware expects code at 0x80000)
   rust-objcopy -j .text -O binary target/aarch64-unknown-none/release/tiny_os kernel8.img
   ```

9. **Copy kernel to SD card:**
   ```bash
   cp kernel8.img /path/to/sdcard/
   ```

### Debugging Hardware Boot Issues

**Check boot files on SD card:**
```bash
# Your SD card should contain these files:
ls /path/to/sdcard/
# Expected: config.txt, kernel8.img, start_cd.elf, fixup_cd.dat, bcm2712-rpi-5-b.dtb (Pi 5)
```

**Enable UART debugging:**

**Pi 5 boots but no output visible? This is likely a UART configuration issue.**

1. **Connect USB-to-TTL serial adapter to Pi GPIO pins:**
   - Pin 8 (GPIO 14, TXD) → Adapter RX
   - Pin 10 (GPIO 15, RXD) → Adapter TX  
   - Pin 6 (Ground) → Adapter Ground

2. **Open serial terminal BEFORE powering on Pi:**
   ```bash
   # Linux/macOS
   screen /dev/ttyUSB0 115200
   # Or use: minicom -D /dev/ttyUSB0 -b 115200
   # Or use: picocom /dev/ttyUSB0 -b 115200
   
   # Windows
   # Use PuTTY, TeraTerm, or similar with COM port at 115200 baud
   ```

3. **Expected output when TinyOS boots:**
   ```
   TinyOS v0.1.0 - Raspberry Pi Kernel
   Kernel started successfully!
   Running on QEMU Raspberry Pi 4 emulation
   Initializing System Timer...
   System Timer initialized!
   Initializing GPIO...
   GPIO initialized!
   Initializing Memory Manager...
   Memory Manager initialized!
   Initializing Interrupt Controller...
   Interrupt Controller initialized!
   
   TinyOS Interactive Shell
   Type 'h' for help, 'x' to exit
   > 
   ```

4. **If you see no output at all:**
   - Check UART adapter connections
   - Verify config.txt has `enable_uart=1`
   - Try different baud rates: 9600, 38400, 115200
   - Check if adapter needs drivers installed

5. **If you see garbled output:**
   - Wrong baud rate (should be 115200)
   - Check TX/RX aren't swapped
   - Verify 3.3V adapter (NOT 5V!)

6. **Alternative: Check HDMI output**
   
   **IMPORTANT: TinyOS has NO HDMI/display output!**
   
   TinyOS is a UART-only operating system designed for embedded/bare-metal development:
   - **No video output**: HDMI monitor will remain blank (this is normal!)
   - **Serial communication only**: All input/output is via UART
   - **Embedded design**: Like most bare-metal kernels, it uses serial for debugging/interaction
   
   **Your HDMI monitor staying blank is expected behavior - not an error!**

**Check kernel build target:**
```bash
# Verify correct target was used
file target/aarch64-unknown-none/release/tiny_os
# Should show: "ELF 64-bit LSB executable, ARM aarch64"

# CRITICAL: Extract only .text section for Pi firmware
rust-objcopy -j .text -O binary target/aarch64-unknown-none/release/tiny_os kernel8.img

# Verify raw binary was created and starts with boot code
file kernel8.img
# Should show: "data"
hexdump -C kernel8.img | head -2
# Should show ARM assembly instructions, starting with something like:
# 00000000  a1 00 38 d5 21 04 40 92  61 00 00 b4 5f 20 03 d5
```

**Common issues:**
- **Missing device tree blob** (Pi 5 requirement)
- **Wrong start/fixup files** (Pi 5 uses different names)
- **Missing config.txt** or incorrect kernel_address
- **Wrong build target** (must be aarch64-unknown-none)
- **Incorrect firmware versions** (use latest from official repo)
- **Wrong binary extraction**: Must use `-j .text` to extract only executable code, not entire ELF
- **SD card filesystem issues**: "Unable to read partition as FAT" - see SD card troubleshooting below

### SD Card Filesystem Troubleshooting

**Error: "Unable to read partition as FAT type: 32 lba: 0"**

This means the Pi firmware cannot read your SD card's filesystem. This is **NOT** a kernel issue:

1. **Check SD card compatibility:**
   ```bash
   # Use high-quality SD card (Class 10, 32GB or less recommended)
   # Avoid cheap/counterfeit cards
   ```

2. **Verify partition table:**
   ```bash
   # Linux/macOS - check partition structure
   sudo fdisk -l /dev/sdX  # Replace sdX with your SD card
   
   # Should show:
   # Device     Boot Start   End Sectors  Size Id Type
   # /dev/sdX1  *        1   ... ...      ...  c  W95 FAT32 (LBA)
   ```

3. **Re-format SD card completely:**
   ```bash
   # SAFEST: Use Raspberry Pi Imager to format
   # Download from: https://www.raspberrypi.org/software/
   # Choose "Erase (Format as FAT32)" option
   ```

4. **Test SD card on another device:**
   ```bash
   # Mount on your computer and verify you can read/write files
   # If this fails, the SD card may be corrupted
   ```

5. **Check file copy integrity:**
   ```bash
   # After copying files, verify they exist and have correct sizes
   ls -la /path/to/sdcard/
   # Should show:
   # config.txt (your config)
   # kernel8.img (88 bytes - your kernel)
   # start_cd.elf (~2.8MB)
   # fixup_cd.dat (~7KB)
   # bcm2712-rpi-5-b.dtb (~50KB)
   ```

### Raspberry Pi 5 Specific Debugging

**Step-by-step debugging for Pi 5:**

1. **Verify SD card contents:**
   ```bash
   # Your SD card root should contain:
   config.txt              # Boot configuration
   kernel8.img            # Your TinyOS kernel (119KB)
   start_cd.elf           # Pi firmware starter
   fixup_cd.dat          # Memory split configuration  
   bcm2712-rpi-5-b.dtb   # Device tree blob for Pi 5
   ```

2. **Test with minimal config.txt:**
   ```ini
   arm_64bit=1
   kernel=kernel8.img
   uart_2ndstage=1
   enable_uart=1
   device_tree=bcm2712-rpi-5-b.dtb
   ```

3. **Troubleshoot "boots but no output":**
   
   If the Pi 5 boots (no more FAT errors) but you see no output:
   
   **Check UART setup:**
   ```bash
   # Verify UART is enabled in config.txt
   grep enable_uart /path/to/sdcard/config.txt
   # Should show: enable_uart=1
   
   # Try alternative UART settings in config.txt:
   enable_uart=1
   uart_2ndstage=1
   dtparam=uart0=on
   ```
   
   **Test serial connection:**
   ```bash
   # Before powering Pi, test your serial adapter:
   # Connect TX to RX on the adapter (loopback test)
   # Type in terminal - you should see characters echoed back
   ```

4. **Alternative debugging - LED blink test:**
   TinyOS has LED control. If it's running, the activity LED should be controllable through the shell (commands `1` and `0`). If you can't see output but suspect it's running, the LED may still respond to input.

3. **Check kernel loading address:**
   ```bash
   # Verify kernel is linked for correct address
   readelf -h target/aarch64-unknown-none/release/tiny_os | grep "Entry point"
   # Should show: Entry point address: 0x80000
   ```

4. **Enable maximum debug output in config.txt:**
   ```ini
   arm_64bit=1
   kernel=kernel8.img
   uart_2ndstage=1
   enable_uart=1
   boot_delay=3
   disable_splash=1
   ```

5. **Alternative kernel names to try:**
   - Copy kernel as `kernel8.img` (standard)
   - Try `kernel.img` (fallback)
   - Try `kernel7l.img` (32-bit fallback)

**If still not working, try this debug sequence:**

1. **SD Card Basic Test:**
   ```bash
   # Test if SD card works with official Raspberry Pi OS first
   # Download Raspberry Pi OS Lite and flash it
   # If this doesn't boot, your SD card/reader has issues
   ```

2. **Test with a known-working kernel:**
   ```bash
   # Download a simple "hello world" Pi kernel to test firmware
   wget https://github.com/bztsrc/raspi3-tutorial/raw/master/01_bareminimum/kernel8.img
   # Copy to SD card and test - this verifies firmware setup
   ```

2. **Check TinyOS boot code:**
   ```bash
   # Verify _start symbol exists
   nm target/aarch64-unknown-none/release/tiny_os | grep _start
   ```

3. **Serial debugging setup:**
   - Use 3.3V USB-to-TTL adapter (NOT 5V!)
   - Pi 5 GPIO: Pin 8=TX, Pin 10=RX, Pin 6=GND
   - 115200 baud, 8N1
   - Connect before powering on Pi

### Optional Configuration (config.txt)
### Alternative Configuration Options
```ini
# Alternative config.txt for troubleshooting
arm_64bit=1
kernel=kernel8.img
gpu_mem=16
enable_uart=1
uart_2ndstage=1
disable_splash=1

# For Pi 5 - explicit device tree
device_tree=bcm2712-rpi-5-b.dtb

# Debug options
boot_delay=1
disable_overscan=1
```

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
│   ├── sdcard.rs         # SD card driver (EMMC interface)
│   └── tests/            # Rust unit tests
├── tests/                # Test suite
│   ├── test_*_automated.sh  # Automated test scripts (no dependencies)
│   ├── test_*_suite.sh      # Interactive test suites (require expect)
│   ├── test_qemu_boot.sh    # Boot validation
│   └── validate_tinyos.sh   # System validation
├── .cargo/
│   └── config.toml       # Cargo configuration for cross-compilation
├── linker.ld             # Custom linker script for memory layout
├── aarch64-raspi.json    # Custom target specification
├── test_tinyos.sh        # Unified test runner (at project root)
├── build.sh              # Build script  
├── run.sh                # QEMU execution script
└── DOCS.md               # Technical documentation
```

## SD Card Driver

TinyOS includes a comprehensive SD card driver that implements the EMMC (Enhanced Multi-Media Card) interface for communicating with SD/SDHC/SDXC cards on Raspberry Pi 4/5.

### Features

- **Hardware Interface**: Direct EMMC register manipulation for maximum performance
- **Card Support**: SDSC (Standard Capacity), SDHC, and SDXC cards
- **Block Operations**: 512-byte block read/write operations
- **Initialization**: Complete SD card initialization sequence (CMD0 through ACMD51)
- **Error Handling**: Comprehensive error detection and timeout management
- **Interactive Commands**: Real-time testing through the shell interface

### Technical Implementation

The SD card driver (`src/sdcard.rs`) provides:

- **EMMC Register Interface**: Direct hardware register access at `0xFE300000`
- **Command Processing**: Full SD command set implementation with proper timing
- **Clock Management**: Dynamic clock frequency adjustment (400kHz initialization, 25MHz operation)
- **GPIO Configuration**: Automatic GPIO pin setup for SD interface (pins 48-53)
- **Data Transfer**: Efficient word-based data transfer with proper synchronization

### Shell Commands

Use these commands to interact with the SD card:

- **`p`** - Display SD card information (type, capacity, manufacturer)
- **`q`** - Read and display block 0 (boot sector analysis)
- **`y`** - Write test pattern to block 1000 and verify

### QEMU Limitations

**Note**: SD card functionality is limited in QEMU emulation:
- EMMC hardware is not fully emulated in QEMU's Raspberry Pi 4 model
- SD card initialization will fail gracefully with appropriate error messages
- Commands will show "SD card not initialized" status in QEMU
- Full functionality is available on real Raspberry Pi 4/5 hardware

### Real Hardware Usage

On actual Raspberry Pi hardware with an SD card inserted:
1. The driver will detect and initialize the card automatically during boot
2. Card information (capacity, type, manufacturer) will be displayed
3. Block read/write operations will function normally
4. Boot sector analysis can identify FAT filesystems

## Development

### Available Commands

**Testing:**
- `./test_tinyos.sh` - Run all available tests
- `./test_tinyos.sh --help` - Show test options
- `./test_tinyos.sh boot` - Test boot and validation
- `./test_tinyos.sh unit` - Test Rust unit tests only
- `./test_tinyos.sh validate` - Quick validation check

**Building:**
- `./build.sh` - Build kernel
- `cargo build` - Standard Rust build
- `cargo build --release` - Release build for hardware

**Running:**
- `./run.sh` - Run in QEMU
- `./test_tinyos.sh validate` - Quick health check

### Interactive Shell Commands

Once TinyOS is running, use these commands in the interactive shell:

- `h` - Help menu with all commands
- `c` - System health check
- `m` - Memory statistics
- `i` - Interrupt status
- `v` - Exception statistics
- `w` - Test exception handling
- `s` - System information
- `t` - Current time
- `1/0` - LED on/off
- `x` - Memory test
- `j` - Interrupt test
- `p` - SD card information
- `q` - Read SD card block
- `y` - Write SD card test block

## To-Do List

### Core Features
- [x] Exception vectors implementation
- [ ] Real hardware validation on Pi 4/5
- [ ] Enhanced GPIO control capabilities
- [ ] Device driver framework
- [ ] Power management
- [ ] Watchdog timer support

### Memory Management
- [ ] Virtual memory management (MMU)
- [ ] Page table management
- [ ] Memory protection improvements
- [ ] Dynamic memory allocation optimizations
- [ ] Memory compression

### Process Management
- [ ] Multi-tasking support
- [ ] Process isolation
- [ ] Context switching optimization
- [ ] Thread support
- [ ] Process scheduler improvements

### Storage & File System
- [x] SD card driver
- [ ] FAT32 file system support
- [ ] File I/O operations
- [ ] Directory management
- [ ] Boot from file system

### Networking
- [ ] Ethernet driver
- [ ] Basic TCP/IP stack
- [ ] UDP support
- [ ] Network interface management
- [ ] DHCP client

### Advanced Features
- [ ] Multi-core support (SMP)
- [ ] Kernel/user mode separation
- [ ] System call interface
- [ ] Dynamic loading
- [ ] Implement basic HDMI video support (framebuffer graphics)
- [ ] USB support

### Development & Testing
- [ ] Automated hardware testing
- [ ] Performance benchmarking suite
- [ ] Code coverage analysis
- [ ] Continuous integration
- [ ] Documentation automation

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make changes with tests
4. Run `./test_tinyos.sh` to verify
5. Submit a pull request

## Documentation

For detailed technical documentation, architecture details, and API references, see [DOCS.md](DOCS.md).

## License

This project is open source. See LICENSE file for details.
