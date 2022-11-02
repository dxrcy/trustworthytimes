use handlebars::Handlebars;
use serde_json::json;

use crate::news::Article;
use std::{collections::HashMap, error::Error, fs, path::Path};

//TODO ERROR HANDLING!!!

pub const BASE_URL: &str = "https://darccyy.github.io/news/";
pub const DIR_BUILD: &str = "./docs/";

/// Create output directory
/// Create build directory if not exists
pub fn create_build_dir() {
  if Path::new(DIR_BUILD).exists() {
    fs::remove_dir_all(DIR_BUILD).expect("Could not remove build directory");
  }
  fs::create_dir(DIR_BUILD).expect("Could not create build directory");
  fs::create_dir(format!("{DIR_BUILD}news")).expect("Could not create build news directory");
}

pub fn compile_articles(
  partials: &HashMap<String, String>,
  dir_in: &str,
) -> Result<Vec<Article>, Box<dyn Error>> {
  let mut articles = Vec::<Article>::new();

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
    let output = reg.render_template(
      &fs::read_to_string("./templates/article.hbs").expect("Could not read article template"),
      &article,
    )?;

    // Write file to corresponding build directory
    fs::write(format!("{DIR_BUILD}news/{id}.html"), output).expect("Could not write article file");

    // Push to articles vector
    articles.push(article);
  }

  Ok(articles)
}

/// Create index file for build
///TODO Create custom file names blah blah
pub fn create_index(
  partials: &HashMap<String, String>,
  _path: &str,
  articles: &Vec<Article>,
) -> Result<(), Box<dyn Error>> {
  // Format with handlebars
  let mut reg = Handlebars::new();
  for (k, v) in partials {
    reg.register_partial(k, v)?;
  }
  let output = reg.render_template(
    &fs::read_to_string("./templates/index.hbs").expect("Could not read template"),
    &json!({ "articles": articles }),
  )?;

  // Write to index file in build directory
  fs::write(format!("{DIR_BUILD}index.html"), output).expect("Could not create index file");

  Ok(())
}

/// Read all templates and partials from template directory
pub fn get_partials() -> Result<HashMap<String, String>, Box<dyn Error>> {
  let mut partials = HashMap::<String, String>::new();

  // Add default partials
  //TODO move to const variable
  partials.insert("url".to_string(), BASE_URL.to_string());

  // Read template directory
  let files = fs::read_dir("./templates/partials").expect("Could not read partials directory");

  for file in files.flatten() {
    if let Some(name) = get_file_name(&file) {
      partials.insert(
        name,
        fs::read_to_string(file.path()).expect("Could not read partial"),
      );
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
