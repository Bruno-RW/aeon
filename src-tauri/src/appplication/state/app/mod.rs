use std::sync::{Arc, RwLock};

use super::TelemetryStore;


/// Application shared state.
pub struct AppState {
  pub telemetry_store: Arc<RwLock<TelemetryStore>>

  // ? Arc (Atomic Reference Count): shared ownership across threads
  // ? RwLock (Read-Write Lock): safe concurrent access with many readers or one writer
}

impl AppState{
  pub fn new() -> Self {
    Self {
      telemetry_store: Arc::new(RwLock::new( TelemetryStore::default() )),
    }
  }
}
