use std::sync::{Arc, RwLock};
use std::time::Duration;
use std::thread::{self, sleep};

use sysinfo::System;

use crate::domain::collectors::Collector;
use crate::infrastructure::collectors::{
  DEFAULT_COLLECTOR_INTERVAL,
  CPUCollector,
  RAMCollector
};

use super::super::state::TelemetryStore;

pub struct TelemetryCollectorWorker {
  telemetry_store: Arc<RwLock<TelemetryStore>>,
  interval: Duration,
}

impl TelemetryCollectorWorker {
  pub fn new(
    telemetry_store: Arc<RwLock<TelemetryStore>>,
    interval: Option<Duration>,
  ) -> Self {
    Self {
      telemetry_store,
      interval: interval.unwrap_or(DEFAULT_COLLECTOR_INTERVAL),
    }
  }

  pub fn spawn(self) {
    let telemetry_store = self.telemetry_store;
    let interval = self.interval;

    thread::spawn(move || {
      let mut system = System::new_all();
      let cpu_collector = CPUCollector;
      let ram_collector = RAMCollector;

      loop {
        system.refresh_all();

        let cpu_telemetry = cpu_collector.collect(&system);
        let ram_telemetry = ram_collector.collect(&system);

        {
          let mut store = telemetry_store.write().unwrap();
          store.update(cpu_telemetry.clone(), ram_telemetry.clone());
        }

        println!("CPU: {:?} | RAM: {:?}", cpu_telemetry, ram_telemetry);

        sleep(interval);
      }
    });
  }
}
