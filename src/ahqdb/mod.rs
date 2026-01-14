//! This crate defines the newer async-friendly version of AHQDB
//! This is not the same as the older AHQ DB
//!
//! This ahqdb outputs a list of instructions to perform
//! as an outputs
//!
//! after extracting the ahqdb
use serde::{Deserialize, Serialize};

use rhai::{
  Dynamic, Engine, EvalAltResult, FnNamespace, FuncRegistration, Module, NativeCallContext,
  Position, exported_module,
};
use std::{
  fs::File,
  io::Read,
  ops::Deref,
  path::Path,
  sync::{Arc, nonpoison::Mutex},
};
use toml::from_str;
use zip::ZipArchive;

use crate::ahqdb::{
  rhaimod::system,
  validation::{PathError, secure_logical_resolve},
};

mod rhaimod;
mod validation;

pub enum ActionInstruction {
  /// Download the data and copy it to the location (relative)
  DownloadCopy {
    asset: String,
    path: String,
  },
  DownloadUnzip {
    asset: String,
    dir: String,
  },
}

pub struct AHQDBInstruction {
  /// May be `true` only if update
  pub patch_old: bool,
  pub inst: Vec<ActionInstruction>,
}

pub enum UpdateInfo {
  None,
  Update { old_root: String },
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AHQDBManifest {
  pub windows: AHQDBWindowsManifest,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AHQDBWindowsManifest {
  pub aumid: Option<String>,
  pub links: Vec<AHQDBWindowsLink>,
  pub allowed: Vector,
}

#[derive(Debug)]
pub struct Vector(Arc<Vec<String>>);

impl Deref for Vector {
  type Target = Vec<String>;

  fn deref(&self) -> &Self::Target {
    self.0.as_ref()
  }
}

impl Serialize for Vector {
  fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    self.0.as_ref().serialize(serializer)
  }
}

impl<'de> Deserialize<'de> for Vector {
  fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>,
  {
    let data: Vec<String> = Deserialize::deserialize(deserializer)?;
    Ok(Vector(Arc::new(data)))
  }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AHQDBWindowsLink {
  pub name: String,
  pub exe: String,
  pub description: Option<String>,
  pub args: Option<String>,
  // Default (exe, 0)
  pub icon: Option<(String, i32)>,
}

pub type Result<T> = core::result::Result<T, AHQDBError>;

#[derive(Debug)]
pub enum AHQDBError {
  JoinError(tokio::task::JoinError),
  TokioIO(tokio::io::Error),
  ZipError(zip::result::ZipError),
  Serde(toml::de::Error),
  Win32(windows_core::Error),
  InvalidScript,
  SyncError,
  Path(PathError),
}

impl From<PathError> for AHQDBError {
  fn from(value: PathError) -> Self {
    AHQDBError::Path(value)
  }
}

impl From<windows_core::Error> for AHQDBError {
  fn from(value: windows_core::Error) -> Self {
    AHQDBError::Win32(value)
  }
}

impl From<toml::de::Error> for AHQDBError {
  fn from(value: toml::de::Error) -> Self {
    Self::Serde(value)
  }
}

impl From<tokio::task::JoinError> for AHQDBError {
  fn from(value: tokio::task::JoinError) -> Self {
    Self::JoinError(value)
  }
}

impl From<zip::result::ZipError> for AHQDBError {
  fn from(value: zip::result::ZipError) -> Self {
    Self::ZipError(value)
  }
}

impl From<tokio::io::Error> for AHQDBError {
  fn from(value: tokio::io::Error) -> Self {
    Self::TokioIO(value)
  }
}

pub struct AHQDB<'a> {
  pub app_id: &'a str,
  zip: ZipArchive<File>,
  pub manifest: AHQDBWindowsManifest,
}

impl<'a> AHQDB<'a> {
  pub fn open(app_id: &'a str, path: &str) -> Result<Self> {
    let ahqdb = File::open(path)?;
    let mut zip = ZipArchive::new(ahqdb)?;

    let mut file = zip.by_path("./manifest.toml")?;

    let mut data = String::default();
    file.read_to_string(&mut data)?;

    let man: AHQDBManifest = from_str(&data)?;
    drop(file);

    Ok(Self {
      app_id,
      zip,
      manifest: man.windows,
    })
  }

  fn generate_rhai() -> Engine {
    let mut eng = Engine::new();

    let system = exported_module!(system);

    eng.register_global_module(system.into());

    eng
      .set_max_operations(100_000)
      .set_max_array_size(50)
      .set_max_functions(10)
      .set_max_map_size(10)
      .set_max_call_levels(10)
      .set_max_variables(20)
      .set_max_string_size(256 * 1024)
      .set_allow_loop_expressions(false)
      .set_allow_anonymous_fn(true)
      .set_allow_looping(true)
      .set_max_expr_depths(40, 10);

    eng
  }

  pub fn get_instructions(&mut self, update: UpdateInfo) -> Result<AHQDBInstruction> {
    let mut eng = Self::generate_rhai();

    let data: Arc<Mutex<_>> = Arc::new(Mutex::new(AHQDBInstruction {
      inst: vec![],
      patch_old: false,
    }));

    /* Get the script */
    let mut script_file = self
      .zip
      .by_path(if matches!(update, UpdateInfo::Update { .. }) {
        "./scripts/update.rhai"
      } else {
        "./scripts/install.rhai"
      })?;

    if script_file.size() > 256 * 1024 {
      return Err(AHQDBError::InvalidScript);
    }

    /* Setup Modules */
    let mut script = String::default();
    script_file.read_to_string(&mut script)?;

    drop(script_file);

    let mut module = Module::new();
    {
      let perms1 = self.manifest.allowed.0.clone();
      let perms2 = self.manifest.allowed.0.clone();

      let data1 = data.clone();
      let data2 = data.clone();
      FuncRegistration::new("download_and_unzip")
        .with_namespace(FnNamespace::Global)
        .set_into_module(&mut module, move |asset: String, dir: String| {
          let mut data = data1.lock();

          if data.inst.len() >= 30 {
            Err::<(), _>("Error: Only 30 instructions are allowed!")?;
          }

          if perms1.contains(&asset) && secure_logical_resolve(None, Path::new("/"), &dir).is_ok() {
            data
              .inst
              .push(ActionInstruction::DownloadUnzip { asset, dir });
            return Ok(());
          }

          return Err(Box::new(EvalAltResult::ErrorRuntime(
            Dynamic::from("Error, unverified asset or path, check"),
            Position::START,
          )));
        });

      FuncRegistration::new("download_and_copy")
        .with_namespace(FnNamespace::Global)
        .set_into_module(&mut module, move |asset: String, path: String| {
          let mut data = data2.lock();

          if data.inst.len() >= 30 {
            Err::<(), _>("Error: Only 30 instructions are allowed!")?;
          }

          if perms2.contains(&asset) && secure_logical_resolve(None, Path::new("/"), &path).is_ok()
          {
            data
              .inst
              .push(ActionInstruction::DownloadCopy { asset, path });
            return Ok(());
          }

          return Err(Box::new(EvalAltResult::ErrorRuntime(
            Dynamic::from("Error, unverified asset"),
            Position::START,
          )));
        });

      if let UpdateInfo::Update { old_root } = update {
        let data3 = data.clone();

        let peek: Arc<*mut ZipArchive<File>> = Arc::new(&mut self.zip);

        FuncRegistration::new("parse_json")
          .with_namespace(FnNamespace::Global)
          .set_into_module(&mut module, move |ctx: NativeCallContext, data: &str| {
            if data.len() > 128 * 1024 {
              Err::<rhai::Map>("Too long")?;
            }

            ctx.engine().parse_json(data, true)
          });

        FuncRegistration::new("fs_read_string")
          .with_namespace(FnNamespace::Global)
          .set_into_module(&mut module, move |file: &str| {
            // 1. Resolve the path and Reader based on the scheme
            let (reader, size_hint) = if let Some(path_str) = file.strip_prefix("old://") {
              let path = secure_logical_resolve(Some("old://"), Path::new(&old_root), path_str)
                .map_err(|_| "Error, invalid path")?;

              let file = File::open(path).map_err(|_| "Error, invalid path")?;
              let size = file.metadata().map(|m| m.len()).unwrap_or(0);
              (Box::new(file) as Box<dyn Read>, size)
            } else if let Some(path_str) = file.strip_prefix("new://") {
              let path = secure_logical_resolve(Some("new://"), Path::new("./dist"), path_str)
                .map_err(|_| "Error, invalid path")?;

              let backend = unsafe { &mut **peek.as_ref() };

              let f = backend.by_path(path).map_err(|_| "Error, invalid path")?;
              let size = f.size(); // Assuming this is from your custom trait

              // We need to read 'f' here. If 'f' implements Read, we can box it.
              // If 'f' isn't a standard Read, you might need to read it to a string here directly.
              // For this example, assuming 'f' implies a reader:
              (Box::new(f) as Box<dyn Read>, size)
            } else {
              return Err(Box::new(EvalAltResult::ErrorRuntime(
                Dynamic::from("Error, invalid root"),
                Position::START,
              )));
            };

            // 2. Enforce Size Limit (Unified Logic)
            if size_hint > 256 * 1024 {
              return Err(Box::new(EvalAltResult::ErrorRuntime(
                Dynamic::from("Error, too large"),
                Position::START,
              )));
            }

            // 3. Perform the Read (Unified Logic)
            // Using 'take' strictly enforces the limit during the read operation,
            // fixing the race condition for standard files.
            let mut data = String::new();
            reader
              .take(256 * 1024 + 1)
              .read_to_string(&mut data)
              .map_err(|_| "Error, read failed")?;

            // Double check explicit size in case 'size_hint' was wrong (e.g. file grew)
            if data.len() > 256 * 1024 {
              return Err(Box::new(EvalAltResult::ErrorRuntime(
                Dynamic::from("Error, too large"),
                Position::START,
              )));
            }

            Ok(data)
          });

        FuncRegistration::new("commit_old")
          .with_namespace(FnNamespace::Global)
          .set_into_module(&mut module, move || {
            let mut locked = data3.lock();
            if locked.inst.len() != 0 {
              return Err(Box::new(EvalAltResult::ErrorRuntime(
                Dynamic::from(
                  "Error, trying to commit to a different location after emitting requests.",
                ),
                Position::START,
              )));
            }

            locked.patch_old = true;

            Ok(())
          });
      }
    }

    eng.register_static_module("ahqstore", module.into());

    eng.eval(&script).map_err(|_| AHQDBError::InvalidScript)?;

    drop(eng);

    let data = Arc::try_unwrap(data)
      .map_err(|_| AHQDBError::SyncError)?
      .into_inner();

    Ok(data)
  }
}
