use crate::domain::collectors::{CPUTelemetry, RAMTelemetry};

#[derive(Default, Debug, Clone, serde::Serialize)]
/// All telemetry data.
pub struct TelemetryStore {
  pub cpu_telemetry: CPUTelemetry,
  pub ram_telemetry: RAMTelemetry,
}

impl TelemetryStore {
  /// Updates the telemetry store with new data.
  pub fn update(
    &mut self,
    cpu_telemetry: CPUTelemetry,
    ram_telemetry: RAMTelemetry,
  ) {
    self.cpu_telemetry = cpu_telemetry;
    self.ram_telemetry = ram_telemetry;
  }
}
