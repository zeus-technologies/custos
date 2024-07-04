use std::sync::OnceLock;

use serde::Deserialize;

pub static CONFIG: OnceLock<Config> = OnceLock::new();

#[derive(Debug, Deserialize)]
pub struct Config {
    pub thread_count: usize,
    pub database_location: String,
    pub scan_directories: Vec<String>,
}

fn init_config() -> Config {
    let contents = match std::fs::read_to_string("custos.toml") {
        Ok(contents) => contents,
        Err(_) => {
            panic!("failed to load configuration file")
        }
    };

    match toml::from_str(&contents) {
        Ok(data) => data,
        Err(_) => {
            panic!("failed to parse configuration file")
        }
    }
}

pub fn get() -> &'static Config {
    CONFIG.get_or_init(init_config)
}
