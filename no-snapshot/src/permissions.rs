use std::path::Path;

use deno_core::error::AnyError;

#[derive(Clone)]
pub struct Permissions;

impl deno_fetch::FetchPermissions for Permissions {
  fn check_net_url(
    &mut self,
    _url: &deno_core::url::Url,
    _api_name: &str,
  ) -> Result<(), deno_core::error::AnyError> {
    Ok(())
  }

  fn check_read(
    &mut self,
    _p: &Path,
    _api_name: &str,
  ) -> Result<(), deno_core::error::AnyError> {
    Ok(())
  }
}

impl deno_websocket::WebSocketPermissions for Permissions {
  fn check_net_url(
    &mut self,
    _url: &deno_core::url::Url,
    _api_name: &str,
  ) -> Result<(), deno_core::error::AnyError> {
    Ok(())
  }
}

impl deno_web::TimersPermission for Permissions {
  fn allow_hrtime(&mut self) -> bool {
    true
  }
}

impl deno_ffi::FfiPermissions for Permissions {
  fn check_partial(
    &mut self,
    _path: Option<&Path>,
  ) -> Result<(), deno_core::error::AnyError> {
    Ok(())
  }
}

impl deno_napi::NapiPermissions for Permissions {
  fn check(
    &mut self,
    _path: Option<&Path>,
  ) -> Result<(), deno_core::error::AnyError> {
    Ok(())
  }
}

impl deno_node::NodePermissions for Permissions {
  fn check_net_url(
    &mut self,
    _url: &deno_core::url::Url,
    _api_name: &str,
  ) -> Result<(), deno_core::error::AnyError> {
    Ok(())
  }
  fn check_read_with_api_name(
    &self,
    _p: &Path,
    _api_name: Option<&str>,
  ) -> Result<(), deno_core::error::AnyError> {
    Ok(())
  }
  fn check_write_with_api_name(
    &self,
    _p: &Path,
    _api_name: Option<&str>,
  ) -> Result<(), deno_core::error::AnyError> {
    Ok(())
  }
  fn check_sys(
    &self,
    _kind: &str,
    _api_name: &str,
  ) -> Result<(), deno_core::error::AnyError> {
    Ok(())
  }
}

impl deno_net::NetPermissions for Permissions {
  fn check_net<T: AsRef<str>>(
    &mut self,
    _host: &(T, Option<u16>),
    _api_name: &str,
  ) -> Result<(), deno_core::error::AnyError> {
    Ok(())
  }

  fn check_read(
    &mut self,
    _p: &Path,
    _api_name: &str,
  ) -> Result<(), deno_core::error::AnyError> {
    Ok(())
  }

  fn check_write(
    &mut self,
    _p: &Path,
    _api_name: &str,
  ) -> Result<(), deno_core::error::AnyError> {
    Ok(())
  }
}

impl deno_fs::FsPermissions for Permissions {
  fn check_read(
    &mut self,
    _path: &Path,
    _api_name: &str,
  ) -> Result<(), AnyError> {
    Ok(())
  }

  fn check_read_all(&mut self, _api_name: &str) -> Result<(), AnyError> {
    Ok(())
  }

  fn check_read_blind(
    &mut self,
    _path: &Path,
    _display: &str,
    _api_name: &str,
  ) -> Result<(), AnyError> {
    Ok(())
  }

  fn check_write(
    &mut self,
    _path: &Path,
    _api_name: &str,
  ) -> Result<(), AnyError> {
    Ok(())
  }

  fn check_write_partial(
    &mut self,
    _path: &Path,
    _api_name: &str,
  ) -> Result<(), AnyError> {
    Ok(())
  }

  fn check_write_all(&mut self, _api_name: &str) -> Result<(), AnyError> {
    Ok(())
  }

  fn check_write_blind(
    &mut self,
    _path: &Path,
    _display: &str,
    _api_name: &str,
  ) -> Result<(), AnyError> {
    Ok(())
  }
}

impl deno_kv::sqlite::SqliteDbHandlerPermissions for Permissions {
  fn check_read(
    &mut self,
    _path: &Path,
    _api_name: &str,
  ) -> Result<(), AnyError> {
    Ok(())
  }

  fn check_write(
    &mut self,
    _path: &Path,
    _api_name: &str,
  ) -> Result<(), AnyError> {
    Ok(())
  }
}
