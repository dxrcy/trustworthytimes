use handlebars::Handlebars;
use serde_json::json;

use crate::news::Article;
use std::{error::Error, fs, path::Path};

pub fn compile_articles(dir_in: &str, dir_out: &str) -> Result<Vec<Article>, Box<dyn Error>> {
  let mut articles = Vec::<Article>::new();

  // Create build directory
  if !Path::new(dir_out).exists() {
    fs::create_dir(dir_out).expect("Could not create output directory");
  }

  // Read input directory
  let files = fs::read_dir(dir_in).expect("Could not read input directory");

  // Loop through input directory files
  for file in files.flatten() {
    // Read input file
    let contents = fs::read_to_string(file.path()).expect("Could not read input file");
    // Get id (filename without extension) from file path
    let id = get_file_name(&file).expect("Could not read name of input file");
    // Compile file contents to article
    let article = Article::from(&id, &contents);

    // Format with handlebars
    let output = Handlebars::new()
      .render_template(&fs::read_to_string("./template/article.hbs")?, &article)?;

    // Write file to corresponding build directory
    create_build_dir();
    fs::write(format!("./build/news/{id}.html"), output)?;

    // Push to articles vector
    articles.push(article);
  }

  Ok(articles)
}

/// Create index file for build
pub fn create_index(
  _path: &str,
  _other: &str,
  articles: &Vec<Article>,
) -> Result<(), Box<dyn Error>> {
  let mut reg = Handlebars::new();
  reg.register_template_string("index", fs::read_to_string("./template/index.hbs")?)?;

  // Format with handlebars
  let output = Handlebars::new().render_template(
    &fs::read_to_string("./template/index.hbs")?,
    &json!({ "articles": articles }),
  )?;

  // Write to index file in build directory
  create_build_dir();
  fs::write("./build/index.html", output)?;

  Ok(())
}

/// Convert DirEntry to string and get file name
fn get_file_name(path: &fs::DirEntry) -> Option<String> {
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

/// Create build directory if not exists
//TODO Change to custom build dir name
fn create_build_dir() {
  if !Path::new("./build").exists() {
    fs::create_dir("./build").expect("Could not create output directory");
  }
}
