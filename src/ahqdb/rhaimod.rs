use rhai::plugin::*;
use std::env::consts::{ARCH, OS};

#[export_module]
pub mod system {
  pub const PLATFORM: &'static str = OS;
  pub const CPUARCH: &'static str = ARCH;
}
