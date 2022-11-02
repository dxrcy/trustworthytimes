pub mod compile;
pub mod news;

mod statics;
pub use statics::*;

use serde_json::Value;
use std::fs;

/// Convert `DirEntry` to string and get file name without extension
pub fn get_file_name(path: &fs::DirEntry) -> Option<String> {
  Some(
    path
      .path()
      .to_str()?
      .replace('\\', "/")
      .split('/')
      .last()?
      .split('.')
      .next()?
      .to_owned(),
  )
}

/// Merge one `serde_json` value with another
fn merge_json(a: &mut Value, b: Value) {
  if let Value::Object(a) = a {
    if let Value::Object(b) = b {
      for (k, v) in b {
        if v.is_null() {
          a.remove(&k);
        } else {
          merge_json(a.entry(k).or_insert(Value::Null), v);
        }
      }

      return;
    }
  }

  *a = b;
}

/// Escape html symbols
/// Does not include backtick!
pub fn escape_html(string: &str) -> String {
  handlebars::html_escape(string).replace("&#x60;", "`")
}
