use lazy_static::lazy_static;
use std::fs::File;
use::std::io::prelude::*;
use toml;
use serde::Deserialize;

lazy_static! {
    // Global configuration variable. 
    pub static ref CONFIG: Config = Config::new();
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub doc_root: String,
    pub default_root_file: Option<String>,
    pub root_file : Option<String>,
    pub max_buffer: Option<usize>,
    pub custom_404: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            host: "127.0.0.1".to_string(),
            port: 8080,
            doc_root: "http".to_string(),
            default_root_file: Some("index.html".to_string()),
            root_file: None,
            max_buffer: Some(2048),
            custom_404: None,
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
