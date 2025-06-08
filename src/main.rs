#[tokio::main(flavor = "current_thread")]
async fn main() {
  run_msix();
}

use windortortent::common::run_as_admin;
use windows::Win32::System::Com::{CoInitializeEx, COINIT_APARTMENTTHREADED, COINIT_DISABLE_OLE1DDE};

fn run_msix() {
  unsafe {
    _ = CoInitializeEx(None, COINIT_APARTMENTTHREADED | COINIT_DISABLE_OLE1DDE);
  };

  let code  = run_as_admin(r"powershell.exe", None).unwrap();
  println!("{code}");

  
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
