//! Copy-on-Write (COW) memory management command handlers
//!
//! This module contains handlers for COW page management, status monitoring,
//! mapping creation, protection control, and testing functionality.

use super::utils::{print_hex, print_number};
use crate::shell::ShellContext;

/// COW Status command
pub fn cmd_cow_status(_args: &[&str], context: &mut ShellContext) {
    context.uart.puts("\r\n=== COW Status ===\r\n");

    if let Some(stats) =
        crate::memory::with_cow_manager(|cow_manager| cow_manager.get_statistics().clone())
    {
        context.uart.puts("COW Pages: ");
        print_number(&context.uart, stats.cow_pages_count as u32);
        context.uart.puts("\r\n");

        context.uart.puts("COW Faults: ");
        print_number(&context.uart, stats.cow_faults_handled as u32);
        context.uart.puts("\r\n");

        context.uart.puts("Pages Duplicated: ");
        print_number(&context.uart, stats.pages_duplicated as u32);
        context.uart.puts("\r\n");

        context.uart.puts("Memory Saved: ");
        print_number(&context.uart, stats.memory_saved_bytes as u32);
        context.uart.puts(" bytes\r\n");

        context.uart.puts("Peak COW Pages: ");
        print_number(&context.uart, stats.peak_cow_pages as u32);
        context.uart.puts("\r\n");

        // List all COW pages - need to do this in a separate call
        context.uart.puts("\r\nCOW Pages:\r\n");
        let _pages_printed = crate::memory::with_cow_manager(|cow_manager| {
            let cow_pages = cow_manager.get_all_cow_pages();
            for (phys_addr, page_opt) in cow_pages.iter() {
                if let Some(page) = page_opt {
                    context.uart.puts("  0x");
                    print_hex(&context.uart, *phys_addr);
                    context.uart.puts(" refs=");
                    print_number(&context.uart, page.ref_count as u32);
                    context.uart.puts(" cow=");
                    context.uart.puts(if page.is_cow { "yes" } else { "no" });
                    context.uart.puts("\r\n");
                }
            }
        });
    } else {
        context.uart.puts("COW manager not initialized\r\n");
    }
}

/// COW Statistics command
pub fn cmd_cow_stats(_args: &[&str], context: &mut ShellContext) {
    context.uart.puts("\r\n=== COW Statistics ===\r\n");

    if let Some(stats) =
        crate::memory::with_cow_manager(|cow_manager| cow_manager.get_statistics().clone())
    {
        context.uart.puts("Total COW Pages: ");
        print_number(&context.uart, stats.cow_pages_count as u32);
        context.uart.puts("\r\n");

        context.uart.puts("Total COW Faults: ");
        print_number(&context.uart, stats.cow_faults_handled as u32);
        context.uart.puts("\r\n");

        context.uart.puts("Pages Duplicated: ");
        print_number(&context.uart, stats.pages_duplicated as u32);
        context.uart.puts("\r\n");

        context.uart.puts("Memory Saved: ");
        print_number(&context.uart, stats.memory_saved_bytes as u32);
        context.uart.puts(" bytes\r\n");

        context.uart.puts("Metadata Memory: ");
        print_number(&context.uart, stats.metadata_memory_bytes as u32);
        context.uart.puts(" bytes\r\n");

        context.uart.puts("Peak COW Pages: ");
        print_number(&context.uart, stats.peak_cow_pages as u32);
        context.uart.puts("\r\n");

        // Calculate efficiency
        if stats.cow_pages_count > 0 {
            let efficiency =
                (stats.memory_saved_bytes * 100) / (stats.cow_pages_count as u64 * 4096);
            context.uart.puts("Memory Efficiency: ");
            print_number(&context.uart, efficiency as u32);
            context.uart.puts("%\r\n");
        }
    } else {
        context.uart.puts("COW manager not initialized\r\n");
    }
}

/// COW Create Mapping command
pub fn cmd_cow_create(_args: &[&str], context: &mut ShellContext) {
    context.uart.puts("\r\n=== Creating COW Mapping ===\r\n");

    // For demonstration, create a simple COW mapping
    let test_virt_addr = 0x10000000u64;
    let test_phys_addr = 0x20000000u64;

    if let Some(result) = crate::memory::with_cow_manager(|cow_manager| {
        cow_manager.create_cow_mapping(
            test_virt_addr,
            test_virt_addr + 0x1000,
            1, // source process
            2, // dest process
            test_phys_addr,
            crate::memory::RegionType::UserData,
        )
    }) {
        match result {
            Ok(()) => {
                context.uart.puts("COW mapping created successfully\r\n");
                context.uart.puts("Source VA: 0x");
                print_hex(&context.uart, test_virt_addr);
                context.uart.puts("\r\nDest VA: 0x");
                print_hex(&context.uart, test_virt_addr + 0x1000);
                context.uart.puts("\r\nPhysical: 0x");
                print_hex(&context.uart, test_phys_addr);
                context.uart.puts("\r\n");
            }
            Err(e) => {
                context.uart.puts("COW mapping failed: ");
                context.uart.puts(e);
                context.uart.puts("\r\n");
            }
        }
    } else {
        context.uart.puts("COW manager not initialized\r\n");
    }
}

/// COW Protection command
pub fn cmd_cow_protect(_args: &[&str], context: &mut ShellContext) {
    context.uart.puts("\r\n=== Forcing COW Protection ===\r\n");

    let test_phys_addr = 0x20000000u64;

    if let Some(result) = crate::memory::with_cow_manager(|cow_manager| {
        cow_manager.force_cow_protection(test_phys_addr)
    }) {
        match result {
            Ok(()) => {
                context.uart.puts("COW protection enabled for page 0x");
                print_hex(&context.uart, test_phys_addr);
                context.uart.puts("\r\n");
            }
            Err(e) => {
                context.uart.puts("COW protection failed: ");
                context.uart.puts(e);
                context.uart.puts("\r\n");
            }
        }
    } else {
        context.uart.puts("COW manager not initialized\r\n");
    }
}

/// COW Unprotect command
pub fn cmd_cow_unprotect(_args: &[&str], context: &mut ShellContext) {
    context.uart.puts("\r\n=== Removing COW Protection ===\r\n");

    let test_phys_addr = 0x20000000u64;

    if let Some(result) = crate::memory::with_cow_manager(|cow_manager| {
        cow_manager.remove_cow_protection(test_phys_addr)
    }) {
        match result {
            Ok(()) => {
                context.uart.puts("COW protection removed from page 0x");
                print_hex(&context.uart, test_phys_addr);
                context.uart.puts("\r\n");
            }
            Err(e) => {
                context.uart.puts("COW protection removal failed: ");
                context.uart.puts(e);
                context.uart.puts("\r\n");
            }
        }
    } else {
        context.uart.puts("COW manager not initialized\r\n");
    }
}

/// COW Test command
pub fn cmd_cow_test(_args: &[&str], context: &mut ShellContext) {
    context.uart.puts("\r\n=== COW Test Suite ===\r\n");

    if let Some((__tests_passed, __tests_failed)) = crate::memory::with_cow_manager(|cow_manager| {
        let mut _tests_passed = 0;
        let mut _tests_failed = 0;

        // Test 1: Register a page
        context.uart.puts("Test 1: Register COW page... ");
        let test_phys = 0x30000000u64;
        let test_virt = 0x40000000u64;
        match cow_manager.register_page(
            test_phys,
            test_virt,
            crate::memory::RegionType::UserData,
            1,
        ) {
            Ok(()) => {
                context.uart.puts("PASS\r\n");
                _tests_passed += 1;
            }
            Err(e) => {
                context.uart.puts("FAIL (");
                context.uart.puts(e);
                context.uart.puts(")\r\n");
                _tests_failed += 1;
            }
        }

        // Test 2: Check if page is COW protected
        context.uart.puts("Test 2: Check COW protection... ");
        if cow_manager.is_cow_protected(test_phys) {
            context.uart.puts("PASS\r\n");
            _tests_passed += 1;
        } else {
            context.uart.puts("FAIL (not protected)\r\n");
            _tests_failed += 1;
        }

        // Test 3: Add another reference to trigger COW
        context.uart.puts("Test 3: Add second reference... ");
        match cow_manager.register_page(
            test_phys,
            test_virt + 0x1000,
            crate::memory::RegionType::UserData,
            2,
        ) {
            Ok(()) => {
                context.uart.puts("PASS\r\n");
                _tests_passed += 1;
            }
            Err(e) => {
                context.uart.puts("FAIL (");
                context.uart.puts(e);
                context.uart.puts(")\r\n");
                _tests_failed += 1;
            }
        }

        // Test 4: Verify COW protection is enabled
        context.uart.puts("Test 4: Verify COW protection... ");
        if cow_manager.is_cow_protected(test_phys) {
            context.uart.puts("PASS\r\n");
            _tests_passed += 1;
        } else {
            context
                .uart
                .puts("FAIL (not protected after multiple refs)\r\n");
            _tests_failed += 1;
        }

        // Test 5: Simulate COW fault
        context.uart.puts("Test 5: Simulate COW fault... ");
        let cow_fault =
            crate::memory::create_cow_fault_from_exception(test_virt, test_phys, true, 1);

        match cow_manager.handle_cow_fault(cow_fault) {
            Ok(new_page) => {
                context.uart.puts("PASS (new page: 0x");
                print_hex(&context.uart, new_page);
                context.uart.puts(")\r\n");
                _tests_passed += 1;
            }
            Err(e) => {
                context.uart.puts("FAIL (");
                context.uart.puts(e);
                context.uart.puts(")\r\n");
                _tests_failed += 1;
            }
        }

        // Summary
        context.uart.puts("\r\nTest Summary:\r\n");
        context.uart.puts("  Passed: ");
        print_number(&context.uart, _tests_passed);
        context.uart.puts("\r\n");
        context.uart.puts("  Failed: ");
        print_number(&context.uart, _tests_failed);
        context.uart.puts("\r\n");

        if _tests_failed == 0 {
            context.uart.puts("All COW tests passed!\r\n");
        } else {
            context.uart.puts("Some COW tests failed.\r\n");
        }

        (_tests_passed, _tests_failed)
    }) {
        context.uart.puts("COW manager not initialized\r\n");
    }
}
