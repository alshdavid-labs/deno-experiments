mod permissions;

use std::path::PathBuf;
use std::sync::Arc;

use deno_core::futures::FutureExt;
use deno_core::unsync::MaskFutureAsSend;
use deno_core::url::Url;
use deno_core::ModuleCodeString;
use deno_runtime::fmt_errors::format_js_error;
use deno_runtime::permissions::PermissionsContainer;
use deno_runtime::worker::MainWorker;
use deno_runtime::worker::WorkerOptions;
use deno_runtime::BootstrapOptions;

const CODE: &str = r#"
  console.log(42)
"#;

pub const SNAPSHOT: &[u8] = include_bytes!("/home/dalsh/Development/deno-sandbox/deno1/pass-no-snapshot/snapshots/v8-deno-linux-amd64.bin");

fn main() {
    deno_current_thread(run_js());
}

pub async fn run_js() {
  // let exe_path = std::env::current_exe().unwrap().parent().unwrap().to_path_buf();
  // let main_module = Url::from_file_path(exe_path).unwrap();
  
  let main_module_path = PathBuf::from("/home/dalsh/Development/deno-sandbox/deno1/pass-no-snapshot/pkg/index.js");
  let main_module = Url::from_file_path(&main_module_path).unwrap();

  let fs = std::sync::Arc::new(deno_fs::RealFs);
  let permissions = PermissionsContainer::allow_all();

  let bootstrap_options = BootstrapOptions {
    has_node_modules_dir: true,
    ..Default::default()
  };

  let worker_options = WorkerOptions {
    bootstrap: bootstrap_options.clone(),
    startup_snapshot: Some(SNAPSHOT),
    fs: fs.clone(),
    create_web_worker_cb: Arc::new(|_| panic!()),
    format_js_error_fn: Some(Arc::new(format_js_error)),
    ..Default::default()
  };


  let mut main_worker = MainWorker::from_options(main_module.clone(), permissions, worker_options);

  main_worker.bootstrap(bootstrap_options.clone());

  main_worker.execute_main_module(&main_module).await.unwrap();
  // main_worker.execute_script("test.js", ModuleCodeString::from_static(CODE)).unwrap();
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
