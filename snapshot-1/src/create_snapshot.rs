use std::io::Write;
use std::path::PathBuf;
use deno_cache::SqliteBackedCache;
use deno_core::snapshot::create_snapshot as create_v8_snapshot;
use deno_core::snapshot::CreateSnapshotOptions;
use deno_core::Extension;
use deno_webidl;
use deno_console;
use deno_url;
use deno_web;
use deno_webgpu;
use deno_canvas;
use deno_fetch;
use deno_cache;
use deno_websocket;

use crate::permissions::Permissions;

pub fn create_snapshot() {
  let cargo_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
  let snapshot = cargo_dir.join("snapshot.bin");

  if snapshot.exists() {
    println!("deleting existing snapshot");
    std::fs::remove_file(&snapshot).unwrap();
  }

  println!("creating new snapshot");

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
