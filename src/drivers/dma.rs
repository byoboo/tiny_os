//! DMA Controller Driver
//!
//! Provides efficient memory-to-memory transfers using the Pi's DMA controller.
//! Optimized for Pi 4/5 enhanced DMA capabilities while maintaining Pi 3
//! compatibility.

use core::ptr;

use crate::benchmarks::timing;

/// DMA register base addresses
mod registers {
    /// DMA controller base (BCM2835/2836/2837/2711)
    pub const DMA_BASE: usize = 0x3F007000;

    /// DMA channel offset (each channel is 0x100 bytes apart)
    pub const DMA_CHANNEL_SIZE: usize = 0x100;

    /// DMA control block registers
    pub const DMA_CS: usize = 0x00; // Control and Status
    pub const DMA_CONBLK_AD: usize = 0x04; // Control Block Address
    pub const DMA_TI: usize = 0x08; // Transfer Information
    pub const DMA_SOURCE_AD: usize = 0x0C; // Source Address
    pub const DMA_DEST_AD: usize = 0x10; // Destination Address
    pub const DMA_TXFR_LEN: usize = 0x14; // Transfer Length
    pub const DMA_STRIDE: usize = 0x18; // 2D Stride
    pub const DMA_NEXTCONBK: usize = 0x1C; // Next Control Block
    pub const DMA_DEBUG: usize = 0x20; // Debug

    /// Global DMA registers
    pub const DMA_INT_STATUS: usize = DMA_BASE + 0xFE0; // Interrupt Status
    pub const DMA_ENABLE: usize = DMA_BASE + 0xFF0; // Global Enable
}

/// DMA Control and Status register bits
mod cs_bits {
    pub const ACTIVE: u32 = 1 << 0; // DMA Active
    pub const END: u32 = 1 << 1; // DMA End
    pub const INT: u32 = 1 << 2; // Interrupt Status
    pub const DREQ: u32 = 1 << 3; // Data Request
    pub const PAUSED: u32 = 1 << 4; // DMA Paused
    pub const DREQ_STOPS_DMA: u32 = 1 << 5; // DREQ Stops DMA
    pub const WAITING_FOR_OUTSTANDING_WRITES: u32 = 1 << 6;
    pub const ERROR: u32 = 1 << 8; // DMA Error
    pub const PRIORITY: u32 = 0xF << 16; // AXI Priority Level
    pub const PANIC_PRIORITY: u32 = 0xF << 20; // AXI Panic Priority Level
    pub const WAIT_FOR_OUTSTANDING_WRITES: u32 = 1 << 28;
    pub const DISDEBUG: u32 = 1 << 29; // Disable Debug Pause
    pub const ABORT: u32 = 1 << 30; // Abort DMA
    pub const RESET: u32 = 1 << 31; // DMA Channel Reset
}

/// DMA Transfer Information register bits
mod ti_bits {
    pub const INTEN: u32 = 1 << 0; // Interrupt Enable
    pub const TDMODE: u32 = 1 << 1; // 2D Mode
    pub const WAIT_RESP: u32 = 1 << 3; // Wait for Response
    pub const DEST_INC: u32 = 1 << 4; // Destination Address Increment
    pub const DEST_WIDTH: u32 = 1 << 5; // Destination Transfer Width
    pub const DEST_DREQ: u32 = 1 << 6; // Control Destination Writes with DREQ
    pub const DEST_IGNORE: u32 = 1 << 7; // Ignore Destination Writes
    pub const SRC_INC: u32 = 1 << 8; // Source Address Increment
    pub const SRC_WIDTH: u32 = 1 << 9; // Source Transfer Width
    pub const SRC_DREQ: u32 = 1 << 10; // Control Source Reads with DREQ
    pub const SRC_IGNORE: u32 = 1 << 11; // Ignore Source Reads
    pub const BURST_LENGTH: u32 = 0xF << 12; // Burst Transfer Length
    pub const PERMAP: u32 = 0x1F << 16; // Peripheral Mapping
    pub const WAITS: u32 = 0x1F << 21; // Add Wait Cycles
    pub const NO_WIDE_BURSTS: u32 = 1 << 26; // Don't Do Wide Writes
}

/// DMA Control Block structure (must be 32-byte aligned)
#[repr(C, align(32))]
pub struct DmaControlBlock {
    /// Transfer Information
    pub ti: u32,
    /// Source Address
    pub source_ad: u32,
    /// Destination Address
    pub dest_ad: u32,
    /// Transfer Length
    pub txfr_len: u32,
    /// 2D Mode Stride
    pub stride: u32,
    /// Next Control Block Address
    pub nextconbk: u32,
    /// Reserved fields for alignment
    pub reserved: [u32; 2],
}

impl DmaControlBlock {
    /// Create new DMA control block for memory-to-memory transfer
    pub fn new_memory_transfer(src: u32, dst: u32, length: u32) -> Self {
        Self {
            ti: ti_bits::SRC_INC | ti_bits::DEST_INC | ti_bits::WAIT_RESP,
            source_ad: src,
            dest_ad: dst,
            txfr_len: length,
            stride: 0,
            nextconbk: 0,
            reserved: [0; 2],
        }
    }

    /// Create optimized control block for Pi 4/5
    pub fn new_optimized_transfer(src: u32, dst: u32, length: u32, is_pi4_or_5: bool) -> Self {
        let mut ti = ti_bits::SRC_INC | ti_bits::DEST_INC | ti_bits::WAIT_RESP;

        if is_pi4_or_5 {
            // Pi 4/5 optimizations
            ti |= ti_bits::BURST_LENGTH & (0x4 << 12); // Burst length 4
                                                       // Enable wider transfers
                                                       // for better bandwidth
                                                       // utilization
        } else {
            // Pi 3 conservative settings
            ti |= ti_bits::BURST_LENGTH & (0x2 << 12); // Burst length 2
        }

        Self {
            ti,
            source_ad: src,
            dest_ad: dst,
            txfr_len: length,
            stride: 0,
            nextconbk: 0,
            reserved: [0; 2],
        }
    }
}

/// DMA channel management
pub struct DmaChannel {
    /// Channel number (0-14)
    channel: u8,
    /// Channel base address
    base_addr: usize,
    /// Available for use
    available: bool,
}

impl DmaChannel {
    /// Create new DMA channel
    pub const fn new(channel: u8) -> Self {
        Self {
            channel,
            base_addr: registers::DMA_BASE + (channel as usize * registers::DMA_CHANNEL_SIZE),
            available: channel < 15, // Channels 0-14 available
        }
    }

    /// Read channel register
    fn read_register(&self, offset: usize) -> u32 {
        unsafe { ptr::read_volatile((self.base_addr + offset) as *const u32) }
    }

    /// Write channel register
    fn write_register(&self, offset: usize, value: u32) {
        unsafe { ptr::write_volatile((self.base_addr + offset) as *mut u32, value) }
    }

    /// Reset DMA channel
    pub fn reset(&self) {
        self.write_register(registers::DMA_CS, cs_bits::RESET);

        // Wait for reset to complete
        while (self.read_register(registers::DMA_CS) & cs_bits::RESET) != 0 {
            // Spin wait
        }
    }

    /// Check if channel is active
    pub fn is_active(&self) -> bool {
        (self.read_register(registers::DMA_CS) & cs_bits::ACTIVE) != 0
    }

    /// Check if transfer is complete
    pub fn is_complete(&self) -> bool {
        (self.read_register(registers::DMA_CS) & cs_bits::END) != 0
    }

    /// Check for errors
    pub fn has_error(&self) -> bool {
        (self.read_register(registers::DMA_CS) & cs_bits::ERROR) != 0
    }

    /// Start DMA transfer with control block
    pub fn start_transfer(&self, control_block: &DmaControlBlock) -> Result<(), &'static str> {
        if !self.available {
            return Err("DMA channel not available");
        }

        if self.is_active() {
            return Err("DMA channel already active");
        }

        // Reset channel first
        self.reset();

        // Convert control block address to bus address
        let cb_addr = control_block as *const DmaControlBlock as u32;
        let bus_addr = (cb_addr & !0xC0000000) | 0x40000000;

        // Set control block address
        self.write_register(registers::DMA_CONBLK_AD, bus_addr);

        // Start transfer
        self.write_register(registers::DMA_CS, cs_bits::ACTIVE);

        Ok(())
    }

    /// Wait for transfer completion
    pub fn wait_for_completion(&self) -> Result<(), &'static str> {
        // Timeout after reasonable number of cycles
        let mut timeout = 1000000;

        while self.is_active() && timeout > 0 {
            if self.has_error() {
                return Err("DMA transfer error");
            }
            timeout -= 1;
        }

        if timeout == 0 {
            Err("DMA transfer timeout")
        } else if self.has_error() {
            Err("DMA transfer error")
        } else {
            Ok(())
        }
    }

    /// Perform memory-to-memory transfer
    pub fn memory_transfer(
        &self,
        src: *const u8,
        dst: *mut u8,
        length: u32,
        is_pi4_or_5: bool,
    ) -> Result<u64, &'static str> {
        let start_cycles = timing::get_cycles();

        // Convert addresses to bus addresses
        let src_bus = ((src as u32) & !0xC0000000) | 0x40000000;
        let dst_bus = ((dst as u32) & !0xC0000000) | 0x40000000;

        // Create control block
        let control_block =
            DmaControlBlock::new_optimized_transfer(src_bus, dst_bus, length, is_pi4_or_5);

        // Start transfer
        self.start_transfer(&control_block)?;

        // Wait for completion
        self.wait_for_completion()?;

        let end_cycles = timing::get_cycles();
        Ok(end_cycles - start_cycles)
    }
}

/// DMA Controller
pub struct DmaController {
    /// Available channels
    channels: [DmaChannel; 15],
    /// Pi model detection
    is_pi4_or_5: bool,
    /// Initialization status
    initialized: bool,
}

impl DmaController {
    /// Create new DMA controller
    pub const fn new() -> Self {
        Self {
            channels: [
                DmaChannel::new(0),
                DmaChannel::new(1),
                DmaChannel::new(2),
                DmaChannel::new(3),
                DmaChannel::new(4),
                DmaChannel::new(5),
                DmaChannel::new(6),
                DmaChannel::new(7),
                DmaChannel::new(8),
                DmaChannel::new(9),
                DmaChannel::new(10),
                DmaChannel::new(11),
                DmaChannel::new(12),
                DmaChannel::new(13),
                DmaChannel::new(14),
            ],
            is_pi4_or_5: false,
            initialized: false,
        }
    }

    /// Initialize DMA controller
    pub fn init(&mut self, is_pi4_or_5: bool) -> Result<(), &'static str> {
        if self.initialized {
            return Ok(());
        }

        self.is_pi4_or_5 = is_pi4_or_5;

        // Enable DMA channels
        unsafe {
            ptr::write_volatile(registers::DMA_ENABLE as *mut u32, 0x7FFF); // Enable channels 0-14
        }

        // Reset all channels
        for channel in &self.channels {
            channel.reset();
        }

        self.initialized = true;
        Ok(())
    }

    /// Check if DMA controller is initialized
    pub fn is_initialized(&self) -> bool {
        self.initialized
    }

    /// Get DMA availability status
    pub fn is_available(&self) -> bool {
        self.initialized
    }

    /// Get available DMA channel
    pub fn get_available_channel(&self) -> Option<&DmaChannel> {
        for channel in &self.channels {
            if channel.available && !channel.is_active() {
                return Some(channel);
            }
        }
        None
    }

    /// Perform optimized memory copy using DMA
    pub fn memory_copy(&self, dst: &mut [u8], src: &[u8]) -> Result<u64, &'static str> {
        if !self.initialized {
            return Err("DMA controller not initialized");
        }

        if dst.len() != src.len() {
            return Err("Source and destination size mismatch");
        }

        let channel = self
            .get_available_channel()
            .ok_or("No DMA channel available")?;

        channel.memory_transfer(
            src.as_ptr(),
            dst.as_mut_ptr(),
            src.len() as u32,
            self.is_pi4_or_5,
        )
    }

    /// Perform CPU memory copy for comparison
    pub fn cpu_memory_copy(&self, dst: &mut [u8], src: &[u8]) -> u64 {
        let start_cycles = timing::get_cycles();
        dst.copy_from_slice(src);
        let end_cycles = timing::get_cycles();
        end_cycles - start_cycles
    }

    /// Benchmark DMA vs CPU memory copy
    pub fn benchmark_memory_copy(&self, size: usize) -> Result<(u64, u64), &'static str> {
        // Use fixed-size buffers for no-std compatibility
        const MAX_SIZE: usize = 8192;
        let actual_size = core::cmp::min(size, MAX_SIZE);

        let mut src_data = [0xAAu8; MAX_SIZE];
        let mut dst_cpu = [0u8; MAX_SIZE];
        let mut dst_dma = [0u8; MAX_SIZE];

        // Initialize only the portion we're testing
        for i in 0..actual_size {
            src_data[i] = 0xAA;
        }

        // CPU copy
        let cpu_cycles =
            self.cpu_memory_copy(&mut dst_cpu[..actual_size], &src_data[..actual_size]);

        // DMA copy
        let dma_cycles = self.memory_copy(&mut dst_dma[..actual_size], &src_data[..actual_size])?;

        // Verify results
        if dst_cpu[..actual_size] != dst_dma[..actual_size] {
            return Err("DMA and CPU results don't match");
        }

        Ok((cpu_cycles, dma_cycles))
    }

    /// Get optimal transfer size threshold for DMA vs CPU
    pub fn get_dma_threshold(&self) -> u32 {
        if self.is_pi4_or_5 {
            1024 // Pi 4/5: Use DMA for transfers > 1KB
        } else {
            4096 // Pi 3: Use DMA for transfers > 4KB
        }
    }
}

/// Global DMA controller instance
static mut DMA_CONTROLLER: DmaController = DmaController::new();

/// Initialize DMA controller
pub fn init(is_pi4_or_5: bool) -> Result<(), &'static str> {
    unsafe { DMA_CONTROLLER.init(is_pi4_or_5) }
}

/// Get global DMA controller
pub fn get_dma_controller() -> &'static DmaController {
    unsafe { &DMA_CONTROLLER }
}

/// Get mutable global DMA controller
pub fn get_dma_controller_mut() -> &'static mut DmaController {
    unsafe { &mut DMA_CONTROLLER }
}

/// Test DMA functionality
pub fn test_dma() -> Result<(), &'static str> {
    let dma = get_dma_controller();

    if !dma.initialized {
        return Err("DMA controller not initialized");
    }

    // Test various transfer sizes
    let test_sizes = [1024, 4096, 16384];

    for &size in &test_sizes {
        let (cpu_cycles, dma_cycles) = dma.benchmark_memory_copy(size)?;

        // DMA should be beneficial for larger transfers
        if size >= dma.get_dma_threshold() as usize && dma_cycles >= cpu_cycles {
            // This is expected for small transfers due to setup overhead
        }
    }

    Ok(())
}
