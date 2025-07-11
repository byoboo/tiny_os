//! User Space Page Table Management Commands
//!
//! This module contains commands for managing user space page tables,
//! including per-process page tables, VMAs, and memory isolation.

use crate::{
    memory::{
        get_user_space_manager, init_user_space_manager, user_space::create_standard_user_layout,
        RegionType, VmaType,
    },
    process::scheduler::get_current_task_id,
    shell::ShellContext,
};

/// Handle user space status command
pub fn handle_user_space_status(context: &ShellContext) {
    context
        .uart
        .puts("\r\n=== User Space Page Table Status ===\r\n");

    if let Some(manager) = get_user_space_manager() {
        let stats = manager.get_statistics();

        context.uart.puts("Global Statistics:\r\n");
        context.uart.puts("  Page Tables Created: ");
        context.uart.put_hex(stats.page_tables_created as u64);
        context.uart.puts("\r\n  Page Tables Destroyed: ");
        context.uart.put_hex(stats.page_tables_destroyed as u64);
        context.uart.puts("\r\n  Context Switches: ");
        context.uart.put_hex(stats.context_switches as u64);
        context.uart.puts("\r\n  VMAs Created: ");
        context.uart.put_hex(stats.vmas_created as u64);
        context.uart.puts("\r\n  VMAs Destroyed: ");
        context.uart.put_hex(stats.vmas_destroyed as u64);
        context.uart.puts("\r\n  Pages Mapped: ");
        context.uart.put_hex(stats.pages_mapped as u64);
        context.uart.puts("\r\n  Virtual Memory Allocated: ");
        context.uart.put_hex(stats.vm_allocated_bytes);
        context.uart.puts(" bytes\r\n");
        context.uart.puts("  TLB Flushes: ");
        context.uart.put_hex(stats.tlb_flushes as u64);
        context.uart.puts("\r\n");

        // Show active page tables
        context.uart.puts("\r\nActive Page Tables:\r\n");
        for i in 0..32 {
            // MAX_USER_PROCESSES
            if let Some(page_table) = manager.get_page_table(i) {
                let pt_stats = page_table.get_stats();
                context.uart.puts("  Slot ");
                context.uart.put_hex(i as u64);
                context.uart.puts(": Process ");
                context.uart.put_hex(pt_stats.process_id as u64);
                context.uart.puts(", ASID ");
                context.uart.put_hex(pt_stats.asid as u64);
                context.uart.puts(", VMAs ");
                context.uart.put_hex(pt_stats.vma_count as u64);
                context.uart.puts(", Active: ");
                context
                    .uart
                    .puts(if pt_stats.is_active { "Yes" } else { "No" });
                context.uart.puts("\r\n");
            }
        }

        // Show current active page table
        if let Some(current_slot) = manager.get_current_active() {
            context.uart.puts("\r\nCurrently Active: Slot ");
            context.uart.put_hex(current_slot as u64);
            context.uart.puts("\r\n");
        } else {
            context.uart.puts("\r\nNo active user page table\r\n");
        }
    } else {
        context.uart.puts("User space manager not initialized\r\n");
    }

    context.uart.puts("==================================\r\n");
}

/// Handle create user page table command
pub fn handle_create_user_page_table(context: &ShellContext) {
    context.uart.puts("\r\n=== Create User Page Table ===\r\n");

    if let Some(manager) = get_user_space_manager() {
        // Use current task ID if available, otherwise use a test process ID
        let process_id = get_current_task_id().unwrap_or(1000) as usize;

        context.uart.puts("Creating page table for process ");
        context.uart.put_hex(process_id as u64);
        context.uart.puts("...\r\n");

        match manager.create_page_table(process_id) {
            Ok(slot) => {
                context.uart.puts("✓ Page table created successfully\r\n");
                context.uart.puts("  Slot: ");
                context.uart.put_hex(slot as u64);
                context.uart.puts("\r\n");

                // Create standard user layout
                match create_standard_user_layout(process_id) {
                    Ok(layout_slot) => {
                        context.uart.puts("✓ Standard user layout created\r\n");
                        context.uart.puts("  Layout slot: ");
                        context.uart.put_hex(layout_slot as u64);
                        context.uart.puts("\r\n");
                    }
                    Err(e) => {
                        context.uart.puts("✗ Failed to create standard layout: ");
                        context.uart.puts(e);
                        context.uart.puts("\r\n");
                    }
                }
            }
            Err(e) => {
                context.uart.puts("✗ Failed to create page table: ");
                context.uart.puts(e);
                context.uart.puts("\r\n");
            }
        }
    } else {
        context.uart.puts("User space manager not initialized\r\n");
    }

    context.uart.puts("=============================\r\n");
}

/// Handle destroy user page table command
pub fn handle_destroy_user_page_table(context: &ShellContext) {
    context.uart.puts("\r\n=== Destroy User Page Table ===\r\n");
    context.uart.puts("Enter slot number (0-31): ");

    // For now, use a test slot - in a real implementation would read from UART
    let test_slot = 0usize;
    context.uart.put_hex(test_slot as u64);
    context.uart.puts("\r\n");

    if let Some(manager) = get_user_space_manager() {
        match manager.destroy_page_table(test_slot) {
            Ok(()) => {
                context.uart.puts("✓ Page table destroyed successfully\r\n");
            }
            Err(e) => {
                context.uart.puts("✗ Failed to destroy page table: ");
                context.uart.puts(e);
                context.uart.puts("\r\n");
            }
        }
    } else {
        context.uart.puts("User space manager not initialized\r\n");
    }

    context.uart.puts("==============================\r\n");
}

/// Handle switch user page table command
pub fn handle_switch_user_page_table(context: &ShellContext) {
    context.uart.puts("\r\n=== Switch User Page Table ===\r\n");
    context.uart.puts("Enter slot number (0-31): ");

    // For now, use a test slot - in a real implementation would read from UART
    let test_slot = 0usize;
    context.uart.put_hex(test_slot as u64);
    context.uart.puts("\r\n");

    if let Some(manager) = get_user_space_manager() {
        match manager.switch_page_table(test_slot) {
            Ok(()) => {
                context.uart.puts("✓ Page table switched successfully\r\n");
                context.uart.puts("  New active slot: ");
                context.uart.put_hex(test_slot as u64);
                context.uart.puts("\r\n");
            }
            Err(e) => {
                context.uart.puts("✗ Failed to switch page table: ");
                context.uart.puts(e);
                context.uart.puts("\r\n");
            }
        }
    } else {
        context.uart.puts("User space manager not initialized\r\n");
    }

    context.uart.puts("=============================\r\n");
}

/// Handle VMA management command
pub fn handle_vma_management(context: &ShellContext) {
    context.uart.puts("\r\n=== VMA Management ===\r\n");

    if let Some(manager) = get_user_space_manager() {
        // Show VMAs for current active page table
        if let Some(current_slot) = manager.get_current_active() {
            context.uart.puts("VMAs for active page table (slot ");
            context.uart.put_hex(current_slot as u64);
            context.uart.puts("):\r\n");

            if let Some(page_table) = manager.get_page_table(current_slot) {
                let vma_count = page_table.vmas.len();
                context.uart.puts("  Total VMAs: ");
                context.uart.put_hex(vma_count as u64);
                context.uart.puts("\r\n");

                for i in 0..vma_count {
                    if let Some(vma) = page_table.vmas.get_vma(i) {
                        context.uart.puts("    VMA ");
                        context.uart.put_hex(i as u64);
                        context.uart.puts(": 0x");
                        context.uart.put_hex(vma.start_addr);
                        context.uart.puts(" - 0x");
                        context.uart.put_hex(vma.end_addr);
                        context.uart.puts(" (");
                        match vma.vma_type {
                            VmaType::Code => context.uart.puts("Code"),
                            VmaType::Data => context.uart.puts("Data"),
                            VmaType::Heap => context.uart.puts("Heap"),
                            VmaType::Stack => context.uart.puts("Stack"),
                            VmaType::Shared => context.uart.puts("Shared"),
                            VmaType::MmapFile => context.uart.puts("MmapFile"),
                            VmaType::MmapAnon => context.uart.puts("MmapAnon"),
                        }
                        context.uart.puts(", ");
                        context
                            .uart
                            .puts(if vma.is_mapped { "Mapped" } else { "Unmapped" });
                        context.uart.puts(")\r\n");
                    }
                }
            }
        } else {
            context.uart.puts("No active page table\r\n");
        }
    } else {
        context.uart.puts("User space manager not initialized\r\n");
    }

    context.uart.puts("====================\r\n");
}

/// Handle user space test command
pub fn handle_user_space_test(context: &mut ShellContext) {
    context.uart.puts("\r\n=== User Space Test ===\r\n");

    // Initialize user space manager if not already done
    if get_user_space_manager().is_none() {
        context.uart.puts("Initializing user space manager...\r\n");
        init_user_space_manager(&mut context.memory_manager as *mut _ as *mut _);
    }

    context
        .uart
        .puts("Running user space page table test...\r\n");

    // Test 1: Create page table
    context.uart.puts("1. Creating test page table...\r\n");
    if let Some(manager) = get_user_space_manager() {
        match manager.create_page_table(1001) {
            Ok(slot) => {
                context.uart.puts("   ✓ Page table created at slot ");
                context.uart.put_hex(slot as u64);
                context.uart.puts("\r\n");

                // Test 2: Add VMA
                context.uart.puts("2. Adding VMA...\r\n");
                if let Some(page_table) = manager.get_page_table_mut(slot) {
                    match page_table.add_vma(0x400000, 0x1000, VmaType::Code, RegionType::UserCode)
                    {
                        Ok(vma_index) => {
                            context.uart.puts("   ✓ VMA added at index ");
                            context.uart.put_hex(vma_index as u64);
                            context.uart.puts("\r\n");
                        }
                        Err(e) => {
                            context.uart.puts("   ✗ Failed to add VMA: ");
                            context.uart.puts(e);
                            context.uart.puts("\r\n");
                        }
                    }
                }

                // Test 3: Switch to page table
                context.uart.puts("3. Switching to page table...\r\n");
                match manager.switch_page_table(slot) {
                    Ok(()) => {
                        context
                            .uart
                            .puts("   ✓ Page table switched successfully\r\n");
                    }
                    Err(e) => {
                        context.uart.puts("   ✗ Failed to switch page table: ");
                        context.uart.puts(e);
                        context.uart.puts("\r\n");
                    }
                }

                // Test 4: Get statistics
                context.uart.puts("4. Getting statistics...\r\n");
                let stats = manager.get_statistics();
                context.uart.puts("   Page tables created: ");
                context.uart.put_hex(stats.page_tables_created as u64);
                context.uart.puts("\r\n   Context switches: ");
                context.uart.put_hex(stats.context_switches as u64);
                context.uart.puts("\r\n");

                // Test 5: Cleanup
                context.uart.puts("5. Cleaning up...\r\n");
                match manager.destroy_page_table(slot) {
                    Ok(()) => {
                        context
                            .uart
                            .puts("   ✓ Page table destroyed successfully\r\n");
                    }
                    Err(e) => {
                        context.uart.puts("   ✗ Failed to destroy page table: ");
                        context.uart.puts(e);
                        context.uart.puts("\r\n");
                    }
                }
            }
            Err(e) => {
                context.uart.puts("   ✗ Failed to create page table: ");
                context.uart.puts(e);
                context.uart.puts("\r\n");
            }
        }
    }

    context.uart.puts("User space test completed\r\n");
    context.uart.puts("======================\r\n");
}

/// Handle user space initialization command
pub fn handle_user_space_init(context: &mut ShellContext) {
    context
        .uart
        .puts("\r\n=== Initialize User Space Manager ===\r\n");

    context.uart.puts("Initializing user space manager...\r\n");
    init_user_space_manager(&mut context.memory_manager as *mut _ as *mut _);

    context.uart.puts("✓ User space manager initialized\r\n");
    context.uart.puts("===================================\r\n");
}
