//! Mock implementations for TinyOS testing
//! 
//! Provides mock drivers and hardware interfaces for testing without real hardware.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Mock UART driver for testing
pub struct MockUart {
    pub write_buffer: Arc<Mutex<Vec<u8>>>,
    pub read_buffer: Arc<Mutex<Vec<u8>>>,
    pub enabled: bool,
}

impl MockUart {
    pub fn new() -> Self {
        Self {
            write_buffer: Arc::new(Mutex::new(Vec::new())),
            read_buffer: Arc::new(Mutex::new(Vec::new())),
            enabled: true,
        }
    }

    pub fn write_byte(&mut self, byte: u8) -> Result<(), &'static str> {
        if !self.enabled {
            return Err("UART not enabled");
        }
        
        if let Ok(mut buffer) = self.write_buffer.lock() {
            buffer.push(byte);
            Ok(())
        } else {
            Err("Failed to lock write buffer")
        }
    }

    pub fn write_string(&mut self, s: &str) -> Result<(), &'static str> {
        for byte in s.bytes() {
            self.write_byte(byte)?;
        }
        Ok(())
    }

    pub fn read_byte(&mut self) -> Option<u8> {
        if !self.enabled {
            return None;
        }
        
        if let Ok(mut buffer) = self.read_buffer.lock() {
            if !buffer.is_empty() {
                Some(buffer.remove(0))
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn add_input(&mut self, data: &[u8]) {
        if let Ok(mut buffer) = self.read_buffer.lock() {
            buffer.extend_from_slice(data);
        }
    }

    pub fn get_output(&self) -> Vec<u8> {
        if let Ok(buffer) = self.write_buffer.lock() {
            buffer.clone()
        } else {
            Vec::new()
        }
    }

    pub fn get_output_string(&self) -> String {
        String::from_utf8_lossy(&self.get_output()).to_string()
    }

    pub fn clear_buffers(&mut self) {
        if let Ok(mut write_buf) = self.write_buffer.lock() {
            write_buf.clear();
        }
        if let Ok(mut read_buf) = self.read_buffer.lock() {
            read_buf.clear();
        }
    }
}

/// Mock GPIO driver for testing
pub struct MockGpio {
    pub pin_states: HashMap<u32, bool>,
    pub pin_modes: HashMap<u32, GpioMode>,
    pub enabled: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum GpioMode {
    Input,
    Output,
    Alt0,
    Alt1,
    Alt2,
    Alt3,
    Alt4,
    Alt5,
}

impl MockGpio {
    pub fn new() -> Self {
        Self {
            pin_states: HashMap::new(),
            pin_modes: HashMap::new(),
            enabled: true,
        }
    }

    pub fn set_pin_mode(&mut self, pin: u32, mode: GpioMode) -> Result<(), &'static str> {
        if !self.enabled {
            return Err("GPIO not enabled");
        }
        
        if pin > 53 {
            return Err("Invalid GPIO pin");
        }
        
        self.pin_modes.insert(pin, mode);
        Ok(())
    }

    pub fn set_pin(&mut self, pin: u32, state: bool) -> Result<(), &'static str> {
        if !self.enabled {
            return Err("GPIO not enabled");
        }
        
        if pin > 53 {
            return Err("Invalid GPIO pin");
        }
        
        // Check if pin is configured as output
        if let Some(mode) = self.pin_modes.get(&pin) {
            if *mode != GpioMode::Output {
                return Err("Pin not configured as output");
            }
        }
        
        self.pin_states.insert(pin, state);
        Ok(())
    }

    pub fn get_pin(&self, pin: u32) -> Option<bool> {
        if !self.enabled || pin > 53 {
            return None;
        }
        
        self.pin_states.get(&pin).copied()
    }

    pub fn toggle_pin(&mut self, pin: u32) -> Result<bool, &'static str> {
        let current_state = self.get_pin(pin).unwrap_or(false);
        let new_state = !current_state;
        self.set_pin(pin, new_state)?;
        Ok(new_state)
    }
}

/// Mock Timer driver for testing
pub struct MockTimer {
    pub current_time: u64,
    pub enabled: bool,
    pub interrupts_enabled: bool,
    pub interrupt_count: u32,
}

impl MockTimer {
    pub fn new() -> Self {
        Self {
            current_time: 0,
            enabled: true,
            interrupts_enabled: false,
            interrupt_count: 0,
        }
    }

    pub fn get_time(&self) -> u64 {
        if self.enabled {
            self.current_time
        } else {
            0
        }
    }

    pub fn advance_time(&mut self, microseconds: u64) {
        if self.enabled {
            self.current_time += microseconds;
            
            // Simulate timer interrupt every 1000 microseconds
            if self.interrupts_enabled && self.current_time % 1000 == 0 {
                self.interrupt_count += 1;
            }
        }
    }

    pub fn delay(&mut self, microseconds: u64) {
        self.advance_time(microseconds);
    }

    pub fn enable_interrupts(&mut self) {
        self.interrupts_enabled = true;
    }

    pub fn disable_interrupts(&mut self) {
        self.interrupts_enabled = false;
    }

    pub fn get_interrupt_count(&self) -> u32 {
        self.interrupt_count
    }

    pub fn reset(&mut self) {
        self.current_time = 0;
        self.interrupt_count = 0;
        self.interrupts_enabled = false;
    }
}

/// Mock Memory Manager for testing
pub struct MockMemoryManager {
    pub heap_start: usize,
    pub heap_size: usize,
    pub block_size: usize,
    pub allocated_blocks: HashMap<usize, usize>,
    pub corruption_detected: bool,
    pub fragmentation_level: f32,
}

impl MockMemoryManager {
    pub fn new(heap_start: usize, heap_size: usize, block_size: usize) -> Self {
        Self {
            heap_start,
            heap_size,
            block_size,
            allocated_blocks: HashMap::new(),
            corruption_detected: false,
            fragmentation_level: 0.0,
        }
    }

    pub fn allocate(&mut self, size: usize) -> Option<usize> {
        if size == 0 || size > self.heap_size {
            return None;
        }
        
        let blocks_needed = (size + self.block_size - 1) / self.block_size;
        let address = self.heap_start + (self.allocated_blocks.len() * self.block_size);
        
        // Simulate allocation failure if too many blocks
        if self.allocated_blocks.len() + blocks_needed > self.heap_size / self.block_size {
            return None;
        }
        
        self.allocated_blocks.insert(address, blocks_needed);
        
        // Update fragmentation
        self.update_fragmentation();
        
        Some(address)
    }

    pub fn free(&mut self, address: usize) -> bool {
        if self.allocated_blocks.remove(&address).is_some() {
            self.update_fragmentation();
            true
        } else {
            false
        }
    }

    pub fn get_stats(&self) -> MemoryStats {
        let total_blocks = self.heap_size / self.block_size;
        let used_blocks: usize = self.allocated_blocks.values().sum();
        let free_blocks = total_blocks - used_blocks;
        
        MemoryStats {
            total_size: self.heap_size,
            used_size: used_blocks * self.block_size,
            free_size: free_blocks * self.block_size,
            fragmentation: self.fragmentation_level,
            largest_free_block: free_blocks * self.block_size,
            allocation_count: self.allocated_blocks.len(),
        }
    }

    pub fn check_corruption(&mut self) -> bool {
        // Simulate corruption detection
        self.corruption_detected = false; // Always clean in mock
        !self.corruption_detected
    }

    fn update_fragmentation(&mut self) {
        // Simple fragmentation calculation for mock
        if self.allocated_blocks.is_empty() {
            self.fragmentation_level = 0.0;
        } else {
            self.fragmentation_level = (self.allocated_blocks.len() as f32) / 100.0;
        }
    }

    pub fn defragment(&mut self) -> usize {
        // Simulate defragmentation
        let freed_blocks = (self.fragmentation_level * 10.0) as usize;
        self.fragmentation_level = 0.0;
        freed_blocks
    }
}

#[derive(Debug, Clone)]
pub struct MemoryStats {
    pub total_size: usize,
    pub used_size: usize,
    pub free_size: usize,
    pub fragmentation: f32,
    pub largest_free_block: usize,
    pub allocation_count: usize,
}

/// Mock Interrupt Controller for testing
pub struct MockInterruptController {
    pub enabled_interrupts: HashMap<u32, bool>,
    pub interrupt_counts: HashMap<u32, u32>,
    pub pending_interrupts: Vec<u32>,
    pub controller_enabled: bool,
}

impl MockInterruptController {
    pub fn new() -> Self {
        Self {
            enabled_interrupts: HashMap::new(),
            interrupt_counts: HashMap::new(),
            pending_interrupts: Vec::new(),
            controller_enabled: true,
        }
    }

    pub fn enable_interrupt(&mut self, irq: u32) -> Result<(), &'static str> {
        if !self.controller_enabled {
            return Err("Interrupt controller not enabled");
        }
        
        self.enabled_interrupts.insert(irq, true);
        Ok(())
    }

    pub fn disable_interrupt(&mut self, irq: u32) -> Result<(), &'static str> {
        if !self.controller_enabled {
            return Err("Interrupt controller not enabled");
        }
        
        self.enabled_interrupts.insert(irq, false);
        Ok(())
    }

    pub fn is_enabled(&self, irq: u32) -> bool {
        self.enabled_interrupts.get(&irq).copied().unwrap_or(false)
    }

    pub fn trigger_interrupt(&mut self, irq: u32) {
        if self.is_enabled(irq) {
            self.pending_interrupts.push(irq);
            *self.interrupt_counts.entry(irq).or_insert(0) += 1;
        }
    }

    pub fn get_pending_interrupt(&mut self) -> Option<u32> {
        self.pending_interrupts.pop()
    }

    pub fn get_interrupt_count(&self, irq: u32) -> u32 {
        self.interrupt_counts.get(&irq).copied().unwrap_or(0)
    }

    pub fn get_total_interrupts(&self) -> u32 {
        self.interrupt_counts.values().sum()
    }

    pub fn reset_statistics(&mut self) {
        self.interrupt_counts.clear();
        self.pending_interrupts.clear();
    }
}

/// Mock SD Card driver for testing
pub struct MockSdCard {
    pub initialized: bool,
    pub card_info: Option<MockSdCardInfo>,
    pub storage: HashMap<u32, [u8; 512]>,  // Block number -> data
    pub simulate_errors: bool,
}

impl MockSdCard {
    pub fn new() -> Self {
        Self {
            initialized: false,
            card_info: None,
            storage: HashMap::new(),
            simulate_errors: false,
        }
    }

    pub fn new_initialized() -> Self {
        let mut card = Self::new();
        card.initialized = true;
        card.card_info = Some(MockSdCardInfo::new());
        card
    }

    pub fn is_initialized(&self) -> bool {
        self.initialized
    }

    pub fn get_card_info(&self) -> Option<&MockSdCardInfo> {
        self.card_info.as_ref()
    }

    pub fn read_block(&self, block_addr: u32, buffer: &mut [u8; 512]) -> Result<(), MockSdError> {
        if !self.initialized {
            return Err(MockSdError::CardNotPresent);
        }

        if self.simulate_errors && block_addr % 100 == 99 {
            return Err(MockSdError::ReadError);
        }

        if let Some(data) = self.storage.get(&block_addr) {
            buffer.copy_from_slice(data);
        } else {
            // Return zeros for unwritten blocks
            buffer.fill(0);
        }

        Ok(())
    }

    pub fn write_block(&mut self, block_addr: u32, buffer: &[u8; 512]) -> Result<(), MockSdError> {
        if !self.initialized {
            return Err(MockSdError::CardNotPresent);
        }

        if self.simulate_errors && block_addr % 100 == 99 {
            return Err(MockSdError::WriteError);
        }

        self.storage.insert(block_addr, *buffer);
        Ok(())
    }

    pub fn set_error_simulation(&mut self, enabled: bool) {
        self.simulate_errors = enabled;
    }
}

/// Mock SD Card information for testing
pub struct MockSdCardInfo {
    pub high_capacity: bool,
    pub rca: u32,
    pub ocr: u32,
    pub cid: [u32; 4],
    pub csd: [u32; 4],
    pub scr: [u32; 2],
}

impl MockSdCardInfo {
    pub fn new() -> Self {
        Self {
            high_capacity: true,
            rca: 0x1234,
            ocr: 0x80300000,  // Card ready, 3.3V support
            cid: [0x12345678, 0x9ABCDEF0, 0x11223344, 0x55667788],
            csd: [0xAABBCCDD, 0xEEFF0011, 0x22334455, 0x66778899],
            scr: [0x02350001, 0x00000000],  // SD spec version 2.0
        }
    }

    pub fn new_sdsc() -> Self {
        let mut info = Self::new();
        info.high_capacity = false;
        info.csd = [0x00260032, 0x5F5A83C6, 0x6DB7FF9F, 0x16800000];  // 1GB SDSC
        info
    }

    pub fn get_capacity(&self) -> u64 {
        if self.high_capacity {
            // SDHC/SDXC capacity calculation (simplified)
            let c_size = ((self.csd[1] & 0x3F) << 16) | ((self.csd[2] & 0xFFFF0000) >> 16);
            (c_size as u64 + 1) * 512 * 1024  // Simplified: 8GB card
        } else {
            // SDSC capacity calculation (simplified)
            1_073_741_824  // 1GB
        }
    }

    pub fn get_manufacturer_id(&self) -> u8 {
        ((self.cid[0] & 0xFF000000) >> 24) as u8
    }

    pub fn get_product_name(&self) -> [u8; 5] {
        [b'T', b'E', b'S', b'T', b'1']  // Mock product name
    }
}

/// Comprehensive mock system for integration testing
pub struct MockSystem {
    pub uart: MockUart,
    pub gpio: MockGpio,
    pub timer: MockTimer,
    pub memory: MockMemoryManager,
    pub interrupts: MockInterruptController,
    pub sdcard: MockSdCard,
}

impl MockSystem {
    pub fn new() -> Self {
        Self {
            uart: MockUart::new(),
            gpio: MockGpio::new(),
            timer: MockTimer::new(),
            memory: MockMemoryManager::new(0x100000, 4 * 1024 * 1024, 64),
            interrupts: MockInterruptController::new(),
            sdcard: MockSdCard::new(),
        }
    }

    pub fn reset_all(&mut self) {
        self.uart.clear_buffers();
        self.gpio.pin_states.clear();
        self.gpio.pin_modes.clear();
        self.timer.reset();
        self.memory.allocated_blocks.clear();
        self.memory.corruption_detected = false;
        self.memory.fragmentation_level = 0.0;
        self.interrupts.reset_statistics();
        self.sdcard = MockSdCard::new();
    }

    pub fn simulate_boot_sequence(&mut self) -> Result<(), &'static str> {
        // Simulate system initialization
        self.uart.enabled = true;
        self.gpio.enabled = true;
        self.timer.enabled = true;
        self.interrupts.controller_enabled = true;
        
        // Initialize SD card
        self.sdcard.initialized = true;
        self.sdcard.card_info = Some(MockSdCardInfo::new());
        
        // Set up LED pin
        self.gpio.set_pin_mode(42, GpioMode::Output)?;
        
        // Enable basic interrupts
        self.interrupts.enable_interrupt(64)?; // Timer
        self.interrupts.enable_interrupt(153)?; // UART
        self.interrupts.enable_interrupt(129)?; // GPIO
        
        Ok(())
    }

    pub fn run_system_health_check(&mut self) -> Result<SystemHealthReport, &'static str> {
        let mut report = SystemHealthReport::new();
        
        // Test UART
        report.uart_healthy = self.test_uart_functionality();
        
        // Test GPIO
        report.gpio_healthy = self.test_gpio_functionality();
        
        // Test Timer
        report.timer_healthy = self.test_timer_functionality();
        
        // Test Memory
        report.memory_healthy = self.test_memory_functionality();
        
        // Test Interrupts
        report.interrupt_healthy = self.test_interrupt_functionality();
        
        // Test SD Card
        report.sd_card_healthy = self.test_sd_card_functionality();
        
        Ok(report)
    }

    fn test_uart_functionality(&mut self) -> bool {
        match self.uart.write_string("test") {
            Ok(()) => self.uart.get_output_string() == "test",
            Err(_) => false,
        }
    }

    fn test_gpio_functionality(&mut self) -> bool {
        match self.gpio.set_pin(42, true) {
            Ok(()) => self.gpio.get_pin(42) == Some(true),
            Err(_) => false,
        }
    }

    fn test_timer_functionality(&mut self) -> bool {
        let initial_time = self.timer.get_time();
        self.timer.advance_time(1000);
        self.timer.get_time() == initial_time + 1000
    }

    fn test_memory_functionality(&mut self) -> bool {
        if let Some(addr) = self.memory.allocate(64) {
            self.memory.free(addr)
        } else {
            false
        }
    }

    fn test_interrupt_functionality(&mut self) -> bool {
        self.interrupts.trigger_interrupt(64);
        self.interrupts.get_interrupt_count(64) > 0
    }

    fn test_sd_card_functionality(&mut self) -> bool {
        let mut buffer = [0u8; 512];
        match self.sd_card.write_block(0, &buffer) {
            Ok(()) => match self.sd_card.read_block(0, &mut buffer) {
                Ok(()) => buffer.iter().all(|&x| x == 0),
                Err(_) => false,
            },
            Err(_) => false,
        }
    }
}

#[derive(Debug)]
pub struct SystemHealthReport {
    pub uart_healthy: bool,
    pub gpio_healthy: bool,
    pub timer_healthy: bool,
    pub memory_healthy: bool,
    pub interrupt_healthy: bool,
    pub sd_card_healthy: bool,
}

impl SystemHealthReport {
    pub fn new() -> Self {
        Self {
            uart_healthy: false,
            gpio_healthy: false,
            timer_healthy: false,
            memory_healthy: false,
            interrupt_healthy: false,
            sd_card_healthy: false,
        }
    }

    pub fn all_healthy(&self) -> bool {
        self.uart_healthy 
            && self.gpio_healthy 
            && self.timer_healthy 
            && self.memory_healthy 
            && self.interrupt_healthy
            && self.sd_card_healthy
    }

    pub fn health_score(&self) -> f32 {
        let total = 6.0;
        let healthy = [
            self.uart_healthy,
            self.gpio_healthy,
            self.timer_healthy,
            self.memory_healthy,
            self.interrupt_healthy,
            self.sd_card_healthy,
        ].iter().filter(|&&x| x).count() as f32;
        
        (healthy / total) * 100.0
    }
}

/// Mock SD Card Error types for testing
#[derive(Debug, Clone, Copy)]
pub enum MockSdError {
    InitializationFailed,
    CommandTimeout,
    CommandError,
    DataTimeout,
    DataError,
    InvalidArgument,
    CardNotPresent,
    ReadError,
    WriteError,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_uart() {
        let mut uart = MockUart::new();
        assert!(uart.write_string("Hello").is_ok());
        assert_eq!(uart.get_output_string(), "Hello");
        
        uart.add_input(b"World");
        assert_eq!(uart.read_byte(), Some(b'W'));
    }

    #[test]
    fn test_mock_gpio() {
        let mut gpio = MockGpio::new();
        assert!(gpio.set_pin_mode(42, GpioMode::Output).is_ok());
        assert!(gpio.set_pin(42, true).is_ok());
        assert_eq!(gpio.get_pin(42), Some(true));
        assert_eq!(gpio.toggle_pin(42).unwrap(), false);
    }

    #[test]
    fn test_mock_memory() {
        let mut memory = MockMemoryManager::new(0x100000, 1024, 64);
        let addr = memory.allocate(64).unwrap();
        assert!(memory.free(addr));
        assert!(!memory.free(addr)); // Double free should fail
    }

    #[test]
    fn test_mock_system() {
        let mut system = MockSystem::new();
        assert!(system.simulate_boot_sequence().is_ok());
        
        let health = system.run_system_health_check().unwrap();
        assert!(health.all_healthy());
        assert_eq!(health.health_score(), 100.0);
    }

    #[test]
    fn test_mock_sd_card() {
        let mut sd_card = MockSdCard::new_initialized();
        let mut buffer = [0u8; 512];
        
        // Test writing and reading a block
        assert!(sd_card.write_block(0, &buffer).is_ok());
        let mut read_buffer = [0u8; 512];
        assert!(sd_card.read_block(0, &mut read_buffer).is_ok());
        assert_eq!(buffer, read_buffer);
        
        // Test error simulation
        sd_card.set_error_simulation(true);
        assert!(sd_card.write_block(1, &buffer).is_err());
        assert!(sd_card.read_block(1, &mut read_buffer).is_err());
    }
}
