//! ARM64 Performance Counter and Timing Module
//!
//! This module provides high-precision timing capabilities using ARM64 PMU
//! (Performance Monitoring Unit) for accurate performance measurements.

use core::arch::asm;

/// ARM64 PMU cycle counter type
pub type CycleCount = u64;

/// Timing calibration data
static mut CYCLES_PER_MICROSECOND: u64 = 1000; // Default assumption: 1GHz

/// Initialize ARM64 Performance Monitoring Unit
pub fn init_pmu() {
    unsafe {
        // Enable PMU user access (PMUSERENR_EL0)
        asm!(
            "msr PMUSERENR_EL0, {val}",
            val = in(reg) 1u64, // Enable user mode access to PMU
            options(nostack, preserves_flags)
        );

        // Enable cycle counter (PMCNTENSET_EL0)
        asm!(
            "msr PMCNTENSET_EL0, {val}",
            val = in(reg) 0x80000000u64, // Enable cycle counter (bit 31)
            options(nostack, preserves_flags)
        );

        // Enable PMU (PMCR_EL0)
        asm!(
            "mrs {val}, PMCR_EL0",
            "orr {val}, {val}, #1", // Set enable bit
            "msr PMCR_EL0, {val}",
            val = out(reg) _,
            options(nostack, preserves_flags)
        );
    }
}

/// Get current cycle count from ARM64 PMU
pub fn get_cycles() -> CycleCount {
    let cycles: u64;
    unsafe {
        asm!(
            "mrs {cycles}, PMCCNTR_EL0",
            cycles = out(reg) cycles,
            options(nostack, preserves_flags, readonly)
        );
    }
    cycles
}

/// Measure cycles for a given operation
pub fn measure_cycles<F: FnOnce() -> T, T>(operation: F) -> (T, CycleCount) {
    let start = get_cycles();
    let result = operation();
    let end = get_cycles();
    (result, end.saturating_sub(start))
}

/// Calibrate timing measurements
pub fn calibrate_timing() {
    // Perform timing calibration by measuring known delays
    let iterations = 1000;
    let start = get_cycles();

    // Perform some known work
    for _ in 0..iterations {
        core::hint::black_box(42u32.wrapping_add(1));
    }

    let end = get_cycles();
    let cycles_per_iteration = (end - start) / iterations;

    unsafe {
        // Rough calibration - this would need proper hardware measurement
        CYCLES_PER_MICROSECOND = cycles_per_iteration * 100; // Rough estimate
    }
}

/// Convert cycles to microseconds (approximate)
pub fn cycles_to_microseconds(cycles: CycleCount) -> u64 {
    unsafe {
        if CYCLES_PER_MICROSECOND > 0 {
            cycles / CYCLES_PER_MICROSECOND
        } else {
            cycles / 1000 // Fallback
        }
    }
}

/// Convert cycles to nanoseconds (approximate)
pub fn cycles_to_nanoseconds(cycles: CycleCount) -> u64 {
    cycles_to_microseconds(cycles) * 1000
}

/// High-precision delay using cycle counting
pub fn delay_cycles(cycles: CycleCount) {
    let start = get_cycles();
    while get_cycles().saturating_sub(start) < cycles {
        core::hint::spin_loop();
    }
}

/// Benchmark timing overhead
pub fn benchmark_timing_overhead() -> CycleCount {
    let start = get_cycles();
    let end = get_cycles();
    end.saturating_sub(start)
}
