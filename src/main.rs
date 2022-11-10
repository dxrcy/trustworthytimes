use std::error::Error;

use serde_json::{json, Value};
use unreact::prelude::*;

use trustworthytimes::{get_articles, URL, shuffle};

fn main() -> Result<(), Box<dyn Error>> {
  let articles = shuffle(get_articles(false)?);

  let mut app = Unreact::new(
    Config::default(),
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
