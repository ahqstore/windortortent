pub mod winrt;
pub mod msi;

pub use windows;
pub use windows::ApplicationModel::Package;

#[cfg(test)]
mod test {
  #[tokio::test]
  async fn run_test() {
    use crate::winrt::metadata::MsixBundle; 
    use crate::winrt::UWPPackageManager;

    let manager = UWPPackageManager::new().unwrap();
    let mut x = MsixBundle::load("./app.Msixbundle", &manager).await.unwrap();

    println!("{x:#?}");

    x.install().await.unwrap();
    
    println!("Installed!");

    #[allow(deprecated)]
    let inst = unsafe { x.async_unsafe_is_installed().await.unwrap() };

    println!("Installed: {inst}");

    x.uninstall().await.unwrap();

    println!("Uninstalled!");
  }
}