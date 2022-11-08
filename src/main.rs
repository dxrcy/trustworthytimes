use std::error::Error;

use serde_json::{json, Value};
use unreact::prelude::*;

use trustworthytimes::{get_articles, URL};

fn main() -> Result<(), Box<dyn Error>> {
  let articles = get_articles(false)?;

  let mut app = Unreact::new(
    Config {
      minify: false,
      ..Config::default()
    },
    is_dev(),
    URL,
  )?;

  app
    .index("pages/index", &json!({ "articles": articles }))?
    .not_found("pages/404", &Value::Null)?;

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
