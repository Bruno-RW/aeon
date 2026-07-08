// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


use sysinfo::{System};

mod collectors;
use collectors::{
  DEFAULT_POLL_INTERVAL,
  Collector,
  CPUCollector,
};


fn main() {
  let mut system = System::new_all();
  let cpu_collector = CPUCollector;

  loop {
    system.refresh_all();

    let telemetry = cpu_collector.collect(&system);
    println!("{:?}", telemetry);

    std::thread::sleep(DEFAULT_POLL_INTERVAL);
  }

  // aeon_lib::run()
}
