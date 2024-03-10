use std::io::Write;
use std::path::PathBuf;
use deno_cache::SqliteBackedCache;
use deno_core::snapshot::create_snapshot as create_v8_snapshot;
use deno_core::snapshot::CreateSnapshotOptions;
use deno_core::Extension;
use std::path::Path;
use deno_core::error::AnyError;
use deno_webidl;
use deno_console;
use deno_url;
use deno_web;
use deno_webgpu;
use deno_canvas;
use deno_fetch;
use deno_cache;
use deno_websocket;
use deno_ffi;
use deno_net;

pub fn create_snapshot() {
  let cargo_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
  let snapshot = cargo_dir.join("snapshot.bin");

  std::fs::remove_file(&snapshot).unwrap();

  let extensions: Vec<Extension> = vec![
    deno_webidl::deno_webidl::init_ops_and_esm(),
    deno_console::deno_console::init_ops_and_esm(),
    deno_url::deno_url::init_ops_and_esm(),
    deno_web::deno_web::init_ops_and_esm::<Permissions>(
      Default::default(),
      Default::default(),
    ),
    deno_webgpu::deno_webgpu::init_ops_and_esm(),
    deno_canvas::deno_canvas::init_ops_and_esm(),
    deno_fetch::deno_fetch::init_ops_and_esm::<Permissions>(Default::default()),
    deno_cache::deno_cache::init_ops_and_esm::<SqliteBackedCache>(None),
    deno_websocket::deno_websocket::init_ops_and_esm::<Permissions>(
      "".to_owned(),
      None,
      None,
    ),
  ];

  let output = create_v8_snapshot(CreateSnapshotOptions {
    cargo_manifest_dir: env!("CARGO_MANIFEST_DIR"),
    startup_snapshot: None,
    skip_op_registration: false,
    extensions,
    with_runtime_cb: None,
    extension_transpiler: None,
  }, None).unwrap();

  let mut snapshot = std::fs::File::create(snapshot).unwrap();
  snapshot.write_all(&output.output).unwrap();
}

#[derive(Clone)]
struct Permissions;

impl deno_fetch::FetchPermissions for Permissions {
  fn check_net_url(
    &mut self,
    _url: &deno_core::url::Url,
    _api_name: &str,
  ) -> Result<(), deno_core::error::AnyError> {
    unreachable!("snapshotting!")
  }

  fn check_read(
    &mut self,
    _p: &Path,
    _api_name: &str,
  ) -> Result<(), deno_core::error::AnyError> {
    unreachable!("snapshotting!")
  }
}

impl deno_websocket::WebSocketPermissions for Permissions {
  fn check_net_url(
    &mut self,
    _url: &deno_core::url::Url,
    _api_name: &str,
  ) -> Result<(), deno_core::error::AnyError> {
    unreachable!("snapshotting!")
  }
}

impl deno_web::TimersPermission for Permissions {
  fn allow_hrtime(&mut self) -> bool {
    unreachable!("snapshotting!")
  }
}

impl deno_ffi::FfiPermissions for Permissions {
  fn check_partial(
    &mut self,
    _path: Option<&Path>,
  ) -> Result<(), deno_core::error::AnyError> {
    unreachable!("snapshotting!")
  }
}

impl deno_napi::NapiPermissions for Permissions {
  fn check(
    &mut self,
    _path: Option<&Path>,
  ) -> Result<(), deno_core::error::AnyError> {
    unreachable!("snapshotting!")
  }
}

impl deno_node::NodePermissions for Permissions {
  fn check_net_url(
    &mut self,
    _url: &deno_core::url::Url,
    _api_name: &str,
  ) -> Result<(), deno_core::error::AnyError> {
    unreachable!("snapshotting!")
  }
  fn check_read_with_api_name(
    &self,
    _p: &Path,
    _api_name: Option<&str>,
  ) -> Result<(), deno_core::error::AnyError> {
    unreachable!("snapshotting!")
  }
  fn check_write_with_api_name(
    &self,
    _p: &Path,
    _api_name: Option<&str>,
  ) -> Result<(), deno_core::error::AnyError> {
    unreachable!("snapshotting!")
  }
  fn check_sys(
    &self,
    _kind: &str,
    _api_name: &str,
  ) -> Result<(), deno_core::error::AnyError> {
    unreachable!("snapshotting!")
  }
}

impl deno_net::NetPermissions for Permissions {
  fn check_net<T: AsRef<str>>(
    &mut self,
    _host: &(T, Option<u16>),
    _api_name: &str,
  ) -> Result<(), deno_core::error::AnyError> {
    unreachable!("snapshotting!")
  }

  fn check_read(
    &mut self,
    _p: &Path,
    _api_name: &str,
  ) -> Result<(), deno_core::error::AnyError> {
    unreachable!("snapshotting!")
  }

  fn check_write(
    &mut self,
    _p: &Path,
    _api_name: &str,
  ) -> Result<(), deno_core::error::AnyError> {
    unreachable!("snapshotting!")
  }
}

impl deno_fs::FsPermissions for Permissions {
  fn check_read(
    &mut self,
    _path: &Path,
    _api_name: &str,
  ) -> Result<(), AnyError> {
    unreachable!("snapshotting!")
  }

  fn check_read_all(&mut self, _api_name: &str) -> Result<(), AnyError> {
    unreachable!("snapshotting!")
  }

  fn check_read_blind(
    &mut self,
    _path: &Path,
    _display: &str,
    _api_name: &str,
  ) -> Result<(), AnyError> {
    unreachable!("snapshotting!")
  }

  fn check_write(
    &mut self,
    _path: &Path,
    _api_name: &str,
  ) -> Result<(), AnyError> {
    unreachable!("snapshotting!")
  }

  fn check_write_partial(
    &mut self,
    _path: &Path,
    _api_name: &str,
  ) -> Result<(), AnyError> {
    unreachable!("snapshotting!")
  }

  fn check_write_all(&mut self, _api_name: &str) -> Result<(), AnyError> {
    unreachable!("snapshotting!")
  }

  fn check_write_blind(
    &mut self,
    _path: &Path,
    _display: &str,
    _api_name: &str,
  ) -> Result<(), AnyError> {
    unreachable!("snapshotting!")
  }
}

impl deno_kv::sqlite::SqliteDbHandlerPermissions for Permissions {
  fn check_read(
    &mut self,
    _path: &Path,
    _api_name: &str,
  ) -> Result<(), AnyError> {
    unreachable!("snapshotting!")
  }

  fn check_write(
    &mut self,
    _path: &Path,
    _api_name: &str,
  ) -> Result<(), AnyError> {
    unreachable!("snapshotting!")
  }
}
