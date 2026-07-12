use std::sync::RwLock;

mod stores;
pub use stores::TelemetryStore;

pub struct AppState {
  pub telemetry_store: RwLock<TelemetryStore>,
}
