//! CPU, GPU and RAM Collectors main module.
mod constants;
pub use constants::DEFAULT_POLL_INTERVAL;

mod _trait;
pub use _trait::Collector;

mod cpu;
// pub use cpu::{CPUTelemetry, CPUCollector};
pub use cpu::{CPUCollector};
