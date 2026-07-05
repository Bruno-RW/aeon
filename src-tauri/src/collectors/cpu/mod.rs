//! CPU collection logic.

use sysinfo::System;

/// Collects CPU metrics.
pub struct CPUCollector {}

impl CPUCollector {
  /// Returns the CPU usage as a percentage.
  pub fn get_usage(system: &System) -> u8 {
    let usage: f32 = system.global_cpu_usage().round();
    return usage as u8;
  }
}
