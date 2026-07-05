//! CPU, GPU and RAM Collectors main module.
pub mod constants;
pub use constants::DEFAULT_POLL_INTERVAL;

pub mod cpu;
pub use cpu::CPUCollector;

// pub mod gpu;
// pub use gpu::gpuCollector;

pub mod ram;
pub use ram::RAMCollector;
