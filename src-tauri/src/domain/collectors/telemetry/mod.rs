#[derive(Default, Debug, Clone, serde::Serialize)]
/// CPU telemetry data.
pub struct CPUTelemetry {
  pub usage: u8,
  pub temperature: Option<u8>,
}

#[derive(Default, Debug, Clone, serde::Serialize)]
/// RAM telemetry data.
pub struct RAMTelemetry {
  pub usage: u8,
  pub temperature: Option<u8>,
}
