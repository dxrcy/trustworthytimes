use handlebars::Handlebars;
use serde_json::json;
use std::{error::Error, fs, path::Path};

use crate::news::Article;
use crate::{get_file_name, merge_json, DIR_BUILD, DIR_NEWS, IS_DEV, PARTIALS};

//TODO ERROR HANDLING!!!
//TODO Use enums or something

/// Create empty build directory if not exists
///TODO Error handling
pub fn clean_build_dir() -> Result<(), Box<dyn Error>> {
  // Delete old folder if exists
  if Path::new(DIR_BUILD).exists() {
    fs::remove_dir_all(DIR_BUILD).expect("Could not remove build directory");
  }
  // Create root build folder
  fs::create_dir(DIR_BUILD).expect("Could not create build directory");
  // Create subfolders
  // ? Convert to loop ?
  fs::create_dir(format!("./{DIR_BUILD}/news")).expect("Could not create build news directory");
  fs::create_dir(format!("./{DIR_BUILD}/css")).expect("Could not create build news directory");
  Ok(())
}

/// Compile all articles in a directory to a Vector
///TODO Error handling
pub fn compile_articles() -> Result<Vec<Article>, Box<dyn Error>> {
  Ok(
    // Loop through input directory files
    fs::read_dir(DIR_NEWS)
      .expect("Could not read input directory")
      .flatten()
      .map(|file| {
        // Compile file contents to article and push to vector
        Article::from(
          // Get id (filename without extension) from file path
          &get_file_name(&file).expect("Could not read name of input file"),
          // Read input file
          &fs::read_to_string(file.path()).expect("Could not read input file"),
        )
      })
      .collect(),
  )
}

/// Fill `article.hbs` template for each article, write html file in news folder in build directory
///TODO Error handling
pub fn render_articles(articles: &Vec<Article>) -> Result<(), Box<dyn Error>> {
  for article in articles {
    // Write file to corresponding file in news folder in build directory
    fs::write(
      format!("./{DIR_BUILD}/news/{id}.html", id = article.id),
      // Render from template
      render_template("article", json!(article))?,
    )
    .expect("Could not write article file");
  }

  Ok(())
}

/// Fill `index.hbs` template with all articles, write html file in build directory
///TODO Error handling
pub fn render_index(articles: &Vec<Article>) -> Result<(), Box<dyn Error>> {
  // Write to index file in build directory
  fs::write(
    format!("./{DIR_BUILD}/index.html"),
    // Render from template
    render_template("index", json!({ "articles": articles }))?,
  )
  .expect("Could not create index file");

  Ok(())
}

/// Render template from file, return rendered string
///TODO Error handling
fn render_template(name: &str, json: serde_json::Value) -> Result<String, Box<dyn Error>> {
  let mut reg = Handlebars::new();

  // Register all partials from static hashmap
  for (k, v) in PARTIALS.iter() {
    reg.register_partial(k, v)?;
  }

  // Merge json with custom templates
  // ? Convert to `.clone()` ?
  //TODO Move second json object to static variable
  let mut json = json;
  merge_json(&mut json, json!({ "IS_DEV": *IS_DEV }));

  // Render template as string
  Ok(reg.render_template(
    &fs::read_to_string(format!("./templates/{name}.hbs")).expect("Could not read template"),
    &json,
  )?)
}

/// Compile styles from `scss` to `css`
pub fn compile_styles() -> Result<(), Box<dyn Error>> {
  //TODO Move path to const
  let css = grass::from_path("./styles/global.scss", &grass::Options::default())?;
  fs::write(format!("./{DIR_BUILD}/css/global.css"), css)?;

  Ok(())
}
