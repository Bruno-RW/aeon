//! CPU collection logic.

use sysinfo::System;
// use std::time::Duration;

// use super::DEFAULT_POLL_INTERVAL;


/// Collects CPU metrics and stores the latest sampled values.
pub struct CPUCollector
{
  // duration: Duration,
  // usage: f32,
  // temperature: f32,
}

/// Methods for creating and updating the CPU collector.
impl CPUCollector {
  // /// Creates a new CPU collector with the default polling interval.
  // pub fn new() -> Self {
  //   Self::new_with_duration(DEFAULT_POLL_INTERVAL)
  // }

  // /// Creates a new CPU collector with the given polling interval.
  // pub fn new_with_duration(duration: Duration) -> Self {
  //   Self {
  //     duration,
  //     usage: 0.0,
  //     temperature: 0.0,
  //   }
  // }

  /// Returns the CPU usage as a percentage.
  pub fn get_usage(system: &System) -> f32 {
    return system.global_cpu_usage().round();
  }
}
