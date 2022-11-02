use handlebars::Handlebars;
use serde_json::json;
use std::{error::Error, fs, path::Path};

use crate::news::Article;
use crate::{get_file_name, merge_json, DIR_BUILD, DIR_NEWS, IS_DEV, PARTIALS};

//TODO ERROR HANDLING!!!
//TODO Use enums or something

/// Create empty build directory if not exists
pub fn clean_build_dir() {
  if Path::new(DIR_BUILD).exists() {
    fs::remove_dir_all(DIR_BUILD).expect("Could not remove build directory");
  }
  fs::create_dir(DIR_BUILD).expect("Could not create build directory");
  fs::create_dir(format!("./{DIR_BUILD}/news")).expect("Could not create build news directory");
}

///TODO Comment here
pub fn compile_articles() -> Result<Vec<Article>, Box<dyn Error>> {
  let mut articles = Vec::<Article>::new();

  // Read input directory
  let files = fs::read_dir(DIR_NEWS).expect("Could not read input directory");
  // Loop through input directory files
  for file in files.flatten() {
    // Read input file
    let contents = fs::read_to_string(file.path()).expect("Could not read input file");
    // Get id (filename without extension) from file path
    let id = get_file_name(&file).expect("Could not read name of input file");
    // Compile file contents to article
    let article = Article::from(&id, &contents);

    // Push to articles vector
    articles.push(article);
  }

  Ok(articles)
}

///TODO Comment here
pub fn render_articles(articles: &Vec<Article>) -> Result<(), Box<dyn Error>> {
  for article in articles {
    // Render from template
    let render = render_template("article", json!(article))?;

    // Write file to corresponding build directory
    fs::write(
      format!("./{DIR_BUILD}/news/{id}.html", id = article.id),
      render,
    )
    .expect("Could not write article file");
  }

  Ok(())
}

///TODO Comment here
pub fn render_index(articles: &Vec<Article>) -> Result<(), Box<dyn Error>> {
  // Render from template
  let render = render_template("index", json!({ "articles": articles }))?;
  // Write to index file in build directory
  fs::write(format!("./{DIR_BUILD}/index.html"), render).expect("Could not create index file");

  Ok(())
}

/// Render template from file, return rendered string
fn render_template(name: &str, json: serde_json::Value) -> Result<String, Box<dyn Error>> {
  // Format with handlebars
  let mut reg = Handlebars::new();

  for (k, v) in PARTIALS.iter() {
    reg.register_partial(k, v)?;
  }

  // ? Convert to `.clone()` ?
  let mut json = json;
  merge_json(&mut json, json!({ "IS_DEV": *IS_DEV }));

  let render = reg.render_template(
    &fs::read_to_string(format!("./templates/{name}.hbs")).expect("Could not read template"),
    &json,
  )?;

  Ok(render)
}
