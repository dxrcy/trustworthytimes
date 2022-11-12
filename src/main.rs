use std::error::Error;

use serde_json::{json, Value};
use unreact::prelude::*;

use trustworthytimes::{get_articles, shuffle, URL};

const DO_MINIFY: bool = false;
const USE_TEST_ARTICLES: bool = false;

fn main() -> Result<(), Box<dyn Error>> {
  let articles = shuffle(get_articles(USE_TEST_ARTICLES)?);

  let mut app = Unreact::new(
    Config {
      minify: DO_MINIFY,
      ..Config::default()
    },
    is_dev(),
    URL,
  )?;

  // Include `articles` in every template
  app.set_globals(json!({ "articles": articles }));

  // Index and 404 pages
  app
    .index("pages/index", &Value::Null)?
    .not_found("pages/404", &Value::Null)?;

  // Article pages in `/news/*`
  for article in articles {
    app.page(
      &format!("news/{}", article.id),
      "pages/article",
      &json!(article),
    )?;
  }

  app.finish()?;
  println!("Successfully compiled.");

  Ok(())
}
