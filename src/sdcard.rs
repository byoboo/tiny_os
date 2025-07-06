use core::ptr::{read_volatile, write_volatile};

use crate::{gpio::*, timer::*};

/// SD card driver for Raspberry Pi EMMC interface
/// Provides block-level access to SD cards through the EMMC controller
///
/// This driver supports:
/// - SD card initialization and detection
/// - Block read/write operations (512-byte sectors)
/// - Error handling and timeout management
/// - QEMU-safe operation (graceful failure in emulation)
// Allow unused constants - these are defined for completeness and future use
#[allow(dead_code)]
const EMMC_BASE: u32 = 0xFE300000; // EMMC base address for Pi 4

// EMMC register offsets
#[allow(dead_code, clippy::identity_op)]
const EMMC_ARG2: u32 = EMMC_BASE + 0x00;
#[allow(dead_code)]
const EMMC_BLKSIZECNT: u32 = EMMC_BASE + 0x04;
#[allow(dead_code)]
const EMMC_ARG1: u32 = EMMC_BASE + 0x08;
#[allow(dead_code)]
const EMMC_CMDTM: u32 = EMMC_BASE + 0x0C;
#[allow(dead_code)]
const EMMC_RESP0: u32 = EMMC_BASE + 0x10;
#[allow(dead_code)]
const EMMC_RESP1: u32 = EMMC_BASE + 0x14;
#[allow(dead_code)]
const EMMC_RESP2: u32 = EMMC_BASE + 0x18;
#[allow(dead_code)]
const EMMC_RESP3: u32 = EMMC_BASE + 0x1C;
#[allow(dead_code)]
const EMMC_DATA: u32 = EMMC_BASE + 0x20;
#[allow(dead_code)]
const EMMC_STATUS: u32 = EMMC_BASE + 0x24;
#[allow(dead_code)]
const EMMC_CONTROL0: u32 = EMMC_BASE + 0x28;
#[allow(dead_code)]
const EMMC_CONTROL1: u32 = EMMC_BASE + 0x2C;
#[allow(dead_code)]
const EMMC_INTERRUPT: u32 = EMMC_BASE + 0x30;
#[allow(dead_code)]
const EMMC_IRPT_MASK: u32 = EMMC_BASE + 0x34;
#[allow(dead_code)]
const EMMC_IRPT_EN: u32 = EMMC_BASE + 0x38;
#[allow(dead_code)]
const EMMC_CONTROL2: u32 = EMMC_BASE + 0x3C;
#[allow(dead_code)]
const EMMC_SLOTISR_VER: u32 = EMMC_BASE + 0xFC;

// Command flags
#[allow(dead_code)]
const CMD_NEED_APP: u32 = 0x80000000;
#[allow(dead_code)]
const CMD_RSPNS_48: u32 = 0x00020000;
#[allow(dead_code)]
const CMD_RSPNS_136: u32 = 0x00010000;
#[allow(dead_code)]
const CMD_RSPNS_BUSY: u32 = 0x00040000;
#[allow(dead_code)]
const CMD_IS_DATA: u32 = 0x00200000;
#[allow(dead_code)]
const CMD_ISDATAW: u32 = 0x00400000;

// Command types
#[allow(dead_code)]
const CMD_GO_IDLE: u32 = 0x00000000;
#[allow(dead_code)]
const CMD_SEND_OP_COND: u32 = 0x01000000 | CMD_RSPNS_48;
#[allow(dead_code)]
const CMD_ALL_SEND_CID: u32 = 0x02000000 | CMD_RSPNS_136;
#[allow(dead_code)]
const CMD_SEND_REL_ADDR: u32 = 0x03000000 | CMD_RSPNS_48;
#[allow(dead_code)]
const CMD_SWITCH_FUNC: u32 = 0x06000000 | CMD_RSPNS_48;
#[allow(dead_code)]
const CMD_CARD_SELECT: u32 = 0x07000000 | CMD_RSPNS_48;
#[allow(dead_code)]
const CMD_SEND_IF_COND: u32 = 0x08000000 | CMD_RSPNS_48;
#[allow(dead_code)]
const CMD_SEND_CSD: u32 = 0x09000000 | CMD_RSPNS_136;
#[allow(dead_code)]
const CMD_SEND_CID: u32 = 0x0A000000 | CMD_RSPNS_136;
#[allow(dead_code)]
const CMD_STOP_TRANS: u32 = 0x0C000000 | CMD_RSPNS_48 | CMD_RSPNS_BUSY;
#[allow(dead_code)]
const CMD_SEND_STATUS: u32 = 0x0D000000 | CMD_RSPNS_48;
#[allow(dead_code)]
const CMD_SET_BLOCLEN: u32 = 0x10000000 | CMD_RSPNS_48;
#[allow(dead_code)]
const CMD_READ_SINGLE: u32 = 0x11000000 | CMD_RSPNS_48 | CMD_IS_DATA;
#[allow(dead_code)]
const CMD_READ_MULTI: u32 = 0x12000000 | CMD_RSPNS_48 | CMD_IS_DATA;
#[allow(dead_code)]
const CMD_WRITE_SINGLE: u32 = 0x18000000 | CMD_RSPNS_48 | CMD_IS_DATA | CMD_ISDATAW;
#[allow(dead_code)]
const CMD_WRITE_MULTI: u32 = 0x19000000 | CMD_RSPNS_48 | CMD_IS_DATA | CMD_ISDATAW;
#[allow(dead_code)]
const CMD_APP_CMD: u32 = 0x37000000 | CMD_RSPNS_48;

// APP commands (preceded by CMD_APP_CMD)
#[allow(dead_code)]
const ACMD_SEND_OP_COND: u32 = 0x29000000 | CMD_RSPNS_48;
#[allow(dead_code)]
const ACMD_SEND_SCR: u32 = 0x33000000 | CMD_RSPNS_48 | CMD_IS_DATA;

// Status register bits
#[allow(dead_code)]
const STATUS_CMD_INHIBIT: u32 = 0x00000001;
#[allow(dead_code)]
const STATUS_DAT_INHIBIT: u32 = 0x00000002;
#[allow(dead_code)]
const STATUS_DAT_ACTIVE: u32 = 0x00000004;
#[allow(dead_code)]
const STATUS_WRITE_AVAILABLE: u32 = 0x00000010;
#[allow(dead_code)]
const STATUS_READ_AVAILABLE: u32 = 0x00000800;

// Interrupt flags
#[allow(dead_code)]
const INT_CMD_DONE: u32 = 0x00000001;
#[allow(dead_code)]
const INT_DATA_DONE: u32 = 0x00000002;
#[allow(dead_code)]
const INT_WRITE_RDY: u32 = 0x00000010;
#[allow(dead_code)]
const INT_READ_RDY: u32 = 0x00000020;
#[allow(dead_code)]
const INT_ERR_MASK: u32 = 0x017E8000;

// Control register bits
#[allow(dead_code)]
const CONTROL0_HCTL_8BIT: u32 = 0x00000020;
#[allow(dead_code)]
const CONTROL1_CLK_INTLEN: u32 = 0x00000001;
#[allow(dead_code)]
const CONTROL1_CLK_STABLE: u32 = 0x00000002;
#[allow(dead_code)]
const CONTROL1_CLK_EN: u32 = 0x00000004;
#[allow(dead_code)]
const CONTROL1_RESET_HOST: u32 = 0x01000000;
#[allow(dead_code)]
const CONTROL1_RESET_CMD: u32 = 0x02000000;
#[allow(dead_code)]
const CONTROL1_RESET_DATA: u32 = 0x04000000;

// Standard constants
#[allow(dead_code)]
const SD_BLOCK_SIZE: u32 = 512;

#[derive(Debug, Clone, Copy)]
pub enum SdError {
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

pub struct SdCard {
    gpio: Gpio,
    timer: SystemTimer,
    card_rca: u32,       // Relative Card Address
    card_ocr: u32,       // Operating Conditions Register
    card_cid: [u32; 4],  // Card Identification
    card_csd: [u32; 4],  // Card Specific Data
    card_scr: [u32; 2],  // SD Configuration Register
    high_capacity: bool, // SDHC/SDXC card
    initialized: bool,
}

impl SdCard {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            gpio: Gpio::new(),
            timer: SystemTimer::new(),
            card_rca: 0,
            card_ocr: 0,
            card_cid: [0; 4],
            card_csd: [0; 4],
            card_scr: [0; 2],
            high_capacity: false,
            initialized: false,
        }
    }

    /// Small delay helper
    fn delay_short(&self) {
        self.timer.delay_us(1);
    }

    /// Medium delay helper
    fn delay_medium(&self) {
        self.timer.delay_us(10);
    }

    /// Initialize the SD card interface
    pub fn init(&mut self) -> Result<(), SdError> {
        // Configure GPIO pins for SD card (ALT3 function)
        // GPIO 48-53 are used for SD card interface
        for pin in 48..=53 {
            self.gpio.set_function(pin, GpioFunction::Alt3);
        }

        // Reset the EMMC controller
        self.reset_controller()?;

        // Set up clocks and power
        self.setup_clocks()?;

        // Initialize the card
        self.initialize_card()?;

        self.initialized = true;
        Ok(())
    }

    /// Reset the EMMC controller
    fn reset_controller(&mut self) -> Result<(), SdError> {
        unsafe {
            // Reset all controller components
            let mut control1 = read_volatile(EMMC_CONTROL1 as *const u32);
            control1 |= CONTROL1_RESET_HOST | CONTROL1_RESET_CMD | CONTROL1_RESET_DATA;
            write_volatile(EMMC_CONTROL1 as *mut u32, control1);

            // Wait for reset to complete
            let mut timeout = 10000;
            while timeout > 0 {
                let control1 = read_volatile(EMMC_CONTROL1 as *const u32);
                if (control1 & (CONTROL1_RESET_HOST | CONTROL1_RESET_CMD | CONTROL1_RESET_DATA))
                    == 0
                {
                    break;
                }
                self.delay_short();
                timeout -= 1;
            }

            if timeout == 0 {
                return Err(SdError::InitializationFailed);
            }
        }

        Ok(())
    }

    /// Setup clocks for SD card operation
    fn setup_clocks(&mut self) -> Result<(), SdError> {
        unsafe {
            // Set initial clock to 400kHz for initialization
            let mut control1 = read_volatile(EMMC_CONTROL1 as *const u32);

            // Clear clock enable
            control1 &= !CONTROL1_CLK_EN;
            write_volatile(EMMC_CONTROL1 as *mut u32, control1);

            // Set clock divider for 400kHz (assuming 50MHz source clock)
            // Divider = 50MHz / (2 * 400kHz) = ~62 (use 64 = 0x40)
            control1 &= !0x0000FFF0; // Clear divider bits
            control1 |= 0x00000400; // Set divider to 64
            write_volatile(EMMC_CONTROL1 as *mut u32, control1);

            // Enable internal clock
            control1 |= CONTROL1_CLK_INTLEN;
            write_volatile(EMMC_CONTROL1 as *mut u32, control1);

            // Wait for clock to stabilize
            let mut timeout = 10000;
            while timeout > 0 {
                let control1 = read_volatile(EMMC_CONTROL1 as *const u32);
                if (control1 & CONTROL1_CLK_STABLE) != 0 {
                    break;
                }
                self.delay_short();
                timeout -= 1;
            }

            if timeout == 0 {
                return Err(SdError::InitializationFailed);
            }

            // Enable SD clock
            control1 |= CONTROL1_CLK_EN;
            write_volatile(EMMC_CONTROL1 as *mut u32, control1);

            // Set interrupt masks
            write_volatile(EMMC_IRPT_EN as *mut u32, 0xFFFFFFFF);
            write_volatile(EMMC_IRPT_MASK as *mut u32, 0xFFFFFFFF);
        }

        self.timer.delay_ms(10); // Allow clocks to settle
        Ok(())
    }

    /// Initialize the SD card through the initialization sequence
    fn initialize_card(&mut self) -> Result<(), SdError> {
        // Step 1: Send CMD0 (GO_IDLE_STATE)
        self.send_command(CMD_GO_IDLE, 0)?;
        self.timer.delay_ms(10);

        // Step 2: Send CMD8 (SEND_IF_COND) to check if card supports 2.7-3.6V
        let cmd8_response = self.send_command(CMD_SEND_IF_COND, 0x000001AA)?;
        let supports_v2 = (cmd8_response & 0x000001AA) == 0x000001AA;

        // Step 3: Send ACMD41 (SD_SEND_OP_COND) until card is ready
        let mut ocr = 0x00300000; // 3.2-3.4V range
        if supports_v2 {
            ocr |= 0x40000000; // HCS bit for SDHC support
        }

        let mut attempts = 0;
        loop {
            // Send CMD55 (APP_CMD) first
            self.send_command(CMD_APP_CMD, 0)?;

            // Send ACMD41
            let response = self.send_command(ACMD_SEND_OP_COND, ocr)?;

            if (response & 0x80000000) != 0 {
                // Card is ready
                self.card_ocr = response;
                self.high_capacity = (response & 0x40000000) != 0;
                break;
            }

            attempts += 1;
            if attempts > 100 {
                return Err(SdError::InitializationFailed);
            }

            self.timer.delay_ms(10);
        }

        // Step 4: Send CMD2 (ALL_SEND_CID)
        {
            let mut temp_cid = [0u32; 4];
            self.send_command_long(CMD_ALL_SEND_CID, 0, &mut temp_cid)?;
            self.card_cid = temp_cid;
        }

        // Step 5: Send CMD3 (SEND_RELATIVE_ADDR)
        let rca_response = self.send_command(CMD_SEND_REL_ADDR, 0)?;
        self.card_rca = rca_response & 0xFFFF0000;

        // Step 6: Send CMD9 (SEND_CSD)
        {
            let card_rca = self.card_rca;
            let mut temp_csd = [0u32; 4];
            self.send_command_long(CMD_SEND_CSD, card_rca, &mut temp_csd)?;
            self.card_csd = temp_csd;
        }

        // Step 7: Send CMD7 (SELECT_CARD)
        let card_rca = self.card_rca;
        self.send_command(CMD_CARD_SELECT, card_rca)?;

        // Step 8: Send ACMD51 (SEND_SCR)
        self.send_command(CMD_APP_CMD, card_rca)?;
        {
            let mut temp_scr_bytes = [0u8; 8];
            self.send_data_command(ACMD_SEND_SCR, 0, temp_scr_bytes.as_mut_ptr(), 8)?;
            // Convert bytes to u32 array
            for i in 0..2 {
                self.card_scr[i] = u32::from_le_bytes([
                    temp_scr_bytes[i * 4],
                    temp_scr_bytes[i * 4 + 1],
                    temp_scr_bytes[i * 4 + 2],
                    temp_scr_bytes[i * 4 + 3],
                ]);
            }
        }

        // Step 9: Set block length to 512 bytes
        self.send_command(CMD_SET_BLOCLEN, SD_BLOCK_SIZE)?;

        // Step 10: Increase clock speed for data operations (25MHz)
        self.set_clock_speed(25000000)?;

        Ok(())
    }

    /// Set clock speed for data operations
    fn set_clock_speed(&mut self, target_hz: u32) -> Result<(), SdError> {
        unsafe {
            let mut control1 = read_volatile(EMMC_CONTROL1 as *const u32);

            // Disable clock
            control1 &= !CONTROL1_CLK_EN;
            write_volatile(EMMC_CONTROL1 as *mut u32, control1);

            // Calculate divider (assuming 50MHz source)
            let source_hz = 50000000;
            #[allow(clippy::manual_clamp)]
            let divider = (source_hz / (2 * target_hz)).max(1).min(1023);

            // Set new divider
            control1 &= !0x0000FFF0;
            control1 |= (divider & 0x3FF) << 8;
            control1 |= ((divider >> 10) & 0x3) << 6;
            write_volatile(EMMC_CONTROL1 as *mut u32, control1);

            // Wait for clock to stabilize
            let mut timeout = 10000;
            while timeout > 0 {
                let control1 = read_volatile(EMMC_CONTROL1 as *const u32);
                if (control1 & CONTROL1_CLK_STABLE) != 0 {
                    break;
                }
                self.delay_short();
                timeout -= 1;
            }

            if timeout == 0 {
                return Err(SdError::InitializationFailed);
            }

            // Re-enable clock
            control1 |= CONTROL1_CLK_EN;
            write_volatile(EMMC_CONTROL1 as *mut u32, control1);
        }

        Ok(())
    }

    /// Send a command to the SD card
    fn send_command(&mut self, command: u32, argument: u32) -> Result<u32, SdError> {
        // Wait for command line to be free
        self.wait_for_command_ready()?;

        // Clear interrupt flags
        unsafe {
            write_volatile(EMMC_INTERRUPT as *mut u32, 0xFFFFFFFF);
        }

        // Send command
        unsafe {
            write_volatile(EMMC_ARG1 as *mut u32, argument);
            write_volatile(EMMC_CMDTM as *mut u32, command);
        }

        // Wait for command completion
        let mut timeout = 100000;
        loop {
            let interrupt = unsafe { read_volatile(EMMC_INTERRUPT as *const u32) };

            if (interrupt & INT_ERR_MASK) != 0 {
                return Err(SdError::CommandError);
            }

            if (interrupt & INT_CMD_DONE) != 0 {
                break;
            }

            timeout -= 1;
            if timeout == 0 {
                return Err(SdError::CommandTimeout);
            }

            self.delay_medium();
        }

        // Return response
        let response = unsafe { read_volatile(EMMC_RESP0 as *const u32) };
        Ok(response)
    }

    /// Send a command with 136-bit response
    fn send_command_long(
        &mut self,
        command: u32,
        argument: u32,
        response: &mut [u32; 4],
    ) -> Result<(), SdError> {
        // Wait for command line to be free
        self.wait_for_command_ready()?;

        // Clear interrupt flags
        unsafe {
            write_volatile(EMMC_INTERRUPT as *mut u32, 0xFFFFFFFF);
        }

        // Send command
        unsafe {
            write_volatile(EMMC_ARG1 as *mut u32, argument);
            write_volatile(EMMC_CMDTM as *mut u32, command);
        }

        // Wait for command completion
        let mut timeout = 100000;
        loop {
            let interrupt = unsafe { read_volatile(EMMC_INTERRUPT as *const u32) };

            if (interrupt & INT_ERR_MASK) != 0 {
                return Err(SdError::CommandError);
            }

            if (interrupt & INT_CMD_DONE) != 0 {
                break;
            }

            timeout -= 1;
            if timeout == 0 {
                return Err(SdError::CommandTimeout);
            }

            self.delay_medium();
        }

        // Read 136-bit response
        unsafe {
            response[0] = read_volatile(EMMC_RESP0 as *const u32);
            response[1] = read_volatile(EMMC_RESP1 as *const u32);
            response[2] = read_volatile(EMMC_RESP2 as *const u32);
            response[3] = read_volatile(EMMC_RESP3 as *const u32);
        }

        Ok(())
    }

    /// Send a command with data transfer
    fn send_data_command(
        &mut self,
        command: u32,
        argument: u32,
        buffer: *mut u8,
        length: usize,
    ) -> Result<(), SdError> {
        if length == 0 || buffer.is_null() {
            return Err(SdError::InvalidArgument);
        }

        // Wait for command and data lines to be free
        self.wait_for_data_ready()?;

        // Set up block size and count
        unsafe {
            #[allow(clippy::manual_div_ceil)]
            let blocks = (length + SD_BLOCK_SIZE as usize - 1) / SD_BLOCK_SIZE as usize;
            write_volatile(
                EMMC_BLKSIZECNT as *mut u32,
                (blocks as u32) << 16 | SD_BLOCK_SIZE,
            );
        }

        // Clear interrupt flags
        unsafe {
            write_volatile(EMMC_INTERRUPT as *mut u32, 0xFFFFFFFF);
        }

        // Send command
        unsafe {
            write_volatile(EMMC_ARG1 as *mut u32, argument);
            write_volatile(EMMC_CMDTM as *mut u32, command);
        }

        // Wait for command completion
        let mut timeout = 100000;
        loop {
            let interrupt = unsafe { read_volatile(EMMC_INTERRUPT as *const u32) };

            if (interrupt & INT_ERR_MASK) != 0 {
                return Err(SdError::CommandError);
            }

            if (interrupt & INT_CMD_DONE) != 0 {
                break;
            }

            timeout -= 1;
            if timeout == 0 {
                return Err(SdError::CommandTimeout);
            }

            self.delay_medium();
        }

        // Transfer data
        if (command & CMD_ISDATAW) != 0 {
            // Write data
            self.write_data(buffer, length)?;
        } else {
            // Read data
            self.read_data(buffer, length)?;
        }

        Ok(())
    }

    /// Read data from the SD card
    fn read_data(&mut self, buffer: *mut u8, length: usize) -> Result<(), SdError> {
        let mut remaining = length;
        let mut offset = 0;

        while remaining > 0 {
            // Wait for data to be available
            let mut timeout = 100000;
            loop {
                let status = unsafe { read_volatile(EMMC_STATUS as *const u32) };
                let interrupt = unsafe { read_volatile(EMMC_INTERRUPT as *const u32) };

                if (interrupt & INT_ERR_MASK) != 0 {
                    return Err(SdError::DataError);
                }

                if (status & STATUS_READ_AVAILABLE) != 0 {
                    break;
                }

                timeout -= 1;
                if timeout == 0 {
                    return Err(SdError::DataTimeout);
                }

                self.delay_medium();
            }

            // Read available data (up to 4 bytes at a time)
            let chunk_size = remaining.min(4);
            let data = unsafe { read_volatile(EMMC_DATA as *const u32) };

            for i in 0..chunk_size {
                unsafe {
                    *buffer.add(offset + i) = ((data >> (i * 8)) & 0xFF) as u8;
                }
            }

            remaining -= chunk_size;
            offset += chunk_size;
        }

        // Wait for data completion
        let mut timeout = 100000;
        loop {
            let interrupt = unsafe { read_volatile(EMMC_INTERRUPT as *const u32) };

            if (interrupt & INT_ERR_MASK) != 0 {
                return Err(SdError::DataError);
            }

            if (interrupt & INT_DATA_DONE) != 0 {
                break;
            }

            timeout -= 1;
            if timeout == 0 {
                return Err(SdError::DataTimeout);
            }

            self.delay_medium();
        }

        Ok(())
    }

    /// Write data to the SD card
    fn write_data(&mut self, buffer: *const u8, length: usize) -> Result<(), SdError> {
        let mut remaining = length;
        let mut offset = 0;

        while remaining > 0 {
            // Wait for write buffer to be available
            let mut timeout = 100000;
            loop {
                let status = unsafe { read_volatile(EMMC_STATUS as *const u32) };
                let interrupt = unsafe { read_volatile(EMMC_INTERRUPT as *const u32) };

                if (interrupt & INT_ERR_MASK) != 0 {
                    return Err(SdError::DataError);
                }

                if (status & STATUS_WRITE_AVAILABLE) != 0 {
                    break;
                }

                timeout -= 1;
                if timeout == 0 {
                    return Err(SdError::DataTimeout);
                }

                self.delay_medium();
            }

            // Write data (up to 4 bytes at a time)
            let chunk_size = remaining.min(4);
            let mut data = 0u32;

            for i in 0..chunk_size {
                let byte = unsafe { *buffer.add(offset + i) };
                data |= (byte as u32) << (i * 8);
            }

            unsafe {
                write_volatile(EMMC_DATA as *mut u32, data);
            }

            remaining -= chunk_size;
            offset += chunk_size;
        }

        // Wait for data completion
        let mut timeout = 100000;
        loop {
            let interrupt = unsafe { read_volatile(EMMC_INTERRUPT as *const u32) };

            if (interrupt & INT_ERR_MASK) != 0 {
                return Err(SdError::DataError);
            }

            if (interrupt & INT_DATA_DONE) != 0 {
                break;
            }

            timeout -= 1;
            if timeout == 0 {
                return Err(SdError::DataTimeout);
            }

            self.delay_medium();
        }

        Ok(())
    }

    /// Wait for command line to be ready
    fn wait_for_command_ready(&mut self) -> Result<(), SdError> {
        let mut timeout = 100000;
        while timeout > 0 {
            let status = unsafe { read_volatile(EMMC_STATUS as *const u32) };
            if (status & STATUS_CMD_INHIBIT) == 0 {
                return Ok(());
            }
            timeout -= 1;
            self.delay_medium();
        }
        Err(SdError::CommandTimeout)
    }

    /// Wait for data line to be ready
    fn wait_for_data_ready(&mut self) -> Result<(), SdError> {
        let mut timeout = 100000;
        while timeout > 0 {
            let status = unsafe { read_volatile(EMMC_STATUS as *const u32) };
            if (status & (STATUS_CMD_INHIBIT | STATUS_DAT_INHIBIT)) == 0 {
                return Ok(());
            }
            timeout -= 1;
            self.delay_medium();
        }
        Err(SdError::DataTimeout)
    }

    /// Read a single block from the SD card
    pub fn read_block(&mut self, block_addr: u32, buffer: &mut [u8; 512]) -> Result<(), SdError> {
        if !self.initialized {
            return Err(SdError::CardNotPresent);
        }

        let addr = if self.high_capacity {
            block_addr // SDHC uses block addressing
        } else {
            block_addr * SD_BLOCK_SIZE // SDSC uses byte addressing
        };

        self.send_data_command(CMD_READ_SINGLE, addr, buffer.as_mut_ptr(), 512)
            .map_err(|_| SdError::ReadError)
    }

    /// Write a single block to the SD card
    pub fn write_block(&mut self, block_addr: u32, buffer: &[u8; 512]) -> Result<(), SdError> {
        if !self.initialized {
            return Err(SdError::CardNotPresent);
        }

        let addr = if self.high_capacity {
            block_addr // SDHC uses block addressing
        } else {
            block_addr * SD_BLOCK_SIZE // SDSC uses byte addressing
        };

        self.send_data_command(CMD_WRITE_SINGLE, addr, buffer.as_ptr() as *mut u8, 512)
            .map_err(|_| SdError::WriteError)
    }

    /// Get card information
    pub fn get_card_info(&self) -> Option<SdCardInfo> {
        if !self.initialized {
            return None;
        }

        Some(SdCardInfo {
            high_capacity: self.high_capacity,
            rca: self.card_rca >> 16,
            ocr: self.card_ocr,
            cid: self.card_cid,
            csd: self.card_csd,
            scr: self.card_scr,
        })
    }

    /// Check if SD card is initialized
    pub fn is_initialized(&self) -> bool {
        self.initialized
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SdCardInfo {
    pub high_capacity: bool,
    pub rca: u32,
    #[allow(dead_code)]
    pub ocr: u32,
    pub cid: [u32; 4],
    pub csd: [u32; 4],
    #[allow(dead_code)]
    pub scr: [u32; 2],
}

impl SdCardInfo {
    /// Get card capacity in bytes (approximate)
    pub fn get_capacity(&self) -> u64 {
        if self.high_capacity {
            // SDHC/SDXC capacity calculation
            let c_size = ((self.csd[1] & 0x3F) << 16) | ((self.csd[2] & 0xFFFF0000) >> 16);
            (c_size as u64 + 1) * 512 * 1024 // In bytes
        } else {
            // SDSC capacity calculation
            let c_size = ((self.csd[1] & 0x3FF) << 2) | ((self.csd[2] & 0xC0000000) >> 30);
            let c_size_mult = (self.csd[2] & 0x00038000) >> 15;
            let read_bl_len = (self.csd[1] & 0x000F0000) >> 16;

            ((c_size + 1) as u64) * (1u64 << (c_size_mult + 2)) * (1u64 << read_bl_len)
        }
    }

    /// Get manufacturer ID from CID
    pub fn get_manufacturer_id(&self) -> u8 {
        ((self.cid[0] & 0xFF000000) >> 24) as u8
    }

    /// Get product name from CID (5 ASCII characters)
    pub fn get_product_name(&self) -> [u8; 5] {
        let mut name = [0u8; 5];
        name[0] = ((self.cid[0] & 0x00FF0000) >> 16) as u8;
        name[1] = ((self.cid[0] & 0x0000FF00) >> 8) as u8;
        name[2] = (self.cid[0] & 0x000000FF) as u8;
        name[3] = ((self.cid[1] & 0xFF000000) >> 24) as u8;
        name[4] = ((self.cid[1] & 0x00FF0000) >> 16) as u8;
        name
    }
}
