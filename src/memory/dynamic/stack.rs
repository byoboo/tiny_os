//! Dynamic Stack Management
//!
//! This module handles dynamic stack growth and shrinking with configurable
//! policies. Supports conservative, aggressive, and predictive growth
//! strategies based on usage patterns.

use crate::memory::PAGE_SIZE;

/// Maximum number of dynamic stacks
pub const MAX_DYNAMIC_STACKS: usize = 32;

/// Memory pressure levels
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PressureLevel {
    Low,
    Medium,
    High,
    Critical,
}

/// Dynamic stack growth policy
#[derive(Debug, Clone, Copy)]
pub enum StackGrowthPolicy {
    Conservative, // Grow by single page
    Aggressive,   // Grow by multiple pages
    Predictive,   // Grow based on usage patterns
}

/// Dynamic stack information
#[derive(Debug, Clone, Copy)]
pub struct DynamicStack {
    pub id: u32,
    pub base_address: u64,
    pub current_size: usize,
    pub max_size: usize,
    pub growth_count: u32,
    pub shrink_count: u32,
    pub pressure_level: PressureLevel,
    pub last_access_time: u64,
}

impl DynamicStack {
    pub fn new(id: u32, base_address: u64, initial_size: usize, max_size: usize) -> Self {
        Self {
            id,
            base_address,
            current_size: initial_size,
            max_size,
            growth_count: 0,
            shrink_count: 0,
            pressure_level: PressureLevel::Low,
            last_access_time: 0,
        }
    }

    pub fn can_grow(&self, requested_size: usize) -> bool {
        self.current_size + requested_size <= self.max_size
    }

    pub fn grow(&mut self, size: usize) -> Result<(), &'static str> {
        if !self.can_grow(size) {
            return Err("Stack growth would exceed maximum size");
        }

        self.current_size += size;
        self.growth_count += 1;
        Ok(())
    }

    pub fn shrink(&mut self, size: usize) -> Result<(), &'static str> {
        if size > self.current_size {
            return Err("Cannot shrink stack below zero size");
        }

        self.current_size -= size;
        self.shrink_count += 1;
        Ok(())
    }
}

/// Dynamic stack manager
pub struct DynamicStackManager {
    stacks: [Option<DynamicStack>; MAX_DYNAMIC_STACKS],
    growth_policy: StackGrowthPolicy,
    #[allow(dead_code)]
    pressure_threshold: usize,
    next_stack_id: u32,
}

impl DynamicStackManager {
    pub fn new() -> Self {
        Self {
            stacks: [None; MAX_DYNAMIC_STACKS],
            growth_policy: StackGrowthPolicy::Conservative,
            pressure_threshold: 1024 * 1024, // 1MB
            next_stack_id: 1,
        }
    }

    pub fn create_dynamic_stack(
        &mut self,
        base_address: u64,
        initial_size: usize,
        max_size: usize,
    ) -> Result<u32, &'static str> {
        // Find available slot
        for i in 0..MAX_DYNAMIC_STACKS {
            if self.stacks[i].is_none() {
                let stack_id = self.next_stack_id;
                self.next_stack_id += 1;

                let stack = DynamicStack::new(stack_id, base_address, initial_size, max_size);
                self.stacks[i] = Some(stack);
                return Ok(stack_id);
            }
        }
        Err("No available dynamic stack slots")
    }

    pub fn handle_stack_growth(
        &mut self,
        stack_id: u32,
        _fault_address: u64,
    ) -> Result<usize, &'static str> {
        // Find the stack
        let stack = self
            .stacks
            .iter_mut()
            .find(|s| s.as_ref().map_or(false, |stack| stack.id == stack_id))
            .ok_or("Stack not found")?
            .as_mut()
            .unwrap();

        // Calculate growth size based on policy
        let growth_size = match self.growth_policy {
            StackGrowthPolicy::Conservative => PAGE_SIZE,
            StackGrowthPolicy::Aggressive => PAGE_SIZE * 4,
            StackGrowthPolicy::Predictive => {
                // Simple predictive algorithm based on growth history
                if stack.growth_count > 5 {
                    PAGE_SIZE * 2 // More aggressive growth for frequently
                                  // growing stacks
                } else {
                    PAGE_SIZE
                }
            }
        };

        // Check if growth is possible
        if !stack.can_grow(growth_size as usize) {
            return Err("Stack growth would exceed maximum size");
        }

        // Grow the stack
        stack.grow(growth_size as usize)?;

        Ok(growth_size as usize)
    }

    #[allow(dead_code)]
    fn calculate_predictive_growth(&self, stack: &DynamicStack) -> u32 {
        // Simple predictive algorithm based on growth history
        if stack.growth_count > 5 {
            PAGE_SIZE * 2 // More aggressive growth for frequently growing
                          // stacks
        } else {
            PAGE_SIZE
        }
    }

    pub fn shrink_unused_stacks(&mut self) -> u32 {
        let mut shrunk_count = 0;

        for stack in self.stacks.iter_mut().flatten() {
            // Simple shrinking logic - can be made more sophisticated
            if stack.current_size > PAGE_SIZE as usize && stack.growth_count > stack.shrink_count {
                if stack.shrink(PAGE_SIZE as usize).is_ok() {
                    shrunk_count += 1;
                }
            }
        }

        shrunk_count
    }

    pub fn get_stack_count(&self) -> u32 {
        self.stacks.iter().filter(|s| s.is_some()).count() as u32
    }
}
