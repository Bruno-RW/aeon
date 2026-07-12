use std::sync::{Arc, RwLock};
use std::thread::{self, sleep};

use sysinfo::System;
use tauri::Manager;

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
      telemetry_store: Arc::new(RwLock::new( TelemetryStore::default() )),
    })
    .setup(|app| {
      // ? Clones shared store handle, not whole store
      let telemetry_store = app.state::<AppState>().telemetry_store.clone();

      // ? Moves telemetry_store into thread to avoid ownership issues
      thread::spawn(move || {
        let mut system = System::new_all();
        let cpu_collector = CPUCollector;
        let ram_collector = RAMCollector;

        loop {
          system.refresh_all();

          let cpu_telemetry = cpu_collector.collect(&system);
          let ram_telemetry = ram_collector.collect(&system);

          // ? Creates smaller scope that opens a write lock, writes data, and releases lock when done
          {
            let mut store = telemetry_store.write().unwrap();
            store.update(cpu_telemetry.clone(), ram_telemetry.clone());
          }

          println!("CPU: {:?} | RAM: {:?}", cpu_telemetry, ram_telemetry);

          sleep(DEFAULT_POLL_INTERVAL);
        }
      });

      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
