use std::fs;

fn main() {
  let file = fs::read_to_string("./example.news").expect("Could not read file");
  let output = newsmarkdown::format(&file);
  fs::write("./example.html", output).expect("Could not write file");
}
