#[cfg(feature = "legacy-experimental-ahqdb")]
pub mod ahqdb_legacy;

#[cfg(feature = "experimental-ahqdb")]
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
