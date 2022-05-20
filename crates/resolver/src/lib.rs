use napi::bindgen_prelude::*;
use napi_derive::napi;
use nodejs_resolver::{ResolveResult, Resolver};
use std::path::Path;

#[napi]
pub fn resolve_file(filepath: String, importpath: String) -> Result<String> {
  let resolver = Resolver::default().with_extensions(vec!["less", "css", "scss", "sass", "js"]);
  match resolver.resolve(&Path::new(filepath.as_str()), importpath.as_str()) {
    Ok(res) => {
      if let ResolveResult::Path(abs_path) = res {
        Ok(abs_path.to_str().unwrap().to_string())
      } else {
        Ok(importpath)
      }
    }
    Err(msg) => Err(Error::new(Status::Unknown, msg.to_string())),
  }
}
