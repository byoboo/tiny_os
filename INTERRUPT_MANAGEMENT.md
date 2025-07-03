# TinyOS Interrupt Management System

## Overview

TinyOS now includes a comprehensive interrupt management system that provides ARM Generic Interrupt Controller (GIC) simulation, interrupt statistics, and comprehensive testing capabilities.

## Interrupt Architecture

### ARM Generic Interrupt Controller (GIC) Simulation
- **GIC Distributor Base**: 0xFF841000
- **GIC CPU Interface Base**: 0xFF842000
- **Simulated for QEMU development** with real hardware hooks ready
- **Support for 256 interrupt sources**

### Supported Interrupt Sources
- **Timer (IRQ 64)**: System timer interrupts
- **UART (IRQ 153)**: Serial communication interrupts  
- **GPIO (IRQ 129)**: General-purpose I/O interrupts

## New Shell Commands

### Interrupt Management Commands
- **i/I** - Show interrupt status and statistics
- **e/E** - Enable all major interrupt sources
- **j/J** - Run comprehensive interrupt test

### Enhanced Existing Commands
- **c/C** - System health check (now includes interrupt testing)
- **s/S** - System information (includes interrupt statistics)
- **d/D** - Hardware diagnostics (includes interrupt information)
- **h/H** - Help menu (updated with interrupt commands)

## Command Examples

### Interrupt Status Display (`i` command)
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

### Interrupt Management (`e` command)
```
=== Interrupt Management ===
1. Enable timer interrupts
   Timer interrupts: ✓ ENABLED
2. Enable UART interrupts
   UART interrupts: ✓ ENABLED
3. Enable GPIO interrupts
   GPIO interrupts: ✓ ENABLED
All major interrupt sources enabled!
Use 'i' to check interrupt status.
============================
```

### Interrupt Testing (`j` command)
```
=== Interrupt System Test ===
Running comprehensive interrupt test...
Interrupt test: ✓ PASSED
Test Results:
  Timer interrupts: 2 simulated
  UART interrupts: 1 simulated
  GPIO interrupts: 1 simulated
All interrupt sources functioning correctly!
=============================
```

## Enhanced System Health Check

The system health check now includes interrupt testing:

```
6. Interrupt System: Running interrupt test...
   - Interrupt controller: ✓ PASS
   - Simulated interrupts: 4 total
```

## Enhanced System Information

System information now shows interrupt statistics:

```
=== TinyOS System Information ===
  OS Name: TinyOS
  Version: 0.1.0
  Platform: Raspberry Pi 4/5 (AArch64)
  Architecture: ARM64
  Timer Frequency: 1MHz
  UART Base: 0xFE201000
  GPIO Base: 0xFE200000
  GIC Base: 0xFF841000
  LED Pin: GPIO 42
  Current Uptime: [45.123s]
  Active Interrupts: 26
=================================
```

## Enhanced Hardware Diagnostics

Hardware diagnostics now include interrupt information:

```
=== Hardware Diagnostics ===
CPU: ARM Cortex-A72 (Pi 4) / A76 (Pi 5)
CPU Cores: 4 (only core 0 active)
Timer: BCM2835 System Timer @ 1MHz
UART: PL011 UART
GPIO: BCM2835 GPIO Controller
GIC: ARM Generic Interrupt Controller
Interrupts: 3 sources enabled, 26 total
Current Time: [45.123s]
============================
```

## Implementation Features

### Interrupt Controller Capabilities
- **Enable/Disable**: Individual interrupt source control
- **Statistics Tracking**: Real-time interrupt counting
- **Testing Framework**: Comprehensive interrupt simulation
- **Status Monitoring**: Detailed interrupt status reporting

### Interrupt Statistics
- **Per-source counters**: Individual interrupt type tracking
- **Total interrupt count**: System-wide interrupt statistics
- **Enable status tracking**: Real-time enable/disable state
- **Test result tracking**: Interrupt test pass/fail history

### Development Features
- **QEMU Simulation**: Full interrupt simulation for development
- **Real Hardware Ready**: GIC register definitions for real Pi
- **Comprehensive Testing**: Multi-source interrupt validation
- **Debug Interface**: Detailed status and statistics reporting

## Technical Implementation

### Interrupt Controller Structure
```rust
pub struct InterruptController {
    enabled_interrupts: u32,
    interrupt_count: [u32; 256],
    timer_enabled: bool,
    uart_enabled: bool,
    gpio_enabled: bool,
}
```

### Key Methods
- `enable_interrupt(irq)` - Enable specific interrupt source
- `disable_interrupt(irq)` - Disable specific interrupt source  
- `simulate_interrupt(irq)` - Simulate interrupt for testing
- `get_interrupt_stats()` - Get comprehensive statistics
- `run_interrupt_test()` - Execute full interrupt test suite

### Interrupt Handler Framework
```rust
// Interrupt handler functions (ready for assembly integration)
extern "C" fn timer_interrupt_handler();
extern "C" fn uart_interrupt_handler(); 
extern "C" fn gpio_interrupt_handler();
```

## Future Enhancements

### Real Hardware Integration
- **GIC Register Programming**: Full hardware GIC configuration
- **Exception Vector Table**: Assembly interrupt routing
- **Context Switching**: Process state preservation
- **Nested Interrupts**: Priority-based interrupt handling

### Advanced Features
- **Interrupt Priorities**: Priority-based interrupt handling
- **Software Interrupts**: Inter-core communication
- **DMA Interrupts**: Direct memory access interrupt support
- **Performance Monitoring**: Interrupt latency tracking

### Multi-Core Support
- **SMP Interrupt Distribution**: Multi-core interrupt routing
- **Core-specific Handlers**: Per-core interrupt management
- **Load Balancing**: Interrupt distribution optimization
- **Synchronization**: Multi-core interrupt coordination

## Building and Testing

```bash
# Build with interrupt support
cargo build --target aarch64-unknown-none --release

# Run with interrupt features
./run.sh

# Test interrupt management specifically
./test_interrupts.sh
```

## Integration with Other Subsystems

### Timer Integration
- Timer interrupts eliminate polling delays
- Precise timing with interrupt-driven updates
- System tick generation for scheduling

### UART Integration  
- Interrupt-driven input/output
- Elimination of polling loops
- Improved responsiveness

### GPIO Integration
- Pin change interrupts
- Hardware event detection
- Real-time GPIO monitoring

The interrupt management system provides a solid foundation for advanced OS features like process scheduling, real-time response, and efficient hardware interaction. It's designed to scale from QEMU development to full Pi hardware deployment.
