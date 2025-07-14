// Week 4 Advanced Hardware Commands
// PCIe, Power Management, and Advanced Benchmarking Commands

use crate::shell::ShellContext;
use crate::drivers::pcie::{get_pcie_controller, init_pcie, PcieError};
use crate::drivers::power_management::{
    get_power_controller, get_power_controller_mut, init_power_management, 
    PowerConfig, PowerBias, CpuFrequency, GpuPowerState, PeripheralPower
};
use crate::benchmarks::week4_advanced::{
    run_week4_benchmark, quick_week4_validation, AdvancedBenchmarkType
};

/// Initialize Week 4 hardware features
pub fn cmd_week4_init(args: &[&str], context: &mut ShellContext) {
    context.print("🚀 Initializing Week 4 Advanced Features...\n");
    
    let enable_dma_opt = args.get(1).unwrap_or(&"true") == &"true";
    let enable_power_mgmt = args.get(2).unwrap_or(&"true") == &"true";
    
    // Initialize PCIe controller
    context.print("📡 Initializing PCIe 2.0 Controller...\n");
    match init_pcie(enable_dma_opt, enable_power_mgmt) {
        Ok(()) => {
            context.print("✅ PCIe Controller initialized successfully\n");
            
            if let Some(pcie_controller) = get_pcie_controller() {
                let metrics = pcie_controller.get_performance_metrics();
                context.print(&format!("   • Devices found: {}\n", metrics.device_count));
                context.print(&format!("   • Link speed: {} Mbps\n", metrics.link_speed_mbps));
                context.print(&format!("   • Enumeration time: {} cycles\n", metrics.enumeration_time_cycles));
            }
        },
        Err(PcieError::LinkTrainingTimeout) => {
            context.print("⚠️  PCIe link training timeout (no devices connected)\n");
        },
        Err(e) => {
            context.print(&format!("❌ PCIe initialization failed: {:?}\n", e));
        }
    }
    
    // Initialize Power Management
    context.print("\n⚡ Initializing Advanced Power Management...\n");
    let power_config = PowerConfig {
        enable_cpu_scaling: true,
        enable_gpu_power_control: enable_power_mgmt,
        enable_peripheral_gating: true,
        enable_thermal_management: true,
        temperature_threshold_celsius: 70,
        performance_bias: PowerBias::Balanced,
    };
    
    match init_power_management(power_config) {
        Ok(()) => {
            context.print("✅ Power Management initialized successfully\n");
            
            if let Some(power_controller) = get_power_controller() {
                let metrics = power_controller.get_metrics();
                context.print(&format!("   • CPU frequency: {} MHz\n", metrics.current_cpu_freq_hz / 1_000_000));
                context.print(&format!("   • GPU state: {:?}\n", metrics.current_gpu_state));
                context.print(&format!("   • Power consumption: {} mW\n", metrics.power_consumption_mw));
                context.print(&format!("   • Temperature: {}°C\n", metrics.temperature_celsius));
            }
        },
        Err(e) => {
            context.print(&format!("❌ Power Management initialization failed: {:?}\n", e));
        }
    }
    
    context.print("\n🎉 Week 4 initialization complete!\n");
    context.print("Use 'week4 status' to view system status\n");
    context.print("Use 'week4 benchmark' to run performance tests\n");
}

/// Show Week 4 system status
pub fn cmd_week4_status(args: &[&str], context: &mut ShellContext) {
    context.print("\n📊 Week 4 Advanced Hardware Status\n");
    context.print("==================================\n");
    
    // PCIe Status
    context.print("\n📡 PCIe Controller Status:\n");
    if let Some(pcie_controller) = get_pcie_controller() {
        let state = pcie_controller.get_state();
        let metrics = pcie_controller.get_performance_metrics();
        
        context.print(&format!("   • State: {:?}\n", state));
        context.print(&format!("   • Devices enumerated: {}\n", metrics.device_count));
        context.print(&format!("   • Link speed: {} Mbps\n", metrics.link_speed_mbps));
        context.print(&format!("   • DMA transfer rate: {} Mbps\n", metrics.dma_transfer_rate_mbps));
        context.print(&format!("   • Interrupt latency: {} cycles\n", metrics.interrupt_latency_cycles));
        
        // Show discovered devices
        if metrics.device_count > 0 {
            context.print("\n   📋 Discovered Devices:\n");
            let devices = pcie_controller.get_devices();
            for (idx, device_opt) in devices.iter().enumerate() {
                if let Some(device) = device_opt {
                    context.print(&format!("     {}. {:?} (Vendor: 0x{:04X}, Device: 0x{:04X})\n", 
                        idx + 1, device.device_type, device.vendor_id, device.device_id));
                }
            }
        }
    } else {
        context.print("   ❌ PCIe Controller not initialized\n");
    }
    
    // Power Management Status
    context.print("\n⚡ Power Management Status:\n");
    if let Some(power_controller) = get_power_controller() {
        let metrics = power_controller.get_metrics();
        let config = power_controller.get_config();
        
        context.print(&format!("   • CPU Frequency: {} MHz\n", metrics.current_cpu_freq_hz / 1_000_000));
        context.print(&format!("   • GPU Power State: {:?}\n", metrics.current_gpu_state));
        context.print(&format!("   • Temperature: {}°C ({})\n", 
            metrics.temperature_celsius, 
            match metrics.thermal_state {
                crate::drivers::power_management::ThermalState::Normal => "Normal",
                crate::drivers::power_management::ThermalState::Warm => "Warm",
                crate::drivers::power_management::ThermalState::Hot => "Hot",
                crate::drivers::power_management::ThermalState::Critical => "Critical",
            }));
        context.print(&format!("   • Power Consumption: {} mW\n", metrics.power_consumption_mw));
        context.print(&format!("   • Power Savings: {}%\n", metrics.power_savings_percent));
        context.print(&format!("   • Active Peripherals: {} devices\n", metrics.active_peripherals.count_ones()));
        context.print(&format!("   • Throttling Events: {}\n", metrics.throttling_events));
        context.print(&format!("   • Frequency Changes: {}\n", metrics.frequency_changes));
        context.print(&format!("   • Performance Bias: {:?}\n", config.performance_bias));
    } else {
        context.print("   ❌ Power Management not initialized\n");
    }
    
    // Integration Status
    context.print("\n🔗 Week 3/4 Integration Status:\n");
    let week3_gpu = crate::drivers::videocore::get_videocore_controller().is_some();
    let week3_dma = crate::drivers::dma::get_dma_controller().is_some();
    let week4_pcie = get_pcie_controller().is_some();
    let week4_power = get_power_controller().is_some();
    
    context.print(&format!("   • Week 3 GPU Integration: {}\n", if week3_gpu { "✅ Active" } else { "❌ Inactive" }));
    context.print(&format!("   • Week 3 DMA Optimization: {}\n", if week3_dma { "✅ Active" } else { "❌ Inactive" }));
    context.print(&format!("   • Week 4 PCIe Controller: {}\n", if week4_pcie { "✅ Active" } else { "❌ Inactive" }));
    context.print(&format!("   • Week 4 Power Management: {}\n", if week4_power { "✅ Active" } else { "❌ Inactive" }));
    
    let integration_score = (week3_gpu as u32 + week3_dma as u32 + week4_pcie as u32 + week4_power as u32) * 25;
    context.print(&format!("   • Overall Integration: {}% complete\n", integration_score));
}

/// Run Week 4 benchmarks
pub fn cmd_week4_benchmark(args: &[&str], context: &mut ShellContext) {
    let benchmark_type = args.get(1).unwrap_or(&"comprehensive");
    
    match benchmark_type {
        "pcie" => {
            context.print("🚀 Running PCIe Performance Benchmark...\n");
            run_week4_benchmark(AdvancedBenchmarkType::PciePerformance, context);
        },
        "power" => {
            context.print("⚡ Running Power Efficiency Benchmark...\n");
            run_week4_benchmark(AdvancedBenchmarkType::PowerEfficiency, context);
        },
        "thermal" => {
            context.print("🌡️  Running Thermal Management Benchmark...\n");
            run_week4_benchmark(AdvancedBenchmarkType::ThermalManagement, context);
        },
        "apps" => {
            context.print("🏆 Running Real-World Application Benchmark...\n");
            run_week4_benchmark(AdvancedBenchmarkType::ApplicationScenarios, context);
        },
        "integration" => {
            context.print("🔗 Running Hardware Integration Benchmark...\n");
            run_week4_benchmark(AdvancedBenchmarkType::HardwareIntegration, context);
        },
        "quick" => {
            quick_week4_validation(context);
        },
        _ => {
            context.print("🚀 Running Comprehensive Week 4 Benchmark Suite...\n");
            run_week4_benchmark(AdvancedBenchmarkType::ApplicationScenarios, context); // Full suite
        }
    }
}

/// Control CPU frequency
pub fn cmd_week4_cpu_freq(args: &[&str], context: &mut ShellContext) {
    if let Some(mut power_controller) = get_power_controller_mut() {
        if args.len() < 2 {
            context.print("Usage: week4 cpu-freq <min|low|medium|high|max>\n");
            return;
        }
        
        let frequency = match args[1] {
            "min" => CpuFrequency::Min,
            "low" => CpuFrequency::Low,
            "medium" => CpuFrequency::Medium,
            "high" => CpuFrequency::High,
            "max" => CpuFrequency::Max,
            _ => {
                context.print("Invalid frequency. Use: min, low, medium, high, max\n");
                return;
            }
        };
        
        context.print(&format!("🔧 Setting CPU frequency to {:?}...\n", frequency));
        match power_controller.set_cpu_frequency(frequency) {
            Ok(()) => {
                let metrics = power_controller.get_metrics();
                context.print(&format!("✅ CPU frequency set to {} MHz\n", metrics.current_cpu_freq_hz / 1_000_000));
                context.print(&format!("   Power consumption: {} mW\n", metrics.power_consumption_mw));
            },
            Err(e) => {
                context.print(&format!("❌ Failed to set CPU frequency: {:?}\n", e));
            }
        }
    } else {
        context.print("❌ Power management not initialized\n");
    }
}

/// Control GPU power state
pub fn cmd_week4_gpu_power(args: &[&str], context: &mut ShellContext) {
    if let Some(mut power_controller) = get_power_controller_mut() {
        if args.len() < 2 {
            context.print("Usage: week4 gpu-power <off|idle|reduced|full>\n");
            return;
        }
        
        let state = match args[1] {
            "off" => GpuPowerState::Off,
            "idle" => GpuPowerState::Idle,
            "reduced" => GpuPowerState::Reduced,
            "full" => GpuPowerState::Full,
            _ => {
                context.print("Invalid state. Use: off, idle, reduced, full\n");
                return;
            }
        };
        
        context.print(&format!("🎮 Setting GPU power state to {:?}...\n", state));
        match power_controller.set_gpu_power_state(state) {
            Ok(()) => {
                let metrics = power_controller.get_metrics();
                context.print(&format!("✅ GPU power state set to {:?}\n", metrics.current_gpu_state));
                context.print(&format!("   Power consumption: {} mW\n", metrics.power_consumption_mw));
            },
            Err(e) => {
                context.print(&format!("❌ Failed to set GPU power state: {:?}\n", e));
            }
        }
    } else {
        context.print("❌ Power management not initialized\n");
    }
}

/// Show Week 4 help
pub fn cmd_week4_help(_args: &[&str], context: &mut ShellContext) {
    context.print("\n🚀 WEEK 4 ADVANCED HARDWARE COMMANDS\n");
    context.print("=====================================\n\n");
    
    context.print("📡 PCIe & Hardware:\n");
    context.print("  week4 init [dma_opt] [power_mgmt]  - Initialize Week 4 features\n");
    context.print("  week4 status                       - Show comprehensive system status\n");
    context.print("  week4 devices                      - List discovered PCIe devices\n\n");
    
    context.print("⚡ Power Management:\n");
    context.print("  week4 cpu-freq <level>             - Set CPU frequency (min/low/medium/high/max)\n");
    context.print("  week4 gpu-power <state>            - Set GPU power state (off/idle/reduced/full)\n");
    context.print("  week4 thermal                      - Show thermal management status\n\n");
    
    context.print("🏆 Benchmarking:\n");
    context.print("  week4 benchmark [type]             - Run benchmarks (pcie/power/thermal/apps/integration)\n");
    context.print("  week4 benchmark quick              - Quick validation test\n");
    context.print("  week4 benchmark comprehensive      - Full benchmark suite\n\n");
    
    context.print("🔗 Integration:\n");
    context.print("  week4 integration                  - Show Week 3/4 integration status\n");
    context.print("  week4 performance                  - Show performance optimization summary\n\n");
    
    context.print("Use 'week4 <command>' to execute commands\n");
    context.print("Week 4 builds on Week 3 GPU integration for advanced Pi 4/5 optimization!\n");
}

/// Show discovered PCIe devices
pub fn cmd_week4_devices(_args: &[&str], context: &mut ShellContext) {
    context.print("\n📋 Discovered PCIe Devices\n");
    context.print("==========================\n");
    
    if let Some(pcie_controller) = get_pcie_controller() {
        let devices = pcie_controller.get_devices();
        let metrics = pcie_controller.get_performance_metrics();
        
        if metrics.device_count == 0 {
            context.print("No PCIe devices found.\n");
            context.print("This is normal if no PCIe cards are connected to the Pi.\n");
            return;
        }
        
        for (idx, device_opt) in devices.iter().enumerate() {
            if let Some(device) = device_opt {
                context.print(&format!("\n🔌 Device {}: {:?}\n", idx + 1, device.device_type));
                context.print(&format!("   • Vendor ID: 0x{:04X}\n", device.vendor_id));
                context.print(&format!("   • Device ID: 0x{:04X}\n", device.device_id));
                context.print(&format!("   • Class Code: 0x{:06X}\n", device.class_code));
                context.print(&format!("   • Bus: {}, Device: {}, Function: {}\n", 
                    device.bus, device.device, device.function));
                context.print(&format!("   • DMA Capable: {}\n", if device.dma_capable { "Yes" } else { "No" }));
                context.print(&format!("   • IRQ Line: {}\n", device.irq_line));
                
                // Show BAR addresses if non-zero
                for (bar_idx, &bar_addr) in device.bar_addresses.iter().enumerate() {
                    if bar_addr != 0 {
                        context.print(&format!("   • BAR{}: 0x{:016X}\n", bar_idx, bar_addr));
                    }
                }
            }
        }
        
        context.print(&format!("\nTotal devices: {}\n", metrics.device_count));
        context.print(&format!("Link configuration: {} Mbps\n", metrics.link_speed_mbps));
    } else {
        context.print("❌ PCIe controller not initialized\n");
        context.print("Run 'week4 init' first to initialize PCIe\n");
    }
}

/// Show thermal management status
pub fn cmd_week4_thermal(_args: &[&str], context: &mut ShellContext) {
    context.print("\n🌡️  Thermal Management Status\n");
    context.print("=============================\n");
    
    if let Some(power_controller) = get_power_controller() {
        let metrics = power_controller.get_metrics();
        
        context.print(&format!("Current Temperature: {}°C\n", metrics.temperature_celsius));
        context.print(&format!("Thermal State: {:?}\n", metrics.thermal_state));
        context.print(&format!("Throttling Events: {}\n", metrics.throttling_events));
        
        // Temperature status
        let temp_status = match metrics.thermal_state {
            crate::drivers::power_management::ThermalState::Normal => "✅ Normal - Full performance available",
            crate::drivers::power_management::ThermalState::Warm => "⚠️  Warm - Slight performance reduction possible",
            crate::drivers::power_management::ThermalState::Hot => "🔥 Hot - Performance throttling active",
            crate::drivers::power_management::ThermalState::Critical => "🚨 Critical - Aggressive throttling in effect",
        };
        context.print(&format!("Status: {}\n", temp_status));
        
        // Thermal recommendations
        context.print("\n📋 Thermal Recommendations:\n");
        match metrics.thermal_state {
            crate::drivers::power_management::ThermalState::Normal => {
                context.print("  • System operating within normal thermal limits\n");
                context.print("  • All performance features available\n");
            },
            crate::drivers::power_management::ThermalState::Warm => {
                context.print("  • Consider improving case ventilation\n");
                context.print("  • Monitor workload intensity\n");
            },
            crate::drivers::power_management::ThermalState::Hot => {
                context.print("  • Add heatsink or improve cooling\n");
                context.print("  • Reduce workload intensity\n");
                context.print("  • Check ambient temperature\n");
            },
            crate::drivers::power_management::ThermalState::Critical => {
                context.print("  • 🚨 IMMEDIATE ACTION REQUIRED\n");
                context.print("  • Add active cooling (fan)\n");
                context.print("  • Reduce system load\n");
                context.print("  • Check for blocked ventilation\n");
            },
        }
    } else {
        context.print("❌ Power management not initialized\n");
        context.print("Run 'week4 init' first to initialize thermal management\n");
    }
}

/// Main Week 4 command dispatcher
pub fn cmd_week4(args: &[&str], context: &mut ShellContext) {
    if args.is_empty() {
        cmd_week4_help(args, context);
        return;
    }
    
    match args[0] {
        "init" => cmd_week4_init(args, context),
        "status" => cmd_week4_status(args, context),
        "benchmark" => cmd_week4_benchmark(args, context),
        "cpu-freq" => cmd_week4_cpu_freq(args, context),
        "gpu-power" => cmd_week4_gpu_power(args, context),
        "devices" => cmd_week4_devices(args, context),
        "thermal" => cmd_week4_thermal(args, context),
        "integration" => cmd_week4_status(args, context), // Reuse status for integration
        "performance" => cmd_week4_benchmark(args, context), // Reuse benchmark for performance
        "help" => cmd_week4_help(args, context),
        _ => {
            context.print(&format!("Unknown Week 4 command: {}\n", args[0]));
            context.print("Use 'week4 help' for available commands\n");
        }
    }
}
