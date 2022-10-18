use regex::Regex;

/// Format news string to html
pub fn format(input: &str) -> String {
  // ! does not distinguish line breaks for spaces
  // Split at space and linebreak for unfiltered tokens
  let tokens_unf = input
    .split(&[' ', '\n'])
    .collect::<Vec<_>>()
    .iter()
    .map(|s| s.trim())
    .collect::<Vec<_>>();

  // Filter tokens
  let mut tokens = Vec::<String>::new();
  let mut token_acc = Vec::<&str>::new();
  for token in tokens_unf {
    if Regex::new(&format!(
      "^{}$",
      [
        r"#+",    // Header
        r"-",     // Unordered list
        r"\d+\.", // Ordered list
      ]
      .join("$|^")
    ))
    .unwrap()
    .is_match(token)
    {
      tokens.push(token_acc.join(" "));
      token_acc = Vec::<&str>::new();
      tokens.push(token.to_string());
    } else {
      token_acc.push(token);
    }
  }
  tokens.push(token_acc.join(" "));

  println!("{tokens:?}");

  "".into()
}

// struct Config {
//   headline: Option<String>,
//   subtitle: Option<String>,
//   author: Option<String>,
//   date: Option<String>,
//   topic: Option<Vec<String>>, // ? Convert to [String; 3] ?
//   image: Option<String>,
//   alt: Option<String>,
//   tags: Option<Vec<String>>,
//   body: Option<String>,
// }

// impl Config {
//   fn new() -> Self {
//     Config {
//       headline: None,
//       subtitle: None,
//       author: None,
//       date: None,
//       topic: None,
//       image: None,
//       alt: None,
//       tags: None,
//       body: None,
//     }
//   }
// }
