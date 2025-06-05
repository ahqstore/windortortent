pub mod av;
pub mod exe;
pub mod msi;
pub mod winrt;
pub mod zip;

pub mod utils;

pub use windows;
pub use windows::ApplicationModel::Package;

pub type ApplicationPackage = Package;

#[cfg(test)]
mod test {
  use windows::Win32::System::Com::CoInitialize;

  use crate::utils::user_desktop;
  use crate::zip::link::{Type, link};
  use crate::zip::{ZipInstaller, ZipShortcut};

  #[tokio::test]
  async fn run_msix() {
    unsafe {
      _ = CoInitialize(None);
    };

    println!("{}", user_desktop().unwrap());

    let mut app = ZipInstaller::new(
      "./scrcpy.zip",
      ZipShortcut {
        name: "Scrcpy",
        exe: "scrcpy.exe",
        args: None,
        icon: None,
        description: Some("Screen Mirroring for Android"),
        desktop: true,
        start_menu_dir: Some("AHQ Store Applications")
      },
    )
    .unwrap();
    app.install("./dist", Type::CurrentUser).unwrap();
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
