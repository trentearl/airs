use serde::Deserialize;
use serde::Serialize;

use crate::io;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub default_profile: Option<String>,
}

pub fn read_config() -> Config {
    let config_string = io::read_config();
    let config: Config = serde_json::from_str(&config_string).unwrap();

    config
}

pub fn set_default_profile(s: String) {
    let mut config = read_config();
    config.default_profile = Some(s);

    io::write_config(serde_json::to_string_pretty(&config).unwrap());
}
