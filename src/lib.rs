pub mod compile;
pub mod news;

use compile::get_partials;

use crate::compile::{compile_articles, create_index};
use std::error::Error;

/// Runs parse and compile functions
///TODO Rename everything!
pub fn run() -> Result<(), Box<dyn Error>> {
  let partials = get_partials()?;

  let articles = compile_articles(&partials, "./news", "./build/news")?;
  create_index(&partials, "./build/index.html", "./news", &articles)?;

  Ok(())
}
