mod permissions;

use std::rc::Rc;

use deno_cache::SqliteBackedCache;
use deno_core::futures::FutureExt;
use deno_core::unsync::MaskFutureAsSend;
use deno_core::PollEventLoopOptions;
use deno_core::url::Url;
use deno_core::FastString;
use permissions::Permissions;

const CODE: &str = r#"
  console.log(42)
"#;

fn main() {
  deno_current_thread(run_js());
}

pub async fn run_js() {
  let extensions = vec![];

  let runtime_options = deno_core::RuntimeOptions {
    module_loader: Some(Rc::new(deno_core::FsModuleLoader)),
    is_main: true,
    skip_op_registration: false,
    extensions,
    ..Default::default()
  };

  let mut js_runtime = deno_core::JsRuntime::new(runtime_options);

  let exe_path = std::env::current_exe().unwrap().parent().unwrap().to_path_buf();
  let main_module = Url::from_file_path(exe_path).unwrap();
  
  let code: FastString = CODE.to_string().into(); 

  let mod_id = js_runtime.load_main_es_module_from_code(&main_module, code)
    .await
    .unwrap();

  let result = js_runtime.mod_evaluate(mod_id);

  js_runtime
    .run_event_loop(PollEventLoopOptions {
      wait_for_inspector: false,
      pump_v8_message_loop: false,
    })
    .await.unwrap();

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