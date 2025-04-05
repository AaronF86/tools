//------------------------------------------//
//                                          //
// config.rs - configuration parser         //
//                                          //
//------------------------------------------//


use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct LanguageConfig {
    pub languages: Vec<Language>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Language {
    pub name: String,
    pub single_line_comment: String,
    pub extensions: Vec<String>,
}

const DEFAULT_LANGUAGES: &str = include_str!("default_languages.toml");

pub fn initialize_config() -> Result<(), Box<dyn std::error::Error>> {
    let config_path = dirs::home_dir()
        .expect("Failed to find home directory")
        .join(".config/bannify_config.toml");

    if !config_path.exists() {
        println!("Creating default config at {:?}", config_path);

        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::write(&config_path, DEFAULT_LANGUAGES)?;
    }

    Ok(())
}

fn get_config_path() -> PathBuf {
    dirs::home_dir()
        .expect("Could not find home directory")
        .join(".config/bannify_config.toml")
}

pub fn load_config() -> LanguageConfig {
    let path = get_config_path();

    if !path.exists() {
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(&path, "").unwrap();
    }

    let content = fs::read_to_string(&path).unwrap_or_default();
    toml::from_str(&content).unwrap_or(LanguageConfig { languages: vec![] })
}

pub fn save_config(config: &LanguageConfig) {
    let path = get_config_path();
    let content = toml::to_string_pretty(config).unwrap();
    fs::write(&path, content).unwrap();
}