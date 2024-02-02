pub mod news;

use std::{collections::HashMap, error::Error, fs};

use news::Article;
use rand::{seq::SliceRandom, thread_rng};
use unreact::is_dev;

pub const URL: &str = "https://dxrcy.dev/trustworthytimes";

/// Compile all articles from `/news` directory
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

/// Map a string (author, tag, ect) to vector of article references
type Dict<'a> = HashMap<&'a str, (u32, Vec<&'a Article>)>;
// type Dict<'a> = HashMap<&'a str, u32>;
// type DictVec<'a> = Vec<&'a (&'a str, (u32, Vec<&'a Article>))>;
// type DictVec<'a> = Vec<(&'a &'a str, &'a u32)>;

/// Compile authors and tags from articles
pub fn compile_categories(articles: &Vec<Article>) -> (Dict, Dict) {
  let mut authors = Dict::new();
  let mut tags = Dict::new();

  for article in articles {
    if let Some(author) = &article.author {
      let entry = authors.entry(author).or_insert((0, Vec::new()));
      entry.0 += 1;
      entry.1.push(article);
      // let mut entry = authors.entry(author).or_insert(0);
      // *entry += 1;
    }

    for tag in &article.tags {
      let entry = tags.entry(tag).or_insert((0, Vec::new()));
      entry.0 += 1;
      entry.1.push(article);
    }
  }

  // hashmap_to_vec(&authors)
  (authors, tags)
}

/// Convert hashmap to vector of tuple
// fn hashmap_to_vec<'a, K, V>(map: &'a HashMap<K, V>) -> Vec<(&'a K, &'a V)> {
//   // Vec::from_iter(map.iter())
//   let vec = Vec::new();

//   for (k, v) in map {
//     vec.push((k, v));
//   }

//   vec
// }

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
