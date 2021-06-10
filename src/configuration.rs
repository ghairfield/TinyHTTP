use ::std::io::prelude::*;
use lazy_static::lazy_static;
use serde::Deserialize;
use std::fs::File;

lazy_static! {
    // Global configuration variable.
    pub static ref CONFIG: Config = Config::new();
}

/// Configure options. These are pulled from Config.toml
/// in the root directory.
#[derive(Deserialize, Debug)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub doc_root: String,
    pub image_list: Vec<String>,
    pub file_list: Vec<String>,
    pub default_root_file: Option<String>,
    pub root_file: Option<String>,
    pub max_buffer: Option<usize>,
    pub custom_404: Option<String>,
    pub print_header_information: Option<bool>,
}

/// Default program options
impl Default for Config {
    fn default() -> Self {
        Config {
            host: "127.0.0.1".to_string(),
            port: 8080,
            doc_root: "http".to_string(),
            image_list: Vec::<String>::new(),
            file_list: Vec::<String>::new(),
            default_root_file: Some("index.html".to_string()),
            root_file: None,
            max_buffer: Some(2048),
            custom_404: None,
            print_header_information: Some(false),
        }
    }
}

impl Config {
    fn new() -> Self {
        let mut config = Config::default();

        let mut file = match File::open("Config.toml") {
            Ok(file) => file,
            Err(_) => return config,
        };

        let mut c = String::new();
        file.read_to_string(&mut c).unwrap();

        config = toml::from_str(&c).unwrap();

        config
    }
}
