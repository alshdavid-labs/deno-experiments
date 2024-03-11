use std::sync::Arc;
use deno_runtime::web_worker as deno_web_worker;
use deno_runtime::ops::worker_host::CreateWebWorkerCb;
use deno_web_worker::WebWorkerOptions;
// Copyright 2018-2024 the Deno authors. All rights reserved. MIT license.

use std::path::Path;
use std::path::PathBuf;
use std::rc::Rc;

use deno_ast::ModuleSpecifier;
use deno_core::anyhow::bail;
use deno_core::anyhow::Context;
use deno_core::error::AnyError;
use deno_core::futures::FutureExt;
use deno_core::located_script_name;
use deno_core::parking_lot::Mutex;
use deno_core::url::Url;
use deno_core::v8;
use deno_core::CompiledWasmModuleStore;
use deno_core::Extension;
use deno_core::FeatureChecker;
use deno_core::ModuleId;
use deno_core::ModuleLoader;
use deno_core::PollEventLoopOptions;
use deno_core::SharedArrayBufferStore;
use deno_core::SourceMapGetter;
// use deno_lockfile::Lockfile;
use deno_runtime::deno_broadcast_channel::InMemoryBroadcastChannel;
use deno_runtime::deno_fs;
use deno_runtime::deno_node;
use deno_runtime::deno_node::NodeResolution;
use deno_runtime::deno_node::NodeResolutionMode;
use deno_runtime::deno_node::NodeResolver;
use deno_runtime::deno_tls::RootCertStoreProvider;
use deno_runtime::deno_web::BlobStore;
use deno_runtime::fmt_errors::format_js_error;
use deno_runtime::inspector_server::InspectorServer;
use deno_runtime::permissions::PermissionsContainer;
use deno_runtime::web_worker::WebWorker;
use deno_runtime::worker::MainWorker;
use deno_runtime::worker::WorkerOptions;
use deno_runtime::BootstrapOptions;
use deno_runtime::WorkerLogLevel;
// use deno_semver::npm::NpmPackageReqReference;
// use deno_semver::package::PackageReqReference;
use deno_terminal::colors;
use tokio::select;

pub trait ModuleLoaderFactory: Send + Sync {
  fn create_for_main(
    &self,
    root_permissions: PermissionsContainer,
    dynamic_permissions: PermissionsContainer,
  ) -> Rc<dyn ModuleLoader>;

  fn create_for_worker(
    &self,
    root_permissions: PermissionsContainer,
    dynamic_permissions: PermissionsContainer,
  ) -> Rc<dyn ModuleLoader>;

  fn create_source_map_getter(&self) -> Option<Rc<dyn SourceMapGetter>>;
}

// todo(dsherret): this is temporary and we should remove this
// once we no longer conditionally initialize the node runtime
pub trait HasNodeSpecifierChecker: Send + Sync {
  fn has_node_specifier(&self) -> bool;
}

#[async_trait::async_trait(?Send)]
pub trait HmrRunner: Send + Sync {
  async fn start(&mut self) -> Result<(), AnyError>;
  async fn stop(&mut self) -> Result<(), AnyError>;
  async fn run(&mut self) -> Result<(), AnyError>;
}

#[async_trait::async_trait(?Send)]
pub trait CoverageCollector: Send + Sync {
  async fn start_collecting(&mut self) -> Result<(), AnyError>;
  async fn stop_collecting(&mut self) -> Result<(), AnyError>;
}

pub type CreateHmrRunnerCb = Box<
  dyn Fn(deno_core::LocalInspectorSession) -> Box<dyn HmrRunner> + Send + Sync,
>;

pub type CreateCoverageCollectorCb = Box<
  dyn Fn(deno_core::LocalInspectorSession) -> Box<dyn CoverageCollector>
    + Send
    + Sync,
>;


pub struct CliMainWorkerOptions {
  pub argv: Vec<String>,
  // pub log_level: WorkerLogLevel,
  // pub coverage_dir: Option<String>,
  // pub enable_op_summary_metrics: bool,
  // pub enable_testing_features: bool,
  // pub has_node_modules_dir: bool,
  // pub hmr: bool,
  // pub inspect_brk: bool,
  // pub inspect_wait: bool,
  // pub strace_ops: Option<Vec<String>>,
  // pub is_inspecting: bool,
  // pub is_npm_main: bool,
  // pub location: Option<Url>,
  pub argv0: Option<String>,
  // pub origin_data_folder_path: Option<PathBuf>,
  // pub seed: Option<u64>,
  // pub unsafely_ignore_certificate_errors: Option<Vec<String>>,
  // pub unstable: bool,
  pub skip_op_registration: bool,
  pub maybe_root_package_json_deps: Option<()>,
  // pub create_hmr_runner: Option<CreateHmrRunnerCb>,
  // pub create_coverage_collector: Option<CreateCoverageCollectorCb>,
}

pub struct SharedWorkerState {
  // pub node_resolver: Arc<NodeResolver>,
  pub blob_store: Arc<BlobStore>,
  pub broadcast_channel: InMemoryBroadcastChannel,
  pub shared_array_buffer_store: SharedArrayBufferStore,
  pub compiled_wasm_module_store: CompiledWasmModuleStore,
  pub fs: Arc<dyn deno_fs::FileSystem>,
  pub argv: Vec<String>,
  pub argv0: Option<String>,
  pub skip_op_registration: bool,
  pub maybe_root_package_json_deps: Option<()>,

}

impl SharedWorkerState {
  // Currently empty
}

pub fn create_web_worker_callback(
  shared: Arc<SharedWorkerState>,
  stdio: deno_runtime::deno_io::Stdio,
) -> Arc<CreateWebWorkerCb> {
  Arc::new(move |args| {
    // let maybe_inspector_server = shared.maybe_inspector_server.clone();

    // let module_loader = shared.module_loader_factory.create_for_worker(
    //   args.parent_permissions.clone(),
    //   args.permissions.clone(),
    // );
    // let maybe_source_map_getter =
    //   shared.module_loader_factory.create_source_map_getter();
    let create_web_worker_cb =
      create_web_worker_callback(shared.clone(), stdio.clone());

    // let maybe_storage_key = shared
    //   .storage_key_resolver
    //   .resolve_storage_key(&args.main_module);
    // let cache_storage_dir = maybe_storage_key.map(|key| {
    //   // TODO(@satyarohith): storage quota management
    //   // Note: we currently use temp_dir() to avoid managing storage size.
    //   std::env::temp_dir()
    //     .join("deno_cache")
    //     .join(checksum::gen(&[key.as_bytes()]))
    // });

    // TODO(bartlomieju): this is cruft, update FeatureChecker to spit out
    // list of enabled features.
    // let feature_checker = shared.feature_checker.clone();
    // let mut unstable_features = vec![];
    //   Vec::with_capacity(crate::UNSTABLE_GRANULAR_FLAGS.len());
    // for (feature_name, _, id) in crate::UNSTABLE_GRANULAR_FLAGS {
    //   if feature_checker.check(feature_name) {
    //     unstable_features.push(*id);
    //   }
    // }

    let options = WebWorkerOptions {
      bootstrap: BootstrapOptions {
        args: shared.argv.clone(),
        cpu_count: std::thread::available_parallelism()
          .map(|p| p.get())
          .unwrap_or(1),
        log_level: WorkerLogLevel::default(),//shared.options.log_level,
        enable_op_summary_metrics: false,//shared.options.enable_op_summary_metrics,
        enable_testing_features: false,//shared.options.enable_testing_features,
        locale: deno_core::v8::icu::get_language_tag(),
        location: Some(args.main_module.clone()),
        no_color: !colors::use_color(),
        is_tty: deno_terminal::is_stdout_tty(),
        unstable:false, // shared.options.unstable,
        unstable_features: vec![],
        user_agent: "Mach/0.0.0".into(),//version::get_user_agent().to_string(),
        inspect: false,//shared.options.is_inspecting,
        has_node_modules_dir: true,//shared.options.has_node_modules_dir,
        argv0: shared.argv0.clone(),
        node_ipc_fd: None,
        disable_deprecated_api_warning: false, //shared.disable_deprecated_api_warning,
        verbose_deprecated_api_warning: false, //shared.verbose_deprecated_api_warning,
        future: false,
      },
      extensions: vec![],
      startup_snapshot: Some(crate::SNAPSHOT),
      unsafely_ignore_certificate_errors: None,// true, //
      root_cert_store_provider: None, //Some(shared.root_cert_store_provider.clone()),
      seed: None,//shared.options.seed,
      create_web_worker_cb,
      format_js_error_fn: Some(Arc::new(format_js_error)),
      source_map_getter: None,//maybe_source_map_getter,
      module_loader: Rc::new(deno_core::FsModuleLoader),
      fs: shared.fs.clone(),
      npm_resolver: None,//Some(shared.npm_resolver.clone().into_npm_resolver()),
      worker_type: args.worker_type,
      maybe_inspector_server: None,
      get_error_class_fn: None,//Some(&errors::get_error_class_name),
      blob_store: shared.blob_store.clone(),
      broadcast_channel: shared.broadcast_channel.clone(),
      shared_array_buffer_store: Some(shared.shared_array_buffer_store.clone()),
      compiled_wasm_module_store: Some(
        shared.compiled_wasm_module_store.clone(),
      ),
      stdio: stdio.clone(),
      cache_storage_dir: None,
      feature_checker: Arc::new(FeatureChecker::default()),
    };

    WebWorker::bootstrap_from_options(
      args.name,
      args.permissions,
      args.main_module,
      args.worker_id,
      options,
    )
  })
}
