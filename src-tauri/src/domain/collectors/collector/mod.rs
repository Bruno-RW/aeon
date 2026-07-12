use sysinfo::System;

/// Trait for collectors that collect metrics for a component.
pub trait Collector {
  type Telemetry;

  // Returns all metrics for the component.
  fn collect(&self, system: &System) -> Self::Telemetry;
}
