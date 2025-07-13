//! Dynamic Memory Manager
//!
//! This module provides the main dynamic memory manager that coordinates all dynamic memory
//! subsystems including stack management, lazy allocation, pressure handling, and context switching.

use crate::memory::{
    mmu_exceptions::{MmuExceptionType, MmuFaultInfo},
    MemoryManager,
};

use super::context::HardwareContextSwitcher;
use super::lazy::LazyPageAllocator;
use super::pressure::{MemoryPressureHandler, OptimizationStrategy};
use super::stack::{DynamicStackManager, PressureLevel};

/// Dynamic memory statistics
#[derive(Debug, Clone)]
pub struct DynamicMemoryStats {
    pub total_dynamic_stacks: u32,
    pub active_dynamic_stacks: u32,
    pub total_stack_growth_events: u32,
    pub total_stack_shrink_events: u32,
    pub total_lazy_pages: u32,
    pub allocated_lazy_pages: u32,
    pub total_lazy_page_faults: u32,
    pub memory_pressure_events: u32,
    pub optimization_events: u32,
    pub context_switch_count: u32,
}

impl DynamicMemoryStats {
    pub fn new() -> Self {
        Self {
            total_dynamic_stacks: 0,
            active_dynamic_stacks: 0,
            total_stack_growth_events: 0,
            total_stack_shrink_events: 0,
            total_lazy_pages: 0,
            allocated_lazy_pages: 0,
            total_lazy_page_faults: 0,
            memory_pressure_events: 0,
            optimization_events: 0,
            context_switch_count: 0,
        }
    }
}

/// Main dynamic memory manager
pub struct DynamicMemoryManager {
    stack_manager: DynamicStackManager,
    lazy_allocator: LazyPageAllocator,
    pressure_handler: MemoryPressureHandler,
    context_switcher: HardwareContextSwitcher,
    statistics: DynamicMemoryStats,
    enabled: bool,
}

impl DynamicMemoryManager {
    pub fn new() -> Self {
        Self {
            stack_manager: DynamicStackManager::new(),
            lazy_allocator: LazyPageAllocator::new(),
            pressure_handler: MemoryPressureHandler::new(),
            context_switcher: HardwareContextSwitcher::new(),
            statistics: DynamicMemoryStats::new(),
            enabled: false,
        }
    }

    pub fn init(&mut self) -> Result<(), &'static str> {
        // Initialize dynamic memory management
        self.enabled = true;
        Ok(())
    }

    pub fn handle_dynamic_fault(
        &mut self,
        fault_info: &MmuFaultInfo,
        memory_manager: &mut MemoryManager,
    ) -> Result<(), &'static str> {
        if !self.enabled {
            return Err("Dynamic memory management not enabled");
        }

        match fault_info.exception_type {
            MmuExceptionType::TranslationFault { level: _ } => {
                // Check if this is a lazy page fault
                if self
                    .lazy_allocator
                    .handle_lazy_page_fault(fault_info.fault_address, memory_manager)
                    .is_ok()
                {
                    self.statistics.total_lazy_page_faults += 1;
                    return Ok(());
                }

                // Check if this is a stack growth fault
                // This is a simplified check - in practice, you'd need more sophisticated logic
                if self
                    .stack_manager
                    .handle_stack_growth(1, fault_info.fault_address)
                    .is_ok()
                {
                    self.statistics.total_stack_growth_events += 1;
                    return Ok(());
                }

                Err("Unhandled page fault")
            }
            _ => Err("Unsupported fault type for dynamic memory management"),
        }
    }

    pub fn check_memory_pressure(&mut self, available_memory: usize) -> PressureLevel {
        let pressure = self
            .pressure_handler
            .check_memory_pressure(available_memory);

        if pressure != PressureLevel::Low {
            self.statistics.memory_pressure_events += 1;

            // Handle memory pressure
            let strategies = self.pressure_handler.handle_memory_pressure(pressure);
            for strategy in strategies {
                self.apply_optimization_strategy(strategy);
            }
        }

        pressure
    }

    fn apply_optimization_strategy(&mut self, strategy: OptimizationStrategy) {
        match strategy {
            OptimizationStrategy::None => {
                // No optimization needed
            }
            OptimizationStrategy::Defragmentation => {
                // Implement memory defragmentation
                self.statistics.optimization_events += 1;
            }
            OptimizationStrategy::PageMigration => {
                // Implement page migration
                self.statistics.optimization_events += 1;
            }
            OptimizationStrategy::CacheOptimization => {
                // Implement cache optimization
                self.statistics.optimization_events += 1;
            }
            OptimizationStrategy::PressureRelief => {
                // Implement pressure relief (e.g., stack shrinking)
                let shrunk_stacks = self.stack_manager.shrink_unused_stacks();
                self.statistics.total_stack_shrink_events += shrunk_stacks;
                self.statistics.optimization_events += 1;
            }
        }
    }

    pub fn create_dynamic_stack(
        &mut self,
        base_address: u64,
        initial_size: usize,
        max_size: usize,
    ) -> Result<u32, &'static str> {
        let stack_id =
            self.stack_manager
                .create_dynamic_stack(base_address, initial_size, max_size)?;
        self.statistics.total_dynamic_stacks += 1;
        self.statistics.active_dynamic_stacks += 1;
        Ok(stack_id)
    }

    pub fn add_lazy_page(&mut self, virtual_address: u64) -> Result<usize, &'static str> {
        let page_index = self.lazy_allocator.add_lazy_page(virtual_address)?;
        self.statistics.total_lazy_pages += 1;
        Ok(page_index)
    }

    pub fn fast_context_switch(
        &mut self,
        from_asid: u16,
        to_asid: u16,
    ) -> Result<(), &'static str> {
        self.context_switcher
            .fast_context_switch(from_asid, to_asid)?;
        self.statistics.context_switch_count += 1;
        Ok(())
    }

    pub fn get_statistics(&self) -> &DynamicMemoryStats {
        &self.statistics
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
}
