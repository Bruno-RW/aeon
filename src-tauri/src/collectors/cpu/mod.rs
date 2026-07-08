use sysinfo::System;

use super::Collector;

mod telemetry;
pub use telemetry::CPUTelemetry;

/// Collects CPU metrics.
pub struct CPUCollector;

impl CPUCollector {
  /// Returns the CPU usage as a percentage.
  fn get_usage(system: &System) -> u8 {
    let usage: f32 = system.global_cpu_usage().round();
    return usage as u8;
  }
}

impl Collector for CPUCollector {
  type Telemetry = CPUTelemetry;

  fn collect(&self, system: &System) -> Self::Telemetry {
    let usage = Self::get_usage(system);

    return Self::Telemetry {
      usage,
      temperature: None,
    }
  }
}
