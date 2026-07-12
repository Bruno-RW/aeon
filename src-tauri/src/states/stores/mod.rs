use crate::collectors::{CPUTelemetry, RAMTelemetry};

#[derive(Default, Debug, Clone, serde::Serialize)]
/// All telemetry data.
pub struct TelemetryStore {
  cpu_telemetry: CPUTelemetry,
  ram_telemetry: RAMTelemetry,
}
