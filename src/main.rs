use std::error::Error;

use serde_json::{json, Value};
use unreact::prelude::*;

use trustworthytimes::get_articles;

const URL: &str = "https://trustworthytimes.github.io";

fn main() -> Result<(), Box<dyn Error>> {
  let articles = get_articles()?;

  let mut app = Unreact::new(Config::github_pages(), is_dev(), URL)?;

  app
    .index("index", &json!({ "articles": articles }))?
    .not_found("404", &Value::Null)?;

  for article in articles {
    app.page(&format!("news/{}", article.id), "article", &json!(article))?;
  }

  app.finish()?;

  Ok(())
}
