use handlebars::Handlebars;
use serde_json::json;

use crate::news::Article;
use std::{collections::HashMap, error::Error, fs, path::Path};

pub fn compile_articles(
  partials: &HashMap<String, String>,
  dir_in: &str,
  dir_out: &str,
) -> Result<Vec<Article>, Box<dyn Error>> {
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
    let mut reg = Handlebars::new();
    for (k, v) in partials {
      reg.register_partial(k, v)?;
    }
    let output = reg.render_template(&fs::read_to_string("./templates/article.hbs")?, &article)?;

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
  partials: &HashMap<String, String>,
  _path: &str,
  _other: &str,
  articles: &Vec<Article>,
) -> Result<(), Box<dyn Error>> {
  let mut reg = Handlebars::new();
  reg.register_template_string("index", fs::read_to_string("./templates/index.hbs")?)?;

  // Format with handlebars
  let mut reg = Handlebars::new();
  for (k, v) in partials {
    reg.register_partial(k, v)?;
  }
  let output = reg.render_template(
    &fs::read_to_string("./templates/index.hbs")?,
    &json!({ "articles": articles }),
  )?;

  // Write to index file in build directory
  create_build_dir();
  fs::write("./build/index.html", output)?;

  Ok(())
}

/// Read all templates and partials from template directory
pub fn get_partials() -> Result<HashMap<String, String>, Box<dyn Error>> {
  let mut partials = HashMap::<String, String>::new();

  // Add default partials
  //TODO move to const variable
  partials.insert(
    "url".to_string(),
    "https://github.com/darccyy/news".to_string(),
  );

  // Read template directory
  let files = fs::read_dir("./templates/partials")?;

  for file in files.flatten() {
    if let Some(name) = get_file_name(&file) {
      partials.insert(name, fs::read_to_string(file.path())?);
    }
  }

  Ok(partials)
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
