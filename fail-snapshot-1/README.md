Fails with

```
cargo run
```

```
    Finished dev [unoptimized + debuginfo] target(s) in 0.13s
     Running `target/debug/deno1`
creating new snapshot
Creating a snapshot...
Error: illegal value for flag --lazy=false of type bool
To set or unset a boolean flag, use --flag or --no-flag.
The remaining arguments were ignored: --lazy-eval=false --lazy-streaming=false
Try --help for options
WARNING: v8::OwnedIsolate for snapshot was leaked
thread 'main' panicked at /home/dalsh/.local/rust/cargo/registry/src/index.crates.io-6f17d22bba15001f/deno_core-0.269.0/runtime/jsruntime.rs:1854:9:
Failed to initialize JsRuntime for snapshotting: Following modules were not evaluated; make sure they are imported from other code:
   - ext:deno_web/15_performance.js
  - ext:deno_webgpu/02_surface.js
  - ext:deno_web/05_base64.js
  - ext:deno_web/01_mimesniff.js
  - ext:deno_web/02_timers.js
  - ext:deno_fetch/20_headers.js
  - ext:deno_fetch/27_eventsource.js
  - ext:deno_web/04_global_interfaces.js
  - ext:deno_web/16_image_data.js
  - ext:deno_cache/01_cache.js
  - ext:deno_websocket/02_websocketstream.js
  - ext:deno_web/10_filereader.js
  - ext:deno_web/03_abort_signal.js
  - ext:deno_web/09_file.js
  - ext:deno_web/06_streams.js
  - ext:deno_web/12_location.js
  - ext:deno_url/00_url.js
  - ext:deno_fetch/22_body.js
  - ext:deno_web/01_dom_exception.js
  - ext:deno_fetch/22_http_client.js
  - ext:deno_webidl/00_webidl.js
  - ext:deno_web/02_structured_clone.js
  - ext:deno_websocket/01_websocket.js
  - ext:deno_fetch/21_formdata.js
  - ext:deno_fetch/23_response.js
  - ext:deno_fetch/23_request.js
  - ext:deno_url/01_urlpattern.js
  - ext:deno_fetch/26_fetch.js
  - ext:deno_web/00_infra.js
  - ext:deno_web/02_event.js
  - ext:deno_web/08_text_encoding.js
  - ext:deno_web/14_compression.js
  - ext:deno_webgpu/00_init.js
  - ext:deno_web/13_message_port.js
  - ext:deno_console/01_console.js

note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```