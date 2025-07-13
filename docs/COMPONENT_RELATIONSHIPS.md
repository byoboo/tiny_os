# TinyOS Component Relationships

## System Architecture Overview

```text
┌─────────────────────────────────────────────────────────────────┐
│                        TinyOS System                            │
├─────────────────────────────────────────────────────────────────┤
│  Interactive Shell                                              │
│  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐ │
│  │  Shell Commands │  │  Command Parser │  │   Shell Core    │ │
│  │                 │  │                 │  │                 │ │
│  │ • Memory Cmds   │  │ • Argument      │  │ • Main Loop     │ │
│  │ • System Cmds   │  │   Processing    │  │ • State Mgmt    │ │
│  │ • Hardware Cmds │  │ • Help System   │  │ • User I/O      │ │
│  │ • Protection    │  │ • Error Handling│  │ • Command Exec  │ │
│  └─────────────────┘  └─────────────────┘  └─────────────────┘ │
├─────────────────────────────────────────────────────────────────┤
│  Memory Management Subsystem                                   │
│  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐ │
│  │  Page Manager   │  │  Protection     │  │   User Space    │ │
│  │                 │  │   Manager       │  │    Manager      │ │
│  │ • Page Tables   │  │ • ASLR          │  │ • Process Pages │ │
│  │ • MMU Control   │  │ • Stack Guard   │  │ • COW Memory    │ │
│  │ • TLB Mgmt      │  │ • CFI           │  │ • Memory Stats  │ │
│  └─────────────────┘  └─────────────────┘  └─────────────────┘ │
│                                                                 │
│  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐ │
│  │   Allocator     │  │  Defragmenter   │  │  Memory Stats   │ │
│  │                 │  │                 │  │                 │ │
│  │ • Heap Mgmt     │  │ • Compaction    │  │ • Usage Track   │ │
│  │ • Free Lists    │  │ • Relocation    │  │ • Performance   │ │
│  │ • Buddy System  │  │ • Optimization  │  │ • Debugging     │ │
│  └─────────────────┘  └─────────────────┘  └─────────────────┘ │
├─────────────────────────────────────────────────────────────────┤
│  Hardware Abstraction Layer                                    │
│  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐ │
│  │     Drivers     │  │   Interrupts    │  │   Exceptions    │ │
│  │                 │  │                 │  │                 │ │
│  │ • UART          │  │ • IRQ Handling  │  │ • Sync Exc      │ │
│  │ • GPIO          │  │ • Timer         │  │ • Async Exc     │ │
│  │ • SD Card       │  │ • System Timer  │  │ • SError        │ │
│  │ • Hardware I/O  │  │ • Scheduling    │  │ • Debug Exc     │ │
│  └─────────────────┘  └─────────────────┘  └─────────────────┘ │
├─────────────────────────────────────────────────────────────────┤
│  Process Management                                             │
│  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐ │
│  │   Scheduler     │  │  Context Switch │  │   Process       │ │
│  │                 │  │                 │  │   Control       │ │
│  │ • Task Queue    │  │ • Register Save │  │ • Process State │ │
│  │ • Priorities    │  │ • Stack Switch  │  │ • Privilege     │ │
│  │ • Time Slicing  │  │ • MMU Switch    │  │ • Isolation     │ │
│  └─────────────────┘  └─────────────────┘  └─────────────────┘ │
├─────────────────────────────────────────────────────────────────┤
│  Core Kernel                                                   │
│  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐ │
│  │   Boot Loader   │  │  System Init    │  │   Main Loop     │ │
│  │                 │  │                 │  │                 │ │
│  │ • ARM64 Boot    │  │ • MMU Setup     │  │ • Kernel Entry  │ │
│  │ • Stack Setup   │  │ • Driver Init   │  │ • Shell Launch  │ │
│  │ • Memory Init   │  │ • Exception Vec │  │ • System Ready  │ │
│  └─────────────────┘  └─────────────────┘  └─────────────────┘ │
└─────────────────────────────────────────────────────────────────┘
```

## Data Flow Patterns

### Memory Allocation Flow

```text
Shell Command → Memory Manager → Page Manager → MMU → Physical Memory
     ↓              ↓              ↓          ↓           ↓
  Arguments    Validate Request  Allocate   Map Pages  Update TLB
     ↓              ↓              ↓          ↓           ↓
  Parse Args   Check Permissions  Update     Configure  Return Address
     ↓              ↓              ↓          ↓           ↓
  Execute      Update Statistics  Return     Update     Success/Error
```

### Exception Handling Flow

```text
Hardware Exception → Exception Vector → Exception Handler → System Recovery
        ↓                    ↓                 ↓                  ↓
   Exception Type        Save Context     Decode ESR         Restore State
        ↓                    ↓                 ↓                  ↓
   Interrupt/Sync       Save Registers   Handle Specific    Return to Code
        ↓                    ↓                 ↓                  ↓
   Route to Handler     Stack Switch     Memory/Process     Continue Execution
```

### Shell Command Flow

```text
User Input → Shell Parser → Command Lookup → System Call → Response
     ↓            ↓              ↓              ↓           ↓
  Raw Command  Parse Arguments  Find Handler  Execute Op  Format Output
     ↓            ↓              ↓              ↓           ↓
  Validation   Validate Args    Check Perms   Access HW   Display Result
     ↓            ↓              ↓              ↓           ↓
  Execute      Build Command    Call Function Return Data  Update State
```

## Component Dependencies

### Memory Management Dependencies

- **Page Manager** depends on:
  - MMU hardware abstraction
  - Physical memory layout
  - Exception handling for page faults

- **Protection Manager** depends on:
  - Page Manager for memory mapping
  - Process Manager for isolation
  - Hardware features (ASLR, CFI)

- **User Space Manager** depends on:
  - Page Manager for page allocation
  - Protection Manager for security
  - Process Manager for context

### Shell System Dependencies

- **Command Modules** depend on:
  - Memory managers for system operations
  - Hardware drivers for I/O operations
  - Process manager for system state

- **Shell Core** depends on:
  - UART driver for I/O
  - Exception handling for error recovery
  - Memory manager for shell state

### Hardware Layer Dependencies

- **Drivers** depend on:
  - Memory manager for DMA buffers
  - Interrupt system for async operations
  - Exception system for error handling

- **Exception System** depends on:
  - Memory manager for stack management
  - Process manager for context switching
  - Hardware registers for state save/restore

## Critical Interaction Points

### Memory-Shell Interface

```rust
// Shell commands access memory subsystem through well-defined interfaces
pub trait MemoryOperations {
    fn allocate_pages(&mut self, count: usize) -> Result<VirtualAddress, MemoryError>;
    fn protect_region(&mut self, addr: VirtualAddress, size: usize, perms: Permissions) -> Result<(), MemoryError>;
    fn get_statistics(&self) -> MemoryStatistics;
}
```

### Memory-Process Interface

```rust
// Process manager coordinates with memory for isolation
pub trait ProcessMemory {
    fn create_address_space(&mut self) -> Result<AddressSpace, ProcessError>;
    fn map_user_pages(&mut self, space: &AddressSpace, pages: &[PhysicalPage]) -> Result<(), ProcessError>;
    fn switch_context(&mut self, from: &ProcessContext, to: &ProcessContext) -> Result<(), ProcessError>;
}
```

### Hardware-Memory Interface

```rust
// Memory manager uses hardware abstraction for MMU operations
pub trait HardwareMemory {
    fn map_page(&mut self, virtual_addr: VirtualAddress, physical_addr: PhysicalAddress, flags: PageFlags) -> Result<(), HardwareError>;
    fn flush_tlb(&mut self, addr: Option<VirtualAddress>);
    fn get_fault_address(&self) -> VirtualAddress;
}
```

## State Management

### Global System State

- **Memory Statistics**: Centralized tracking of all memory usage
- **Process Table**: List of active processes and their state
- **Hardware State**: Current configuration of MMU, timers, etc.
- **Shell State**: Current command context and history

### Thread-Safe Patterns

```rust
// All mutable global state uses thread-safe patterns
static MEMORY_MANAGER: Mutex<MemoryManager> = Mutex::new(MemoryManager::new());
static PROCESS_MANAGER: Mutex<ProcessManager> = Mutex::new(ProcessManager::new());
static HARDWARE_STATE: Mutex<HardwareState> = Mutex::new(HardwareState::new());
```

### State Synchronization

- **Memory Operations**: Atomic updates to page tables and allocation state
- **Process Context**: Careful ordering of context switches and memory mapping
- **Hardware Config**: Synchronized access to hardware registers and state

## Error Propagation

### Error Handling Chain

```text
Hardware Error → Driver Error → System Error → Shell Error → User Feedback
      ↓               ↓             ↓             ↓              ↓
  Hardware Fault  Driver Failure  System Fault  Command Error  Error Message
      ↓               ↓             ↓             ↓              ↓
  Exception      Error Return   Graceful Fail  Help Message   Continue Shell
```

### Recovery Strategies

- **Memory Errors**: Attempt alternative allocation strategies
- **Hardware Errors**: Retry operations with backoff
- **Process Errors**: Isolate failures to prevent system corruption
- **Shell Errors**: Provide helpful error messages and continue operation

This component relationship map provides a comprehensive view of how all the major systems in TinyOS interact with each other, making it easier to understand the codebase and make informed changes.
