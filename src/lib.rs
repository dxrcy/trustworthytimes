pub mod news;

use std::{error::Error, fs};

use news::Article;
use rand::{seq::SliceRandom, thread_rng};
use unreact::is_dev;

pub const URL: &str = "https://trustworthytimes.github.io";

pub fn get_articles(ignore_test_files: bool) -> Result<Vec<Article>, Box<dyn Error>> {
  Ok(
    // Loop through input directory files
    fs::read_dir("./news")
      .expect("Could not read input directory")
      .flatten()
      // Filter - ignore '.test.news' files in production
      .filter(|file| {
        (is_dev() && !ignore_test_files) || !file.path().to_str().unwrap_or("").contains(".test.")
      })
      // Map to articles
      .map(|file| {
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

/// Convert `DirEntry` to string and get file name without extension
pub fn get_file_name(path: &fs::DirEntry) -> Option<String> {
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

/// Escape html symbols
/// Does not include backtick!
pub fn escape_html(string: &str) -> String {
  handlebars::html_escape(string).replace("&#x60;", "`")
}

/// Shuffle a vector
pub fn shuffle<T>(vec: Vec<T>) -> Vec<T> {
  let mut vec = vec;
  let mut rng = thread_rng();
  vec.shuffle(&mut rng);
  vec
}
