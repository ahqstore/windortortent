#[tokio::main(flavor = "current_thread")]
async fn main() {
  run_msix();
}

use windortortent::{
  ahqdb::{AHQDBApplication, BasicShortcutInfo}, common::run_as_admin, zip::link::Type
};
use windows::Win32::System::Com::{
  COINIT_APARTMENTTHREADED, COINIT_DISABLE_OLE1DDE, CoInitializeEx,
};

fn run_msix() {
  unsafe {
    _ = CoInitializeEx(None, COINIT_APARTMENTTHREADED | COINIT_DISABLE_OLE1DDE);
  };

  let cmd = r#"-ExecutionPolicy Bypass -NoExit -Command "& { `$env:AHQ=`"welp`" ; (cat `"E:\GitHub\windortortent\install.ps1`" | iex)" }"#;

  println!("{cmd}");

  run_as_admin("powershell.exe", Some(cmd)).unwrap();

  // let mut app = AHQDBApplication::new(
  //   "./app.ahqdb",
  //   "initial",
  //   BasicShortcutInfo {
  //     desktop: false,
  //     start_menu_folder: Some("AHQ Store Applications"),
  //   },
  // )
  // .unwrap();

  // app.install("packages", Type::CurrentUser).unwrap();

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
