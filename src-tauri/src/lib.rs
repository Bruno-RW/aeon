use tauri::Manager;

mod domain;
mod infrastructure;
mod appplication;

use appplication::{
  state::AppState,
  workers::TelemetryCollectorWorker,
};


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .plugin(tauri_plugin_opener::init())
    .manage(AppState::new())
    .setup(|app| {
      let telemetry_store = app.state::<AppState>().telemetry_store.clone();
      TelemetryCollectorWorker::new(telemetry_store, None).spawn();
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
