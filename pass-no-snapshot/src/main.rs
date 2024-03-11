mod permissions;
mod worker;

use std::rc::Rc;
use std::sync::Arc;

use deno_cache::SqliteBackedCache;
use deno_core::futures::FutureExt;
use deno_core::unsync::MaskFutureAsSend;
use deno_core::url::Url;
use deno_core::FastString;
use deno_core::PollEventLoopOptions;
use deno_http::DefaultHttpPropertyExtractor;
use deno_runtime::fmt_errors::format_js_error;
use deno_runtime::permissions::PermissionsContainer;
use deno_runtime::worker::MainWorker;
use deno_runtime::worker::WorkerOptions;
use deno_runtime::BootstrapOptions;
use permissions::Permissions;
use deno_runtime::ops;
use deno_runtime::runtime;
use worker::create_web_worker_callback;
use worker::SharedWorkerState;

const CODE: &str = r#"
  console.log(42)
"#;

pub const SNAPSHOT: &[u8] = include_bytes!("./snapshot.bin");

fn main() {
    deno_current_thread(run_js());
}

pub async fn run_js() {
    let exe_path = std::env::current_exe().unwrap().parent().unwrap().to_path_buf();
    let main_module = Url::from_file_path(exe_path).unwrap();
    let fs = std::sync::Arc::new(deno_fs::RealFs);
    let permissions = PermissionsContainer::allow_all();

    let bootstrap_options = BootstrapOptions {
      ..Default::default()
    };

    let worker_options = WorkerOptions {
      bootstrap: bootstrap_options.clone(),
      startup_snapshot: Some(SNAPSHOT),
      fs: fs.clone(),
      ..Default::default()
    };

    let broadcast_channel = deno_broadcast_channel::InMemoryBroadcastChannel::default();

    let web_worker_callback = create_web_worker_callback(Arc::new(SharedWorkerState {
          // node_resolver: todo!(),
          blob_store: Default::default(),
          broadcast_channel: broadcast_channel.clone(),
          shared_array_buffer_store: Default::default(),
          compiled_wasm_module_store: Default::default(),
          fs: fs.clone(),
          argv: vec![],
          argv0: Some("".to_string()),
          skip_op_registration: false,
          maybe_root_package_json_deps: None,
      }),
      Default::default(),
    );

    let mut extensions = vec![
        deno_webidl::deno_webidl::init_ops_and_esm(),
        deno_console::deno_console::init_ops_and_esm(),
        deno_url::deno_url::init_ops_and_esm(),
        deno_web::deno_web::init_ops_and_esm::<Permissions>(
          Arc::new(deno_web::BlobStore::default()),
          None,
        ),
        deno_webgpu::deno_webgpu::init_ops_and_esm(),
        deno_canvas::deno_canvas::init_ops_and_esm(),
        deno_fetch::deno_fetch::init_ops_and_esm::<Permissions>(Default::default()),
        deno_cache::deno_cache::init_ops_and_esm::<SqliteBackedCache>(None),
        deno_websocket::deno_websocket::init_ops_and_esm::<Permissions>("".to_owned(), None, None),
        deno_webstorage::deno_webstorage::init_ops_and_esm(None),
        deno_crypto::deno_crypto::init_ops_and_esm(None),
        deno_broadcast_channel::deno_broadcast_channel::init_ops_and_esm(
          broadcast_channel,
        ),
        deno_ffi::deno_ffi::init_ops_and_esm::<Permissions>(),
        deno_net::deno_net::init_ops_and_esm::<Permissions>(None, None),
        deno_tls::deno_tls::init_ops_and_esm(),
        deno_kv::deno_kv::init_ops_and_esm(deno_kv::sqlite::SqliteDbHandler::<Permissions>::new(
            None, None,
        )),
        deno_cron::deno_cron::init_ops_and_esm(deno_cron::local::LocalCronHandler::new()),
        deno_napi::deno_napi::init_ops_and_esm::<Permissions>(),
        deno_http::deno_http::init_ops_and_esm::<DefaultHttpPropertyExtractor>(),
        deno_io::deno_io::init_ops_and_esm(Default::default()),
        deno_fs::deno_fs::init_ops_and_esm::<Permissions>(fs.clone()),
        deno_node::deno_node::init_ops_and_esm::<Permissions>(None, fs),
        ops::runtime::deno_runtime::init_ops(main_module.clone()),
        ops::worker_host::deno_worker_host::init_ops(
            web_worker_callback.clone(),
            Some(Arc::new(format_js_error)),
        ),
        ops::fs_events::deno_fs_events::init_ops(),
        ops::os::deno_os::init_ops(Default::default()),
        ops::permissions::deno_permissions::init_ops(),
        ops::process::deno_process::init_ops(),
        ops::signal::deno_signal::init_ops(),
        ops::tty::deno_tty::init_ops(),
        ops::http::deno_http_runtime::init_ops(),
        ops::bootstrap::deno_bootstrap::init_ops(None),
        // deno_permissions_worker::init_ops_and_esm(
        //   permissions,
        //   false,
        // ),
        runtime::init_ops_and_esm(),
        ops::web_worker::deno_web_worker::init_ops(),
    ];

    let main_worker = MainWorker::from_options(main_module, permissions, worker_options);
    for extension in &mut extensions {
      extension.js_files = std::borrow::Cow::Borrowed(&[]);
      extension.esm_files = std::borrow::Cow::Borrowed(&[]);
      extension.esm_entry_point = None;
    }


    let runtime_options = deno_core::RuntimeOptions {
        module_loader: Some(Rc::new(deno_core::FsModuleLoader)),
        is_main: true,
        startup_snapshot: Some(SNAPSHOT),
        extensions,
        ..Default::default()
    };

    let mut js_runtime = deno_core::JsRuntime::new(runtime_options);

    {
      let op_state = &mut js_runtime.op_state();
      let mut state = op_state.borrow_mut();
      state.put(bootstrap_options.clone());
      if let Some(node_ipc_fd) = bootstrap_options.node_ipc_fd {
        state.put(deno_node::ChildPipeFd(node_ipc_fd));
      }
    }


    let code: FastString = CODE.to_string().into();

    let mod_id = js_runtime
        .load_main_es_module_from_code(&main_module, code)
        .await
        .unwrap();

    let result = js_runtime.mod_evaluate(mod_id);

    js_runtime
        .run_event_loop(PollEventLoopOptions {
            wait_for_inspector: false,
            pump_v8_message_loop: false,
        })
        .await
        .unwrap();

    result.await.unwrap();
}

#[inline(always)]
fn deno_current_thread<F, R>(future: F) -> R
where
    F: std::future::Future<Output = R> + 'static,
    R: Send + 'static,
{
    let tokio_runtime = tokio::runtime::Builder::new_current_thread()
        .enable_io()
        .enable_time()
        .event_interval(61)
        .global_queue_interval(31)
        .max_io_events_per_tick(1024)
        .max_blocking_threads(32)
        .build()
        .unwrap();

    let future = async move {
        deno_core::unsync::spawn(async move { future.await }.boxed_local())
            .await
            .unwrap()
    };

    #[cfg(debug_assertions)]
    let future = Box::pin(unsafe { MaskFutureAsSend::new(future) });

    #[cfg(not(debug_assertions))]
    let future = unsafe { MaskFutureAsSend::new(future) };

    let join_handle = tokio_runtime.spawn(future);

    tokio_runtime.block_on(join_handle).unwrap().into_inner()
}
