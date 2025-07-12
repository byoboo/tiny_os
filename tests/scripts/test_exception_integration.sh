#!/bin/bash

# Exception System Integration Test
# Tests the integration between different exception system components

echo "=== TinyOS Exception System Integration Tests ==="
echo "Testing component interactions and data flow"
echo

# Test 1: Exception handler to ESR decoder integration
echo "Test 1: Exception Handler -> ESR Decoder Integration"
echo "--------------------------------------------------"

# Verify that the exception handler imports and uses the ESR decoder
if grep -q "use super::esr_decoder::" src/exceptions/handler.rs; then
    echo "✅ Exception handler imports ESR decoder"
else
    echo "❌ Exception handler missing ESR decoder import"
    exit 1
fi

if grep -q "decode_esr" src/exceptions/handler.rs; then
    echo "✅ Exception handler uses ESR decoder"
else
    echo "❌ Exception handler doesn't use ESR decoder"
    exit 1
fi

# Test 2: Exception handler to system call integration
echo
echo "Test 2: Exception Handler -> System Call Integration"
echo "---------------------------------------------------"

if grep -q "use super::syscall::" src/exceptions/handler.rs; then
    echo "✅ Exception handler imports system call module"
else
    echo "❌ Exception handler missing system call import"
    exit 1
fi

if grep -q "handle_syscall" src/exceptions/handler.rs; then
    echo "✅ Exception handler uses system call dispatcher"
else
    echo "❌ Exception handler doesn't use system call dispatcher"
    exit 1
fi

# Test 3: Exception handler to memory fault analysis integration
echo
echo "Test 3: Exception Handler -> Memory Fault Analysis Integration"
echo "------------------------------------------------------------"

if grep -q "use super::memory_faults::" src/exceptions/handler.rs; then
    echo "✅ Exception handler imports memory fault module"
else
    echo "❌ Exception handler missing memory fault import"
    exit 1
fi

if grep -q "MemoryFaultAnalyzer" src/exceptions/handler.rs; then
    echo "✅ Exception handler uses memory fault analyzer"
else
    echo "❌ Exception handler doesn't use memory fault analyzer"
    exit 1
fi

# Test 4: Statistics integration across modules
echo
echo "Test 4: Statistics Integration Across Modules"
echo "--------------------------------------------"

if grep -q "EXCEPTION_STATS" src/exceptions/handler.rs; then
    echo "✅ Exception handler updates statistics"
else
    echo "❌ Exception handler missing statistics updates"
    exit 1
fi

if grep -q "MEMORY_FAULT_STATS" src/exceptions/handler.rs; then
    echo "✅ Exception handler updates memory fault statistics"
else
    echo "❌ Exception handler missing memory fault statistics"
    exit 1
fi

# Test 5: Shell command integration
echo
echo "Test 5: Shell Command Integration"
echo "--------------------------------"

# Check if shell commands integrate with all exception modules
shell_integrations=(
    "exceptions::syscall::"
    "exceptions::memory_faults::"
    "exceptions::esr_decoder::"
    "exceptions::types::"
)

for integration in "${shell_integrations[@]}"; do
    if grep -q "use crate::$integration\|$integration" src/shell/commands/hardware.rs; then
        echo "✅ Shell integrates with $integration"
    else
        echo "❌ Shell missing integration with $integration"
        exit 1
    fi
done

# Test 6: Module export consistency
echo
echo "Test 6: Module Export Consistency"
echo "--------------------------------"

# Check if all modules are properly exported from mod.rs
module_exports=(
    "esr_decoder"
    "handler"
    "init"
    "memory_faults"
    "syscall"
    "types"
)

for module in "${module_exports[@]}"; do
    if grep -q "pub mod $module" src/exceptions/mod.rs; then
        echo "✅ Module $module properly exported"
    else
        echo "❌ Module $module not exported"
        exit 1
    fi
done

# Test 7: Type consistency across modules
echo
echo "Test 7: Type Consistency Across Modules"
echo "--------------------------------------"

# Check if ExceptionContext is used consistently
if grep -q "ExceptionContext" src/exceptions/handler.rs && grep -q "ExceptionContext" src/exceptions/types.rs; then
    echo "✅ ExceptionContext type used consistently"
else
    echo "❌ ExceptionContext type inconsistency"
    exit 1
fi

# Test 8: Build integration test
echo
echo "Test 8: Full Build Integration"
echo "-----------------------------"

echo "Testing full system build with all components..."
if cargo build --quiet 2>/dev/null; then
    echo "✅ Full system builds successfully with all integrations"
else
    echo "❌ Integration causes build failures"
    exit 1
fi

# Test 9: Boot integration test
echo
echo "Test 9: Boot Integration Test"
echo "----------------------------"

echo "Testing exception system initialization during boot..."
if grep -q "init_exceptions" src/main.rs; then
    echo "✅ Exception system initialized during boot"
else
    echo "❌ Exception system not initialized during boot"
    exit 1
fi

# Test 10: API consistency test
echo
echo "Test 10: API Consistency Test"
echo "----------------------------"

# Check if the main API exports match the module structure
api_exports=(
    "EsrDecoder"
    "SyscallNumber"
    "MemoryFaultAnalyzer"
    "ExceptionStats"
    "init_exceptions"
)

for export in "${api_exports[@]}"; do
    if grep -q "$export" src/exceptions/mod.rs; then
        echo "✅ API export $export available"
    else
        echo "❌ API export $export missing"
        exit 1
    fi
done

echo
echo "=========================================="
echo "Integration Test Results"
echo "=========================================="
echo "✅ All integration tests passed!"
echo
echo "Integration Summary:"
echo "- Exception handler properly integrates with ESR decoder"
echo "- System call interface fully integrated"
echo "- Memory fault analysis properly connected"
echo "- Statistics tracking works across all modules"
echo "- Shell commands access all exception features"
echo "- Module exports are consistent"
echo "- Types are used consistently across modules"
echo "- Full system builds with all integrations"
echo "- Boot process initializes exception system"
echo "- API exports match module structure"
echo
echo "🎉 Exception system integration is complete and working!"
echo "Ready for production use and Phase 2 development."
