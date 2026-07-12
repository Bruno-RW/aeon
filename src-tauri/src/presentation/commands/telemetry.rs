use crate::appplication::state::{AppState, TelemetryStore};

#[tauri::command]
pub fn get_telemetry_store(state: tauri::State<AppState>) -> TelemetryStore {
  state.telemetry_store.read().unwrap().clone()
}
