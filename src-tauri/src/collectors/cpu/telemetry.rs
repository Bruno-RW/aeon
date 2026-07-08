#[derive(Debug, Clone, serde::Serialize)]
/// CPU telemetry data.
pub struct CPUTelemetry {
  pub usage: u8,
  pub temperature: Option<u8>,
}
