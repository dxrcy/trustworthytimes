pub mod news;
use news::Article;
use std::{fs, path::Path};

// Read files from input directory
// Save articles to `articles` vector
// Write html files to output directory
pub fn compile_articles(dir_in: &str, dir_out: &str) -> Vec<Article> {
  let mut articles = Vec::<Article>::new();

  // Create build directory
  if !Path::new(dir_out).exists() {
    fs::create_dir(dir_out).expect("Could not create output directory");
  }

  // Read input directory
  let files = fs::read_dir(dir_in).expect("Could not read input directory");

  // Loop through input directory files
  for file in files {
    if let Ok(file) = file {
      // Read input file
      let contents = fs::read_to_string(file.path()).expect("Could not read input file");
      // Get id (filename without extension) from file path
      let id = get_file_name(&file).expect("Could not read name of input file");
      // Compile file contents to article
      let article = Article::from(&id, &contents);

      // Write file to corresponding output directory
      fs::write(format!("{dir_out}/{id}.html",), &article.body)
        .expect("Could not write to build file");

      // Push to articles vector
      articles.push(article);
    }
  }

  articles
}

/// Convert DirEntry to string and get file name
fn get_file_name(path: &fs::DirEntry) -> Option<String> {
  Some(
    path
      .path()
      .to_str()?
      .replace("\\", "/")
      .split("/")
      .last()?
      .split(".")
      .nth(0)?
      .to_owned(),
  )
}

/// Create index file for build
pub fn create_index(articles: &Vec<Article>) {
  let mut index = Vec::<String>::new();

  for article in articles {
    let article = (*article).clone(); // ? Remove clone ? how ?

    // Article values
    let id = article.id;
    let headline = article.headline.unwrap_or("[headline]".to_string());
    let title = article.title.unwrap_or("[title]".to_string());

    //TODO Remove .html in production
    index.push(format!(
      r#"<a href="./news/{id}.html"> {headline} <br/> {title} </a> <hr>"#,
    ));
  }

  fs::write("./build/index.html", index.join("\n\n<br /><hr>\n\n"))
    .expect("Could not write to index file");
}
