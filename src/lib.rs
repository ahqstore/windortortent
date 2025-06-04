pub mod winrt;
pub mod msi;
pub mod exe;
pub mod zip;
pub mod av;

pub mod utils;

pub use windows;
pub use windows::ApplicationModel::Package;

pub type ApplicationPackage = Package;

#[cfg(test)]
mod test {
    use windows::Win32::System::Com::CoInitialize;

    use crate::utils::user_desktop;
    use crate::zip::link::{link, Type};
    use crate::zip::ZipShortcut;

  #[tokio::test]
  async fn run_msix() {
    unsafe {
      _ = CoInitialize(None);
    };

    println!("{}", user_desktop().unwrap());

    link(
      &ZipShortcut {
        args: Some("--nothing guys"),
        description: Some("Some high level wizardry"),
        exe: "scrcpy.exe",
        icon: Some((r"C:\Users\Windows\Downloads\icon.ico", 0)),
        name: "AHQ Softwares"
      },
      format!("./data"),
      Type::AllUsers
    ).await.unwrap();
    // use crate::winrt::metadata::MsixBundle; 
    // use crate::winrt::MSIXPackageManager;

    // let manager = MSIXPackageManager::new().unwrap();
    // let mut x = MsixBundle::load("./app.Msixbundle", &manager).await.unwrap();

    // println!("{x:#?}");

    // x.install().await.unwrap();
    
    // println!("Installed!");

    // #[allow(deprecated)]
    // let inst = unsafe { x.async_unsafe_is_installed().await.unwrap() };

    // println!("Installed: {inst}");

    // x.uninstall().await.unwrap();

    // println!("Uninstalled!");
  }
}