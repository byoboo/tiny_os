# TinyOS Memory Testing Guide

## 🧠 Memory Testing Architecture

TinyOS has a **dual-layer testing approach** for memory functionality:

### 1. 🏗️ **External Integration Tests** (Shell Scripts)

**Location**: `tests/scripts/test_memory_*.sh`
**Purpose**: Test what can be validated from outside the kernel
**Scope**: Limited to build validation and boot sequence checking

**What these tests CAN do:**

- ✅ Verify memory modules compile correctly
- ✅ Check system boots with memory manager
- ✅ Validate source code structure
- ✅ Test integration with other systems

**What these tests CANNOT do:**

- ❌ Test actual memory allocation/deallocation
- ❌ Test MMU page table operations
- ❌ Test memory protection mechanisms
- ❌ Test copy-on-write functionality
- ❌ Test virtual memory management
- ❌ Test dynamic memory features

### 2. 🎯 **Internal Kernel Tests** (Rust Framework)

**Location**: `src/testing/` directory
**Purpose**: Test actual memory functionality from inside the running kernel
**Scope**: Comprehensive testing of all memory features

**What these tests CAN do:**

- ✅ Test memory allocation and deallocation
- ✅ Test MMU operations and page tables
- ✅ Test memory protection and access control
- ✅ Test copy-on-write mechanisms
- ✅ Test virtual memory management
- ✅ Test dynamic memory features
- ✅ Test memory statistics and monitoring
- ✅ Test memory fault handling

## 🚀 **How to Test Memory Functionality**

### Method 1: Complete Memory Testing (Recommended)

```bash
cargo run          # Boot TinyOS
TinyOS> t          # Run complete test suite including memory tests
```

### Method 2: Memory-Specific Testing

```bash
cargo run          # Boot TinyOS
TinyOS> mmu        # Test MMU functionality
TinyOS> memory     # Test memory management
TinyOS> stats      # View memory statistics
```

### Method 3: External Integration Testing

```bash
./tests/scripts/test_memory_automated.sh     # Basic integration tests
./test_tinyos.sh memory                      # Run memory integration suite
```

## 📊 **Test Coverage**

| Feature | External Tests | Internal Tests |
|---------|---------------|----------------|
| Build System | ✅ | ✅ |
| Boot Integration | ✅ | ✅ |
| Memory Allocation | ❌ | ✅ |
| MMU Operations | ❌ | ✅ |
| Virtual Memory | ❌ | ✅ |
| Copy-on-Write | ❌ | ✅ |
| Memory Protection | ❌ | ✅ |
| Dynamic Memory | ❌ | ✅ |
| Fault Handling | ❌ | ✅ |

## 🔧 **Testing Framework Files**

### External Tests (Shell Scripts)

- `test_memory_automated.sh` - Basic memory integration
- `test_memory_suite.sh` - Interactive memory testing
- `test_memory_modular.sh` - Modular architecture testing

### Internal Tests (Rust)

- `src/testing/mod.rs` - Test runner framework
- `src/testing/mmu_tests.rs` - MMU and virtual memory tests
- `src/testing/kernel_tests.rs` - Kernel memory tests
- `src/testing/integration_tests.rs` - Cross-system integration

## 💡 **Best Practices**

1. **For Development**: Use internal tests (`cargo run` → `t`)
2. **For CI/CD**: Use external tests (`test_tinyos.sh`)
3. **For Debugging**: Use specific commands (`mmu`, `memory`, `stats`)
4. **For Validation**: Use both approaches complementarily

## 🎯 **Why This Architecture?**

**Shell scripts** run outside the kernel and can only observe external behavior (boot messages, compilation success, etc.). They cannot access internal kernel data structures or perform actual memory operations.

**Internal Rust tests** run inside the kernel with full access to memory management structures, allowing comprehensive testing of actual functionality.

This dual approach ensures both **integration validation** and **functional correctness**.
