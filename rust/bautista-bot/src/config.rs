use serde::Deserialize;
use std::fs;

pub struct Config {
    pub bautista: Bautista,
    pub telegram: Telegram,
}

#[derive(Deserialize)]
pub struct Bautista {
    pub poll_seconds: i32,
}

#[derive(Deserialize)]
pub struct Telegram {
    pub admin_user: i64,
    pub allowed_users: Vec<i64>,
    pub token: String,
}

#[derive(Deserialize)]
pub struct Toml {
    pub bautista: Bautista,
    pub telegram: Telegram,
}

impl Config {
    pub fn new() -> Config {
        let toml = fs::read_to_string("/etc/bautista/config.toml")
            .expect("Failed to load /etc/bautista/config.toml");

        let toml: Toml = toml::from_str(&toml).unwrap();

        Config {
            bautista: toml.bautista,
            telegram: toml.telegram,
        }
    }
}
