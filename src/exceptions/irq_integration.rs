//! IRQ Controller Integration for TinyOS Phase 2
//!
//! This module integrates the exception system with the existing interrupt
//! controller, providing proper IRQ routing and acknowledgment.

use spin::Mutex;

use super::types::{ExceptionContext, ExceptionLevel, ExceptionStats, ExceptionType};
use crate::{interrupts::InterruptController, uart::Uart};

/// IRQ source identification
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IrqSource {
    /// System Timer interrupt
    Timer = 64,
    /// UART PL011 interrupt
    Uart = 153,
    /// GPIO interrupt
    Gpio = 129,
    /// Unknown interrupt source
    Unknown = 0xFFFF,
}

impl From<u32> for IrqSource {
    fn from(value: u32) -> Self {
        match value {
            64 => IrqSource::Timer,
            153 => IrqSource::Uart,
            129 => IrqSource::Gpio,
            _ => IrqSource::Unknown,
        }
    }
}

/// IRQ routing information
#[derive(Debug, Clone, Copy)]
pub struct IrqInfo {
    /// Source of the interrupt
    pub source: IrqSource,
    /// Interrupt ID
    pub interrupt_id: u32,
    /// Priority level (0-255, lower number = higher priority)
    pub priority: u8,
    /// Whether the interrupt is valid
    pub is_valid: bool,
}

impl IrqInfo {
    pub fn new(interrupt_id: u32, priority: u8) -> Self {
        Self {
            source: IrqSource::from(interrupt_id),
            interrupt_id,
            priority,
            is_valid: true,
        }
    }

    pub fn invalid() -> Self {
        Self {
            source: IrqSource::Unknown,
            interrupt_id: 0,
            priority: 255,
            is_valid: false,
        }
    }
}

/// IRQ controller integration
pub struct IrqControllerIntegration {
    /// Reference to the main interrupt controller
    interrupt_controller: Option<*mut InterruptController>,
    /// IRQ statistics
    irq_stats: IrqStats,
}

// SAFETY: In a bare-metal environment, we don't have actual threads,
// so the raw pointer is safe to share across "threads" (interrupt contexts)
unsafe impl Send for IrqControllerIntegration {}
unsafe impl Sync for IrqControllerIntegration {}

impl IrqControllerIntegration {
    pub const fn new() -> Self {
        Self {
            interrupt_controller: None,
            irq_stats: IrqStats::new(),
        }
    }

    /// Initialize the IRQ controller integration
    pub fn init(&mut self, interrupt_controller: *mut InterruptController) {
        self.interrupt_controller = Some(interrupt_controller);

        let mut uart = Uart::new();
        uart.init();
        uart.puts("IRQ controller integration initialized\r\n");
    }

    /// Handle an IRQ exception
    pub fn handle_irq(&mut self, ctx: &mut ExceptionContext) -> IrqInfo {
        // Update exception statistics
        ExceptionStats::record_exception_occurrence(
            ExceptionType::Irq,
            ExceptionLevel::CurrentSpElx,
        );

        // Read the interrupt acknowledge register to get the interrupt ID
        let interrupt_id = self.read_interrupt_acknowledge();

        if interrupt_id == 0x3FF {
            // Spurious interrupt
            return IrqInfo::invalid();
        }

        let irq_info = IrqInfo::new(interrupt_id, self.get_interrupt_priority(interrupt_id));

        // Update IRQ statistics
        self.irq_stats.record_irq(irq_info.source);

        // Route to appropriate handler
        match irq_info.source {
            IrqSource::Timer => self.handle_timer_irq(ctx, &irq_info),
            IrqSource::Uart => self.handle_uart_irq(ctx, &irq_info),
            IrqSource::Gpio => self.handle_gpio_irq(ctx, &irq_info),
            IrqSource::Unknown => self.handle_unknown_irq(ctx, &irq_info),
        }

        // Acknowledge the interrupt
        self.acknowledge_interrupt(interrupt_id);

        irq_info
    }

    /// Read the interrupt acknowledge register
    fn read_interrupt_acknowledge(&self) -> u32 {
        // GIC CPU Interface - Interrupt Acknowledge Register
        const GICC_IAR: u32 = 0xFF842000 + 0x00C;

        unsafe { core::ptr::read_volatile(GICC_IAR as *const u32) }
    }

    /// Get interrupt priority
    fn get_interrupt_priority(&self, interrupt_id: u32) -> u8 {
        // GIC Distributor - Interrupt Priority Register
        const GICD_IPRIORITYR: u32 = 0xFF841000 + 0x400;

        let reg_offset = (interrupt_id / 4) * 4;
        let byte_offset = interrupt_id % 4;

        unsafe {
            let reg_value = core::ptr::read_volatile((GICD_IPRIORITYR + reg_offset) as *const u32);
            ((reg_value >> (byte_offset * 8)) & 0xFF) as u8
        }
    }

    /// Acknowledge an interrupt
    fn acknowledge_interrupt(&self, interrupt_id: u32) {
        // GIC CPU Interface - End of Interrupt Register
        const GICC_EOIR: u32 = 0xFF842000 + 0x010;

        unsafe {
            core::ptr::write_volatile(GICC_EOIR as *mut u32, interrupt_id);
        }
    }

    /// Handle timer interrupt
    fn handle_timer_irq(&mut self, _ctx: &mut ExceptionContext, irq_info: &IrqInfo) {
        let mut uart = Uart::new();
        uart.init();
        uart.puts("Timer IRQ handled (ID: ");
        uart.put_hex(irq_info.interrupt_id as u64);
        uart.puts(")\r\n");

        // TODO: Call timer driver's IRQ handler
    }

    /// Handle UART interrupt
    fn handle_uart_irq(&mut self, _ctx: &mut ExceptionContext, irq_info: &IrqInfo) {
        let mut uart = Uart::new();
        uart.init();
        uart.puts("UART IRQ handled (ID: ");
        uart.put_hex(irq_info.interrupt_id as u64);
        uart.puts(")\r\n");

        // TODO: Call UART driver's IRQ handler
    }

    /// Handle GPIO interrupt
    fn handle_gpio_irq(&mut self, _ctx: &mut ExceptionContext, irq_info: &IrqInfo) {
        let mut uart = Uart::new();
        uart.init();
        uart.puts("GPIO IRQ handled (ID: ");
        uart.put_hex(irq_info.interrupt_id as u64);
        uart.puts(")\r\n");

        // TODO: Call GPIO driver's IRQ handler
    }

    /// Handle unknown interrupt
    fn handle_unknown_irq(&mut self, _ctx: &mut ExceptionContext, irq_info: &IrqInfo) {
        let mut uart = Uart::new();
        uart.init();
        uart.puts("Unknown IRQ handled (ID: ");
        uart.put_hex(irq_info.interrupt_id as u64);
        uart.puts(")\r\n");
    }

    /// Get IRQ statistics
    pub fn get_stats(&self) -> IrqStats {
        self.irq_stats
    }
}

/// IRQ statistics
#[derive(Debug, Clone, Copy)]
pub struct IrqStats {
    pub total_irqs: u64,
    pub timer_irqs: u64,
    pub uart_irqs: u64,
    pub gpio_irqs: u64,
    pub unknown_irqs: u64,
    pub spurious_irqs: u64,
}

impl IrqStats {
    pub const fn new() -> Self {
        Self {
            total_irqs: 0,
            timer_irqs: 0,
            uart_irqs: 0,
            gpio_irqs: 0,
            unknown_irqs: 0,
            spurious_irqs: 0,
        }
    }

    pub fn record_irq(&mut self, source: IrqSource) {
        self.total_irqs += 1;

        match source {
            IrqSource::Timer => self.timer_irqs += 1,
            IrqSource::Uart => self.uart_irqs += 1,
            IrqSource::Gpio => self.gpio_irqs += 1,
            IrqSource::Unknown => self.unknown_irqs += 1,
        }
    }

    pub fn record_spurious(&mut self) {
        self.spurious_irqs += 1;
    }
}

/// Global IRQ controller integration instance
static IRQ_CONTROLLER: Mutex<IrqControllerIntegration> =
    Mutex::new(IrqControllerIntegration::new());

/// Initialize IRQ controller integration
pub fn init_irq_integration(interrupt_controller: *mut InterruptController) {
    IRQ_CONTROLLER.lock().init(interrupt_controller);
}

/// Handle IRQ from exception handler
pub fn handle_irq_integration(ctx: &mut ExceptionContext) -> IrqInfo {
    IRQ_CONTROLLER.lock().handle_irq(ctx)
}

/// Get IRQ statistics
pub fn get_irq_stats() -> IrqStats {
    IRQ_CONTROLLER.lock().get_stats()
}

/// Test IRQ controller integration
pub fn test_irq_integration() -> bool {
    let mut uart = Uart::new();
    uart.init();
    uart.puts("Testing IRQ controller integration...\r\n");

    // Test IRQ info creation
    let irq_info = IrqInfo::new(64, 128);
    if irq_info.source != IrqSource::Timer {
        uart.puts("❌ IRQ source identification failed\r\n");
        return false;
    }

    // Test IRQ statistics
    let stats = get_irq_stats();
    uart.puts("IRQ stats initialized: ");
    uart.put_hex(stats.total_irqs);
    uart.puts("\r\n");

    uart.puts("✅ IRQ controller integration tests passed\r\n");
    true
}
