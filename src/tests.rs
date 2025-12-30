use crate::winrt::{MSIXPackageManager, metadata::MsixBundle};

#[tokio::test]
pub async fn install_msix() {
  let man = MSIXPackageManager::new().expect("Unable to create package manager");

  let mut bundle = MsixBundle::load("./app.MsixBundle", &man)
    .await
    .expect("Unable to load msixbundle");

  #[allow(deprecated)]
  let is_installed = unsafe { bundle.async_unsafe_is_installed().await }.expect("Is it installed?");

  assert!(!is_installed);

  bundle.install().await.expect("Unable to install");

  #[allow(deprecated)]
  let is_installed = unsafe { bundle.async_unsafe_is_installed().await }.expect("Is it installed?");

  assert!(is_installed);

  bundle.uninstall().await.expect("Unable to uninstall!");
}
