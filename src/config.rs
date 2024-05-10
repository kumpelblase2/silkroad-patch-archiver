use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{ErrorKind, Read, Write};
use std::path::Path;

#[derive(Copy, Clone, Deserialize, Serialize)]
pub struct Configuration {
    pub base_version: u32,
    pub current_version: u32,
}

impl Default for Configuration {
    fn default() -> Self {
        Configuration {
            base_version: 635,
            current_version: 635,
        }
    }
}

pub fn load_config() -> Configuration {
    try_load_config().unwrap_or_default()
}

fn try_load_config() -> std::io::Result<Configuration> {
    let config_path = Path::new("./config.toml");
    if !Path::exists(config_path) || !config_path.is_file() {
        return Err(std::io::Error::new(
            ErrorKind::NotFound,
            "Config file is missing or not a file.",
        ));
    }

    let mut config_file = File::open(config_path)?;

    let mut content = String::new();
    config_file.read_to_string(&mut content)?;
    let res = toml::from_str::<Configuration>(&content)
        .map_err(|err| std::io::Error::new(ErrorKind::Other, err))?;
    Ok(res)
}

pub fn save_config(config: &Configuration) -> std::io::Result<()> {
    let config_path = Path::new("./config.toml");
    if config_path.exists() && !config_path.is_file() {
        return Err(std::io::Error::new(
            ErrorKind::AlreadyExists,
            "config file is a directory.",
        ));
    }

    let content = toml::to_string_pretty(&config)
        .map_err(|err| std::io::Error::new(ErrorKind::Other, err))?;
    let mut config_file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(config_path)?;

    config_file.write_all(&content.as_bytes())?;

    Ok(())
}
