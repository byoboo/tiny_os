//! TinyOS Application Framework
//!
//! This module provides a framework for developing applications that run on TinyOS,
//! optimized for Raspberry Pi 4/5 hardware performance.

pub mod editor;

/// Application trait for all TinyOS applications
pub trait Application {
    /// Initialize the application
    fn init(&mut self) -> Result<(), &'static str>;
    
    /// Run the application main loop
    fn run(&mut self) -> Result<(), &'static str>;
    
    /// Clean up when the application exits
    fn cleanup(&mut self);
    
    /// Get the application name
    fn name(&self) -> &'static str;
}

/// Application runtime for managing applications
pub struct ApplicationRuntime {
    current_app: Option<Box<dyn Application>>,
}

impl ApplicationRuntime {
    /// Create a new application runtime
    pub fn new() -> Self {
        Self {
            current_app: None,
        }
    }
    
    /// Launch an application
    pub fn launch(&mut self, mut app: Box<dyn Application>) -> Result<(), &'static str> {
        // Initialize the application
        app.init()?;
        
        // Run the application
        let result = app.run();
        
        // Clean up
        app.cleanup();
        
        result
    }
    
    /// Check if an application is currently running
    pub fn is_running(&self) -> bool {
        self.current_app.is_some()
    }
}

/// Global application runtime instance
static mut APP_RUNTIME: Option<ApplicationRuntime> = None;

/// Initialize the application runtime
pub fn init_app_runtime() {
    unsafe {
        APP_RUNTIME = Some(ApplicationRuntime::new());
    }
}

/// Get the global application runtime
pub fn get_app_runtime() -> Option<&'static mut ApplicationRuntime> {
    unsafe { APP_RUNTIME.as_mut() }
}