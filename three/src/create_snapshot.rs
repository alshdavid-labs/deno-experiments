use std::io::Write;
use std::net::Ipv4Addr;
use std::net::SocketAddr;
use std::net::SocketAddrV4;
use std::path::PathBuf;
use std::rc::Rc;
use std::sync::Arc;
use deno_cache::SqliteBackedCache;
use deno_core::error::AnyError;
use deno_core::snapshot::create_snapshot as create_v8_snapshot;
use deno_core::snapshot::CreateSnapshotOptions;
use deno_core::Extension;
use deno_core::OpState;
use deno_core::ResourceId;
use deno_http::DefaultHttpPropertyExtractor;
use deno_net::raw::NetworkStream;
use deno_net::raw::NetworkStreamAddress;
use deno_webidl;
use deno_console;
use deno_url;
use deno_web;
use deno_webgpu;
use deno_canvas;
use deno_fetch;
use deno_cache;
use deno_websocket;
use deno_runtime;
use crate::runtime::maybe_transpile_source;
use crate::runtime::runtime;
use crate::permissions::Permissions;

pub fn create_snapshot() {
  let cargo_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
  let snapshot = cargo_dir.join("snapshot.bin");

  if snapshot.exists() {
    println!("deleting existing snapshot");
    std::fs::remove_file(&snapshot).unwrap();
  }

  println!("creating new snapshot");
  
  let fs = Arc::new(deno_fs::RealFs);

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
    deno_webstorage::deno_webstorage::init_ops_and_esm(None),
    deno_crypto::deno_crypto::init_ops_and_esm(None),
    deno_broadcast_channel::deno_broadcast_channel::init_ops_and_esm(
      deno_broadcast_channel::InMemoryBroadcastChannel::default(),
    ),
    deno_ffi::deno_ffi::init_ops_and_esm::<Permissions>(),
    deno_net::deno_net::init_ops_and_esm::<Permissions>(None, None),
    deno_tls::deno_tls::init_ops_and_esm(),
    // deno_kv::deno_kv::init_ops_and_esm(deno_kv::sqlite::SqliteDbHandler::<
    //   Permissions,
    // >::new(None, None)),
    deno_cron::deno_cron::init_ops_and_esm(
      deno_cron::local::LocalCronHandler::new(),
    ),
    deno_napi::deno_napi::init_ops_and_esm::<Permissions>(),
    deno_http::deno_http::init_ops_and_esm::<DefaultHttpPropertyExtractor>(),
    deno_io::deno_io::init_ops_and_esm(Default::default()),
    deno_fs::deno_fs::init_ops_and_esm::<Permissions>(fs.clone()),
    deno_node::deno_node::init_ops_and_esm::<Permissions>(None, fs.clone()),
    deno_runtime::runtime::init_ops_and_esm(),
    // runtime::runtime::init_ops_and_esm(),
    // ops::runtime::deno_runtime::init_ops("deno:runtime".parse().unwrap()),
    // ops::worker_host::deno_worker_host::init_ops(
    //   Arc::new(|_| unreachable!("not used in snapshot.")),
    //   None,
    // ),
    // ops::fs_events::deno_fs_events::init_ops(),
    // ops::os::deno_os::init_ops(Default::default()),
    // ops::permissions::deno_permissions::init_ops(),
    // ops::process::deno_process::init_ops(),
    // ops::signal::deno_signal::init_ops(),
    // ops::tty::deno_tty::init_ops(),
    // ops::http::deno_http_runtime::init_ops(),
    // ops::bootstrap::deno_bootstrap::init_ops(Some(snapshot_options)),
    // ops::web_worker::deno_web_worker::init_ops(),
  ];

  let output = create_v8_snapshot(CreateSnapshotOptions {
    cargo_manifest_dir: env!("CARGO_MANIFEST_DIR"),
    startup_snapshot: None,
    skip_op_registration: false,
    extensions,
    with_runtime_cb: None,
    extension_transpiler: Some(Rc::new(|specifier, source| {
      maybe_transpile_source(specifier, source)
    })),
  }, None).unwrap();

  let mut snapshot = std::fs::File::create(snapshot).unwrap();
  snapshot.write_all(&output.output).unwrap();
}
