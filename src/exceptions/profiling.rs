//! Exception-based Performance Profiling
//!
//! Uses ARM64 exception handling infrastructure for advanced performance
//! monitoring. This module leverages our exception system to create
//! sophisticated benchmarking capabilities that can measure performance at a
//! much deeper level.

use crate::{drivers::uart::Uart, exceptions::types::ExceptionType};

/// Performance profiling statistics
#[derive(Debug, Clone, Copy)]
pub struct ProfileStats {
    pub sync_exceptions: u64,
    pub irq_exceptions: u64,
    pub fiq_exceptions: u64,
    pub serror_exceptions: u64,
    pub total_cycles: u64,
    pub context_switch_cycles: u64,
}

impl ProfileStats {
    pub const fn new() -> Self {
        Self {
            sync_exceptions: 0,
            irq_exceptions: 0,
            fiq_exceptions: 0,
            serror_exceptions: 0,
            total_cycles: 0,
            context_switch_cycles: 0,
        }
    }
}

/// Global profiling statistics
static mut PROFILE_STATS: ProfileStats = ProfileStats::new();

/// Enable performance profiling using exceptions
pub fn enable_exception_profiling() {
    // Enable PMU for exception-based profiling
    unsafe {
        // Enable user-space access to performance counters
        core::arch::asm!(
            "mrs x0, pmuserenr_el0",
            "orr x0, x0, #1",
            "msr pmuserenr_el0, x0",
            out("x0") _,
        );

        // Enable cycle counter
        core::arch::asm!(
            "mrs x0, pmcntenset_el0",
            "orr x0, x0, #0x80000000",
            "msr pmcntenset_el0, x0",
            out("x0") _,
        );
    }
}

/// Record exception performance data
pub fn record_exception_performance(exception_type: ExceptionType, cycles: u64) {
    unsafe {
        let stats = &mut PROFILE_STATS;
        stats.total_cycles = stats.total_cycles.saturating_add(cycles);

        match exception_type {
            ExceptionType::Synchronous => stats.sync_exceptions += 1,
            ExceptionType::Irq => stats.irq_exceptions += 1,
            ExceptionType::Fiq => stats.fiq_exceptions += 1,
            ExceptionType::SError => stats.serror_exceptions += 1,
        }
    }
}

/// Measure context switch performance
pub fn measure_context_switch() -> u64 {
    let start_cycles = read_cycle_counter();

    // Simulate context switch by saving/restoring minimal context
    unsafe {
        core::arch::asm!(
            // Save some registers
            "stp x0, x1, [sp, #-16]!",
            "stp x2, x3, [sp, #-16]!",

            // Simulate some context switch work
            "mov x0, #100",
            "1: subs x0, x0, #1",
            "bne 1b",

            // Restore registers
            "ldp x2, x3, [sp], #16",
            "ldp x0, x1, [sp], #16",
            out("x0") _,
        );
    }

    let end_cycles = read_cycle_counter();
    let switch_cycles = end_cycles.saturating_sub(start_cycles);

    unsafe {
        PROFILE_STATS.context_switch_cycles = switch_cycles;
    }

    switch_cycles
}

/// Read ARM64 cycle counter
fn read_cycle_counter() -> u64 {
    let mut cycles: u64;
    unsafe {
        core::arch::asm!(
            "mrs {}, pmccntr_el0",
            out(reg) cycles,
        );
    }
    cycles
}

/// Performance test using exception system
pub fn test_exception_performance() {
    let mut uart = Uart::new();

    uart.puts("🔬 Exception-based Performance Profiling\r\n");
    uart.puts("==========================================\r\n");

    // Enable profiling
    enable_exception_profiling();

    // Test 1: Context switch performance
    uart.puts("📊 Context Switch Performance:\r\n");
    let switch_cycles = measure_context_switch();
    uart.puts("  Context switch: ");
    print_number(&mut uart, switch_cycles);
    uart.puts(" cycles\r\n");

    // Test 2: Exception overhead measurement
    uart.puts("📊 Exception System Overhead:\r\n");
    let start = read_cycle_counter();

    // Simulate some work that might trigger exceptions
    for i in 0..10 {
        let _dummy = i * 42;
    }

    let end = read_cycle_counter();
    let overhead = end.saturating_sub(start);
    uart.puts("  Loop overhead: ");
    print_number(&mut uart, overhead);
    uart.puts(" cycles\r\n");

    // Display current stats
    unsafe {
        let stats = &PROFILE_STATS;
        uart.puts("📈 Profiling Statistics:\r\n");
        uart.puts("  Total cycles measured: ");
        print_number(&mut uart, stats.total_cycles);
        uart.puts("\r\n");
        uart.puts("  Sync exceptions: ");
        print_number(&mut uart, stats.sync_exceptions);
        uart.puts("\r\n");
        uart.puts("  IRQ exceptions: ");
        print_number(&mut uart, stats.irq_exceptions);
        uart.puts("\r\n");
    }

    uart.puts("✅ Exception profiling complete\r\n");
}

/// Helper function to print numbers
fn print_number(uart: &mut Uart, mut num: u64) {
    if num == 0 {
        uart.puts("0");
        return;
    }

    let mut buffer = [0u8; 20];
    let mut i = 0;

    while num > 0 {
        buffer[i] = (num % 10) as u8 + b'0';
        num /= 10;
        i += 1;
    }

    // Print in reverse order
    while i > 0 {
        i -= 1;
        uart.putc(buffer[i]);
    }
}

/// Initialize exception-based profiling
pub fn init_exception_profiling() {
    enable_exception_profiling();
}
