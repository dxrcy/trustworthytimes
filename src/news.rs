use regex::Regex;
use serde::Serialize;
use std::collections::HashMap;
use unreact::is_dev;

use crate::{escape_html, URL};

/// Article metadata and body, with id
#[derive(Debug, Clone, Serialize)]
pub struct Article {
  pub id: String,
  pub headline: Option<String>,
  pub desc: Option<String>,
  pub author: Option<String>,
  pub date: Option<String>,
  pub topic: Option<Vec<String>>, // ? Change to array ?
  pub image: Option<String>,
  pub alt: Option<String>,
  pub tags: Vec<String>,
  pub body: String,
}

impl Article {
  /// Get key of hashmap as Option<String>
  fn hashmap_key(hm: &HashMap<String, String>, key: &str) -> Option<String> {
    hm.get(key).map(|s| s.to_owned())
  }

  /// Build meta struct from hashmap
  fn build(id: &str, body: &str, meta: &HashMap<String, String>) -> Self {
    // Optional vector
    let topic = Self::hashmap_key(meta, "topic").map(|s| {
      s.split('|')
        .collect::<Vec<_>>()
        .iter()
        .map(|x| x.trim().to_string())
        .collect()
    });

    // Format image path
    let image = {
      // Case to use local file
      let image = Self::hashmap_key(meta, "image");
      if let Some(name) = &image {
        if name == "@" {
          // Use image from id
          Some(format!("{url}/public/thumb/{id}.jpg", url = real_url()))
        } else if name.starts_with('@') {
          // Use custom path relative to url
          // Remove first character
          let mut chars = name.chars();
          chars.next();
          Some(real_url() + chars.as_str())
        } else {
          // Full url
          image
        }
      } else {
        None
      }
    };

    // Vector
    let tags = match Self::hashmap_key(meta, "tags") {
      Some(s) => s
        .split('|')
        .collect::<Vec<_>>()
        .iter()
        .map(|x| x.trim().to_string())
        .collect(),
      None => vec![],
    };

    Article {
      id: id.to_string(),
      headline: Self::hashmap_key(meta, "headline"),
      desc: Self::hashmap_key(meta, "desc"),
      author: Self::hashmap_key(meta, "author"),
      date: Self::hashmap_key(meta, "date"),
      topic,
      image,
      alt: Self::hashmap_key(meta, "alt"),
      tags,
      body: body.to_string(),
    }
  }

  /// Format news string to html
  //TODO Add custom style support
  pub fn from(id: &str, input: &str) -> Article {
    // Parse news to raw body and meta
    let (mut body, meta) = parse_news(input);

    // Filter body
    body = include_meta(&body, &meta);
    body = format_primatives(&body);
    body = init_links(&body);

    Article::build(id, &body, &meta)
  }
}

/// Replace metadata key names with value in body
fn include_meta(body: &str, meta: &HashMap<String, String>) -> String {
  let mut body = body.to_string();
  for (key, value) in meta {
    body = body.replace(&format!("@{key}"), value);
  }
  body
}

/// Use `URL`, or dev url if `is_dev`
fn real_url() -> String {
  if is_dev() {
    format!("http://{}", unreact::dev::ADDRESS)
  } else {
    URL.to_string()
  }
}

/// Replace `@` at start of link with url
fn replace_with_url(link: &str) -> String {
  if link.starts_with('@') {
    // Remove first char
    let mut chars = link.chars();
    chars.next();

    real_url() + chars.as_str()
  } else {
    link.to_string()
  }
}

/// Create and format links of body
#[allow(clippy::format_push_string)]
fn init_links(body: &str) -> String {
  let mut output = String::new();
  // Current building link
  let mut link = None::<String>;

  for ch in body.chars() {
    // If link active
    if let Some(content) = &mut link {
      // Close link
      if ch == ']' {
        let split = content.split('|').collect::<Vec<&str>>();
        if let Some(desc) = split.first() {
          let href = replace_with_url(split.get(1).unwrap_or(&"#").trim());
          output += &format!("<a href={href}> {desc} </a>");
        }
        link = None;
      } else {
        content.push(ch);
      }
    } else {
      // Open link
      if ch == '[' {
        link = Some(String::new());
      } else {
        output.push(ch);
      }
    }
  }

  output
}

/// Format basic inline html styles of body
fn format_primatives(body: &str) -> String {
  #[derive(Default)]
  struct Primatives {
    italic: bool,
    bold: bool,
    underline: bool,
    strike: bool,
    code: bool,
    purple: bool,
  }
  let mut prims = Primatives::default();

  let mut output = String::new();
  let mut is_escaped = false;

  for ch in body.chars() {
    if is_escaped {
      output.push(ch);
    } else {
      match ch {
        // Non-escaped slash
        '\\' => (),

        // Italic
        '*' => {
          output.push_str(if prims.italic { "</i>" } else { "<i>" });
          prims.italic = !prims.italic;
        }

        // Bold
        '^' => {
          output.push_str(if prims.bold { "</b>" } else { "<b>" });
          prims.bold = !prims.bold;
        }

        // Underline
        '_' => {
          output.push_str(if prims.underline { "</u>" } else { "<u>" });
          prims.underline = !prims.underline;
        }

        // Strike
        '~' => {
          output.push_str(if prims.strike {
            "</strike>"
          } else {
            "<strike>"
          });
          prims.strike = !prims.strike;
        }

        // Code
        //TODO Fix??? its not working???
        '`' => {
          output.push_str(if prims.code { "</code>" } else { "<code>" });
          prims.code = !prims.code;
        }

        // Underline
        '%' => {
          output.push_str(if prims.purple {
            "</span>"
          } else {
            r#"<span class="purple">"#
          });
          prims.purple = !prims.purple;
        }

        // Other
        _ => {
          output.push(ch);
        }
      }
    }

    // Escape next character
    if ch == '\\' && !is_escaped {
      is_escaped = true;
    } else {
      is_escaped = false;
    }
  }

  output
}

/// Separate body and parse metadata of input string, add line styles
fn parse_news(input: &str) -> (String, HashMap<String, String>) {
  // Type of html list building
  enum ListType {
    NoList,
    Unordered,
    Ordered,
  }
  use ListType::*;

  // Return values
  let mut body = Vec::<String>::new();
  let mut meta = HashMap::<String, String>::new();
  // Build values
  let mut is_meta = true;
  let mut active_list = ListType::NoList;

  // Creating line styles (headers, lists, ect)
  // Loop lines in input
  let lines: Vec<String> = input.lines().map(|x| x.to_string()).collect();
  for line in lines {
    // Split line into `token` and `rest` at first space
    // If no space, token is line, rest is empty
    let (token, rest) = match line.find(' ') {
      Some(pos) => line.split_at(pos),
      None => (line.as_str(), ""),
    };
    let rest = rest.trim();

    // If metadata is currently building
    if is_meta {
      // Add metadata if tag starts with '@', or preset header tags
      match token {
        c if c.starts_with('@') => {
          meta.insert(token.split_at(1).1.to_string(), rest.to_string());
        }
        "#" => {
          meta.insert("headline".to_string(), rest.to_string());
        }
        "##" => {
          meta.insert("desc".to_string(), rest.to_string());
        }
        "---" => is_meta = false,
        _ => (),
      }
    } else {
      // Add closing list tag, if token does not match with list pattern
      if !Regex::new(r"^-$|^\d+\.$").unwrap().is_match(token) {
        match active_list {
          NoList => (),
          Unordered => body.push("</ol>".to_string()),
          Ordered => body.push("</ul>".to_string()),
        }
        active_list = NoList;
      }

      // Add tags if token matches
      let maybe_push = match token {
        // Header
        c if Regex::new(r"^#+$").unwrap().is_match(c) => Some(format!(
          "<h{d}> {} </h{d}>",
          escape_html(rest),
          d = c.len() + 1,
        )),

        // Quote
        ">" => Some(format!("<blockquote> {} </blockquote>", escape_html(rest))),

        // Hr
        "---" => Some("<hr />".to_string()),

        // Unordered list
        "-" => {
          // Add opening list tag if not active, and close other previous list if active
          let parent = match active_list {
            NoList => "<ul>",
            Ordered => "",
            Unordered => "</ol>\n<ul>\n",
          };
          active_list = Ordered;

          Some(format!("{parent}<li> {} </li>", escape_html(rest)))
        }

        // Ordered list
        c if Regex::new(r"^\d+\.$").unwrap().is_match(c) => {
          // Add opening list tag if not active, and close other previous list if active
          let parent = match active_list {
            NoList => "<ol>",
            Ordered => "</ul>\n<ol>\n",
            Unordered => "",
          };
          active_list = Unordered;

          Some(format!("{parent}<li> {} </li>", escape_html(rest)))
        }

        // Comment
        c if Regex::new(r"^~{3,}").unwrap().is_match(c) => None,
        // Normal line
        _ => {
          let s = line.trim();
          if !s.is_empty() {
            Some(format!("<p> {} </p>\n", escape_html(s)))
          } else {
            None
          }
        }
      };

      // Push line if not None
      if let Some(do_push) = maybe_push {
        body.push(do_push.trim().to_string());
      }
    }
  }

  (body.join("\n"), meta)
}
