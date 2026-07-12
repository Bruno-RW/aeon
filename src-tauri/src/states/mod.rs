use std::sync::{Arc, RwLock};

mod stores;
pub use stores::TelemetryStore;

/// Application shared state.
pub struct AppState {
  pub telemetry_store: Arc<RwLock<TelemetryStore>>,
}

// ? Arc (Atomic Reference Count): shared ownership across threads
// ? RwLock (Read-Write Lock): safe concurrent access with many readers or one writer
