//! High-level GPIO Driver API
//!
//! This module provides a safe, high-level interface to the GPIO peripheral
//! with pin management, configuration, and type-safe pin operations.

use super::hardware::{GpioFunction, GpioHardware};
use crate::drivers::{
    config::{DefaultHardware, HardwareVersion},
    traits::{DriverError, DriverStatus, Initialize, Status},
};

/// GPIO driver configuration
#[derive(Debug, Clone, Copy)]
pub struct GpioConfig {
    /// Default function for all pins
    pub default_function: GpioFunction,
}

impl Default for GpioConfig {
    fn default() -> Self {
        Self {
            default_function: GpioFunction::Input,
        }
    }
}

/// High-level GPIO driver
pub struct GpioDriver<H: HardwareVersion = DefaultHardware> {
    hardware: GpioHardware<H>,
    status: DriverStatus,
}

impl<H: HardwareVersion> GpioDriver<H> {
    /// Create a new GPIO driver instance
    pub const fn new() -> Self {
        Self {
            hardware: GpioHardware::new(),
            status: DriverStatus::Uninitialized,
        }
    }

    /// Set GPIO pin function (input, output, alt functions)
    #[inline]
    pub fn set_function(&self, pin: u32, function: GpioFunction) {
        self.hardware.set_function(pin, function);
    }

    /// Set a GPIO pin high
    #[inline]
    pub fn set_high(&self, pin: u32) {
        self.hardware.set_high(pin);
    }

    /// Set a GPIO pin low
    #[inline]
    pub fn set_low(&self, pin: u32) {
        self.hardware.set_low(pin);
    }

    /// Set a GPIO pin to a specific level
    #[inline]
    pub fn set_pin(&self, pin: u32, high: bool) {
        if high {
            self.set_high(pin);
        } else {
            self.set_low(pin);
        }
    }

    /// Read the current level of a GPIO pin
    #[inline]
    pub fn read_pin(&self, pin: u32) -> bool {
        self.hardware.read_pin(pin)
    }

    /// Toggle a GPIO pin
    #[inline]
    pub fn toggle_pin(&self, pin: u32) {
        let current = self.read_pin(pin);
        self.set_pin(pin, !current);
    }

    /// Configure a pin as an input
    #[inline]
    pub fn set_input(&self, pin: u32) {
        self.set_function(pin, GpioFunction::Input);
    }

    /// Configure a pin as an output
    #[inline]
    pub fn set_output(&self, pin: u32) {
        self.set_function(pin, GpioFunction::Output);
    }

    /// Configure a pin as an output and set its initial level
    #[inline]
    pub fn set_output_with_level(&self, pin: u32, high: bool) {
        self.set_output(pin);
        self.set_pin(pin, high);
    }
}

impl<H: HardwareVersion> Initialize for GpioDriver<H> {
    type Config = GpioConfig;

    fn init(&mut self) -> Result<(), DriverError> {
        let config = GpioConfig::default();
        self.init_with_config(&config)
    }

    fn init_with_config(&mut self, _config: &Self::Config) -> Result<(), DriverError> {
        // GPIO doesn't require special initialization
        // Individual pins are configured as needed
        self.status = DriverStatus::Ready;
        Ok(())
    }
}

impl<H: HardwareVersion> Status for GpioDriver<H> {
    fn status(&self) -> DriverStatus {
        self.status
    }
}

/// Type-safe GPIO pin representation
pub struct GpioPin<const PIN: u32, H: HardwareVersion = DefaultHardware> {
    driver: *const GpioDriver<H>,
}

impl<const PIN: u32, H: HardwareVersion> GpioPin<PIN, H> {
    /// Create a new GPIO pin reference
    ///
    /// # Safety
    /// The driver reference must be valid for the lifetime of this pin
    pub unsafe fn new(driver: &GpioDriver<H>) -> Self {
        Self {
            driver: driver as *const _,
        }
    }

    /// Get the pin number
    #[inline]
    pub const fn pin_number(&self) -> u32 {
        PIN
    }

    /// Set this pin's function
    #[inline]
    pub fn set_function(&self, function: GpioFunction) {
        unsafe {
            (*self.driver).set_function(PIN, function);
        }
    }

    /// Set this pin high
    #[inline]
    pub fn set_high(&self) {
        unsafe {
            (*self.driver).set_high(PIN);
        }
    }

    /// Set this pin low
    #[inline]
    pub fn set_low(&self) {
        unsafe {
            (*self.driver).set_low(PIN);
        }
    }

    /// Set this pin to a specific level
    #[inline]
    pub fn set(&self, high: bool) {
        unsafe {
            (*self.driver).set_pin(PIN, high);
        }
    }

    /// Read the current level of this pin
    #[inline]
    pub fn read(&self) -> bool {
        unsafe { (*self.driver).read_pin(PIN) }
    }

    /// Toggle this pin
    #[inline]
    pub fn toggle(&self) {
        unsafe {
            (*self.driver).toggle_pin(PIN);
        }
    }

    /// Configure this pin as an input
    #[inline]
    pub fn set_input(&self) {
        self.set_function(GpioFunction::Input);
    }

    /// Configure this pin as an output
    #[inline]
    pub fn set_output(&self) {
        self.set_function(GpioFunction::Output);
    }

    /// Configure this pin as an output and set its initial level
    #[inline]
    pub fn set_output_with_level(&self, high: bool) {
        self.set_output();
        self.set(high);
    }
}

/// Commonly used GPIO pins on Raspberry Pi
impl<H: HardwareVersion> GpioDriver<H> {
    /// Get a type-safe reference to the activity LED pin (GPIO 42 on Pi 4)
    pub fn activity_led(&self) -> GpioPin<42, H> {
        unsafe { GpioPin::new(self) }
    }

    /// Get a type-safe reference to GPIO pin 18 (commonly used for PWM)
    pub fn pin_18(&self) -> GpioPin<18, H> {
        unsafe { GpioPin::new(self) }
    }

    /// Get a type-safe reference to any GPIO pin
    pub fn pin<const PIN: u32>(&self) -> GpioPin<PIN, H> {
        unsafe { GpioPin::new(self) }
    }
}

/// Type alias for the default GPIO driver
pub type Gpio = GpioDriver<DefaultHardware>;
