//! Test Configuration and Constants
//! 
//! Centralized configuration for all tests in TinyOS

pub const HEAP_START: u32 = 0x100000;
pub const HEAP_SIZE: u32 = 0x400000; // 4MB
pub const BLOCK_SIZE: u32 = 64;
pub const TOTAL_BLOCKS: u32 = HEAP_SIZE / BLOCK_SIZE;
pub const BITMAP_SIZE: u32 = (TOTAL_BLOCKS + 7) / 8;

// GPIO configuration
pub const LED_PIN: u32 = 42;
pub const GPIO_BASE: u32 = 0xFE200000;

// UART configuration
pub const UART_BASE: u32 = 0xFE201000;

// Timer configuration
pub const TIMER_BASE: u32 = 0xFE003000;

// Interrupt configuration
pub const GIC_DIST_BASE: u32 = 0xFF841000;
pub const GIC_CPU_BASE: u32 = 0xFF842000;

// Test scenarios
pub const TEST_BLOCK_COUNT: usize = 10;
pub const STRESS_TEST_ITERATIONS: usize = 100;
pub const PERFORMANCE_TEST_ITERATIONS: usize = 1000;

// Test data patterns
pub const CANARY_START: u32 = 0xDEADBEEF;
pub const CANARY_END: u32 = 0xBEEFDEAD;
pub const TEST_PATTERN_1: u32 = 0xCAFEBABE;
pub const TEST_PATTERN_2: u32 = 0x12345678;
pub const TEST_PATTERN_3: u32 = 0xABCDEF00;

// Performance thresholds
pub const MAX_ALLOCATION_TIME_US: u64 = 100;
pub const MAX_INTERRUPT_LATENCY_US: u64 = 50;
pub const MIN_UART_THROUGHPUT_BPS: u64 = 9600;
