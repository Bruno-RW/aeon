use std::sync::RwLock;
use std::thread::sleep;

use sysinfo::{System};

mod collectors;
use collectors::{
  DEFAULT_POLL_INTERVAL,
  Collector,
  CPUCollector,
  RAMCollector,
};

mod states;
use states::{AppState, TelemetryStore};


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .plugin(tauri_plugin_opener::init())
    .manage(AppState {
      telemetry_store: RwLock::new(TelemetryStore::default()),
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");

  let mut system = System::new_all();
  let cpu_collector = CPUCollector;
  let ram_collector = RAMCollector;

  loop {
    system.refresh_all();

    let cpu_telemetry = cpu_collector.collect(&system);
    println!("{:?}", cpu_telemetry);

    let ram_telemetry = ram_collector.collect(&system);
    println!("{:?}", ram_telemetry);

    sleep(DEFAULT_POLL_INTERVAL);
  }
}
