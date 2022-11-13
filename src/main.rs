use std::error::Error;

use serde_json::{json, Value};
use unreact::prelude::*;

use trustworthytimes::{compile_categories, get_articles, shuffle, URL};

const DO_MINIFY: bool = false;
const IGNORE_TEST_ARTICLES: bool = true;

fn main() -> Result<(), Box<dyn Error>> {
  let articles = shuffle(get_articles(IGNORE_TEST_ARTICLES)?);
  let (_authors, _tags) = compile_categories(&articles);

  let mut app = Unreact::new(
    Config {
      minify: DO_MINIFY,
      ..Config::default()
    },
    is_dev(),
    URL,
  )?;

  // Include `articles` in every template
  app.set_globals(json!({
    "articles": articles,
    // Different shuffle for scroller banner
    "articles_scroller": shuffle(get_articles(IGNORE_TEST_ARTICLES)?),
    // ? Move to only `/authors` path ?
    // "authors": authors,
    // ? Move to only `/tags` path ?
    // "tags": tags,
  }));

  // Index and 404 pages
  app
    .index("pages/index", &Value::Null)?
    .not_found("pages/404", &Value::Null)?;
    // .page("authors", "pages/author_list", &Value::Null)?;

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
