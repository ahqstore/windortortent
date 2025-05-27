pub mod winrt;
pub mod msi;

pub use windows;
pub use windows::ApplicationModel::Package;

#[cfg(test)]
mod test {
  use std::fs;

  #[tokio::test]
  async fn install() {
    use crate::winrt::UWPPackageManager;

    let man = UWPPackageManager::new().unwrap();

    let path = fs::canonicalize("./app.Msixbundle").unwrap();
    let path = path.to_str().unwrap().replace(r"\\?\", "");

    man.install(format!("file://{path}")).await.unwrap();
  }

  #[tokio::test]
  #[cfg(feature = "metadata")]
  async fn run_metadata_test() {
    use crate::winrt::UWPPackageManager;

    let man = UWPPackageManager::new().unwrap();

    let path = fs::canonicalize("./app.Msixbundle").unwrap();
    let path = path.to_str().unwrap().replace(r"\\?\", "");

    man.get_intalled_info_sync(

    ).await.unwrap();
  }

  #[tokio::test]
  #[cfg(feature = "metadata")]
  async fn get_installed_package() {
    use crate::winrt::UWPPackageManager;

    let man = UWPPackageManager::new().unwrap();

    let path = fs::canonicalize("./app.Msixbundle").unwrap();
    let path = path.to_str().unwrap().replace(r"\\?\", "");

    man.get_intalled_info_sync(

    ).await.unwrap();
  }

  #[tokio::test]
  async fn uninstall() {
    use crate::winrt::UWPPackageManager;

    let man = UWPPackageManager::new().unwrap();

    let path = fs::canonicalize("./app.Msixbundle").unwrap();
    let path = path.to_str().unwrap().replace(r"\\?\", "");

    man.remove(format!("file://{path}")).await.unwrap();
  }
}