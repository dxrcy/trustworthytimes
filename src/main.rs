use std::error::Error;

use newsmarkdown::compile::{clean_build_dir, compile_articles, render_articles, render_index};

fn main() -> Result<(), Box<dyn Error>> {
  clean_build_dir();

  let articles = compile_articles()?;
  render_index(&articles)?;
  render_articles(&articles)?;

  Ok(())
}
