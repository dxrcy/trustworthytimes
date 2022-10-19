use newsmarkdown::{compile_articles, create_index};

fn main() {
  let articles = compile_articles("./news", "./build/news");
  create_index(&articles);
}
