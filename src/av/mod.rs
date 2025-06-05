use windows::{core::w, Win32::System::Antimalware::{AmsiInitialize, HAMSICONTEXT}};

pub fn test() {
  let ctx: HAMSICONTEXT = unsafe { AmsiInitialize(w!("Windortortent")).unwrap() };
}