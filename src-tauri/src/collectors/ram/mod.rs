use sysinfo::System;

use super::Collector;

mod telemetry;
pub use telemetry::RAMTelemetry;

/// Collects RAM metrics.
pub struct RAMCollector;

impl RAMCollector {
  /// Returns the RAM usage as a percentage.
  pub fn get_usage(system: &System) -> u8 {
    let total_memory: f64 = system.total_memory() as f64;
    let used_memory: f64 = system.used_memory() as f64;

    let usage: f64 = (used_memory / total_memory * 100.0).round();
    return usage as u8;
  }
}

impl Collector for RAMCollector {
  type Telemetry = RAMTelemetry;

  fn collect(&self, system: &System) -> Self::Telemetry {
    let usage = Self::get_usage(system);

    return Self::Telemetry {
      usage,
      temperature: None,
    }
  }
}
