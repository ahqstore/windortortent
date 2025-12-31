#![feature(sync_nonpoison, nonpoison_mutex)]

#[cfg(feature = "legacy-experimental-ahqdb")]
#[deprecated(
  since = "0.0.1",
  note = "'legacy_ahqdb' is outdated. Please use the 'ahqdb' module instead."
)]
pub mod ahqdb_legacy;

pub mod ahqdb;

pub mod av;
pub mod exe;
pub mod msi;
pub mod winrt;
pub mod zip;

pub mod common;
pub mod utils;

pub use windows;
pub use windows::ApplicationModel::Package;

pub type ApplicationPackage = Package;

#[cfg(test)]
pub mod tests;
