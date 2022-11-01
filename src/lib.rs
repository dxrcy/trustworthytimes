pub mod compile;
pub mod news;

use crate::compile::{compile_articles, create_index};
use std::error::Error;

/// Runs parse and compile functions
///TODO Rename everything!
pub fn run() -> Result<(), Box<dyn Error>> {
  let articles = compile_articles("./news", "./build/news")?;
  create_index("./build/index.html", "./news", &articles)?;

  Ok(())
}
