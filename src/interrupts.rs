// Interrupt handling for TinyOS

// ARM Generic Interrupt Controller (GIC) addresses for Raspberry Pi 4
#[allow(dead_code)]
const GIC_DIST_BASE: u32 = 0xFF841000; // GIC Distributor
#[allow(dead_code)]
const GIC_CPU_BASE: u32 = 0xFF842000; // GIC CPU Interface

// GIC Distributor registers
#[allow(dead_code, clippy::identity_op)]
const GICD_CTLR: u32 = GIC_DIST_BASE + 0x000; // Distributor Control
#[allow(dead_code)]
const GICD_TYPER: u32 = GIC_DIST_BASE + 0x004; // Interrupt Controller Type
#[allow(dead_code)]
const GICD_ISENABLER: u32 = GIC_DIST_BASE + 0x100; // Interrupt Set-Enable
#[allow(dead_code)]
const GICD_ICENABLER: u32 = GIC_DIST_BASE + 0x180; // Interrupt Clear-Enable
#[allow(dead_code)]
const GICD_IPRIORITYR: u32 = GIC_DIST_BASE + 0x400; // Interrupt Priority
#[allow(dead_code)]
const GICD_ITARGETSR: u32 = GIC_DIST_BASE + 0x800; // Interrupt Processor Targets

// GIC CPU Interface registers
#[allow(dead_code, clippy::identity_op)]
const GICC_CTLR: u32 = GIC_CPU_BASE + 0x000; // CPU Interface Control
#[allow(dead_code)]
const GICC_PMR: u32 = GIC_CPU_BASE + 0x004; // Interrupt Priority Mask
#[allow(dead_code)]
const GICC_IAR: u32 = GIC_CPU_BASE + 0x00C; // Interrupt Acknowledge
#[allow(dead_code)]
const GICC_EOIR: u32 = GIC_CPU_BASE + 0x010; // End of Interrupt

// Interrupt IDs for Raspberry Pi 4
const IRQ_TIMER: u32 = 64; // System Timer
const IRQ_UART: u32 = 153; // UART PL011
const IRQ_GPIO: u32 = 129; // GPIO interrupts

// Interrupt vector table size
const MAX_INTERRUPTS: usize = 256;

pub struct InterruptController {
    enabled_interrupts: u32,
    interrupt_count: [u32; MAX_INTERRUPTS],
    timer_enabled: bool,
    uart_enabled: bool,
    gpio_enabled: bool,
}

impl InterruptController {
    pub fn new() -> Self {
        Self {
            enabled_interrupts: 0,
            interrupt_count: [0; MAX_INTERRUPTS],
            timer_enabled: false,
            uart_enabled: false,
            gpio_enabled: false,
        }
    }

    /// Initialize the GIC (Generic Interrupt Controller)
    pub fn init(&mut self) {
        // For QEMU, we'll do basic initialization
        // On real hardware, this would configure the actual GIC

        // Initialize interrupt counters
        self.interrupt_count = [0; MAX_INTERRUPTS];
        self.enabled_interrupts = 0;

        // Note: Full GIC initialization would be needed for real hardware
        // For now, we'll simulate interrupt capabilities
    }

    /// Enable a specific interrupt
    pub fn enable_interrupt(&mut self, irq: u32) -> bool {
        if irq as usize >= MAX_INTERRUPTS {
            return false;
        }

        match irq {
            IRQ_TIMER => {
                self.timer_enabled = true;
                self.enabled_interrupts |= 1 << 0;
            }
            IRQ_UART => {
                self.uart_enabled = true;
                self.enabled_interrupts |= 1 << 1;
            }
            IRQ_GPIO => {
                self.gpio_enabled = true;
                self.enabled_interrupts |= 1 << 2;
            }
            _ => return false,
        }

        // On real hardware, would write to GIC registers:
        // unsafe {
        //     let reg = GICD_ISENABLER + (irq / 32) * 4;
        //     let bit = irq % 32;
        //     let current = read_volatile(reg as *const u32);
        //     write_volatile(reg as *mut u32, current | (1 << bit));
        // }

        true
    }

    /// Disable a specific interrupt
    #[allow(dead_code)]
    pub fn disable_interrupt(&mut self, irq: u32) -> bool {
        if irq as usize >= MAX_INTERRUPTS {
            return false;
        }

        match irq {
            IRQ_TIMER => {
                self.timer_enabled = false;
                self.enabled_interrupts &= !(1 << 0);
            }
            IRQ_UART => {
                self.uart_enabled = false;
                self.enabled_interrupts &= !(1 << 1);
            }
            IRQ_GPIO => {
                self.gpio_enabled = false;
                self.enabled_interrupts &= !(1 << 2);
            }
            _ => return false,
        }

        true
    }

    /// Check if an interrupt is enabled
    pub fn is_interrupt_enabled(&self, irq: u32) -> bool {
        match irq {
            IRQ_TIMER => self.timer_enabled,
            IRQ_UART => self.uart_enabled,
            IRQ_GPIO => self.gpio_enabled,
            _ => false,
        }
    }

    /// Simulate interrupt occurrence (for QEMU testing)
    pub fn simulate_interrupt(&mut self, irq: u32) {
        if (irq as usize) < MAX_INTERRUPTS && self.is_interrupt_enabled(irq) {
            self.interrupt_count[irq as usize] += 1;
        }
    }

    /// Get interrupt statistics
    pub fn get_interrupt_stats(&self) -> InterruptStats {
        InterruptStats {
            enabled_interrupts: self.enabled_interrupts,
            timer_enabled: self.timer_enabled,
            uart_enabled: self.uart_enabled,
            gpio_enabled: self.gpio_enabled,
            timer_count: self.interrupt_count[IRQ_TIMER as usize],
            uart_count: self.interrupt_count[IRQ_UART as usize],
            gpio_count: self.interrupt_count[IRQ_GPIO as usize],
            total_interrupts: self.interrupt_count.iter().sum(),
        }
    }

    /// Enable all supported interrupts
    pub fn enable_all(&mut self) {
        self.enable_interrupt(IRQ_TIMER);
        self.enable_interrupt(IRQ_UART);
        self.enable_interrupt(IRQ_GPIO);
    }

    /// Disable all interrupts
    #[allow(dead_code)]
    pub fn disable_all(&mut self) {
        self.disable_interrupt(IRQ_TIMER);
        self.disable_interrupt(IRQ_UART);
        self.disable_interrupt(IRQ_GPIO);
    }

    /// Test interrupt system by simulating interrupts
    pub fn run_interrupt_test(&mut self) -> bool {
        let initial_timer = self.interrupt_count[IRQ_TIMER as usize];
        let initial_uart = self.interrupt_count[IRQ_UART as usize];
        let initial_gpio = self.interrupt_count[IRQ_GPIO as usize];

        // Enable all interrupts
        self.enable_all();

        // Simulate some interrupts
        self.simulate_interrupt(IRQ_TIMER);
        self.simulate_interrupt(IRQ_TIMER);
        self.simulate_interrupt(IRQ_UART);
        self.simulate_interrupt(IRQ_GPIO);

        // Check if interrupts were counted
        let timer_increased = self.interrupt_count[IRQ_TIMER as usize] > initial_timer;
        let uart_increased = self.interrupt_count[IRQ_UART as usize] > initial_uart;
        let gpio_increased = self.interrupt_count[IRQ_GPIO as usize] > initial_gpio;

        timer_increased && uart_increased && gpio_increased
    }

    /// Reset interrupt counters
    #[allow(dead_code)]
    pub fn reset_counters(&mut self) {
        self.interrupt_count = [0; MAX_INTERRUPTS];
    }
}

#[derive(Debug)]
pub struct InterruptStats {
    pub enabled_interrupts: u32,
    pub timer_enabled: bool,
    pub uart_enabled: bool,
    pub gpio_enabled: bool,
    pub timer_count: u32,
    pub uart_count: u32,
    pub gpio_count: u32,
    pub total_interrupts: u32,
}

// Interrupt handler functions (would be called from assembly)
#[no_mangle]
pub extern "C" fn timer_interrupt_handler() {
    // Timer interrupt handling would go here
    // For now, just a placeholder
}

#[no_mangle]
pub extern "C" fn uart_interrupt_handler() {
    // UART interrupt handling would go here
    // For now, just a placeholder
}

#[no_mangle]
pub extern "C" fn gpio_interrupt_handler() {
    // GPIO interrupt handling would go here
    // For now, just a placeholder
}

// Constants for interrupt IDs (public interface)
pub const TIMER_IRQ: u32 = IRQ_TIMER;
pub const UART_IRQ: u32 = IRQ_UART;
pub const GPIO_IRQ: u32 = IRQ_GPIO;
