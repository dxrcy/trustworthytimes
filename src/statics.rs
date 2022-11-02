use lazy_static::lazy_static;
use std::{collections::HashMap, fs};

use crate::get_file_name;

/// URL without paths for website in production
pub const BASE_URL_PROD: &str = "https://trustworthytimes.github.io/";

/// Where static files are build to
/// Should be `docs` to allow publishing on Github Pages
/// Must not include `.` or `/` before or after
pub const DIR_BUILD: &str = "docs";
// Where `.news` files are stored
pub const DIR_NEWS: &str = "news";
// Where template files are stored
pub const DIR_TEMPLATES: &str = "templates";
pub const DIR_PARTIALS: &str = "templates/partials";

// Check if args contains `--dev` flag
lazy_static! {
  pub static ref IS_DEV: bool = std::env::args()
    .collect::<Vec<_>>()
    .contains(&"--dev".to_string());
}

// URL or filepath for website
lazy_static! {
  pub static ref BASE_URL: String =
  if *IS_DEV
  {
    // Current directory plus build directory
    std::env::current_dir()
      .unwrap()
      .to_str()
      .unwrap()
      .to_string() + "/"
      + DIR_BUILD
  } else {
    // Production directory
    BASE_URL_PROD.to_string()
  };
}

// Compile all partials from files and constants
lazy_static! {
  pub static ref PARTIALS: HashMap<String, String> = {
    let mut partials = HashMap::<String, String>::new();

    // Add const partials
    partials.insert(
      "BASE_URL".to_string(),
      if *IS_DEV {
        format!("file:///{}", *BASE_URL)
      } else {
        BASE_URL.to_string()
      },
    );
    // Only if in dev mode
    if *IS_DEV {
      // Include extension for html file
      partials.insert("IF_EXT".to_string(), ".html".to_string());
      // Include index.html file for path
      partials.insert("IF_FILE".to_string(), "/index.html".to_string());
    }

    // Read template directory
    let files = fs::read_dir(format!("./{DIR_PARTIALS}")).expect("Could not read partials directory");
    // Loop files
    for file in files.flatten() {
      if let Some(name) = get_file_name(&file) {
        partials.insert(
          name,
          fs::read_to_string(file.path()).expect("Could not read partial"),
        );
      }
    }

    partials
  };
}
