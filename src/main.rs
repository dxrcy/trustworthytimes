use std::error::Error;

use newsmarkdown::compile::{clean_build_dir, compile_articles, render_articles, render_root_files, compile_styles};

fn main() -> Result<(), Box<dyn Error>> {
  clean_build_dir()?;

  let articles = compile_articles()?;

  render_root_files(&articles)?;
  render_articles(&articles)?;

  compile_styles()?;

  Ok(())
}
