pub mod compile;
pub mod news;

use compile::{get_partials, create_build_dir};

use crate::compile::{compile_articles, create_index};
use std::error::Error;

/// Runs parse and compile functions
///TODO Rename everything!
pub fn run() -> Result<(), Box<dyn Error>> {
  create_build_dir();

  let partials = get_partials()?;

  let articles = compile_articles(&partials, "./news")?;
  create_index(&partials, "index.html", &articles)?;

  Ok(())
}
