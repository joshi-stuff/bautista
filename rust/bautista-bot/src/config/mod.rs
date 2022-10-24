use crate::rule::*;
use std::fs;
use toml::Value;

mod types;

pub struct Config {
    pub bautista: Bautista,
    pub esios: Esios,
    pub meross: Meross,
    pub telegram: Telegram,
    pub toml: Value,
}

pub struct Bautista {
    pub poll_seconds: i32,
}

pub struct Esios {
    pub token: String,
}

pub struct Meross {
    pub bridge_path: String,
    pub devices: Vec<String>,
    pub password: String,
    pub user: String,
}

pub enum Rule {
    Cheap(RuleCheap),
    Heater(RuleHeater),
}

pub struct Telegram {
    pub admin_user: i64,
    pub allowed_users: Vec<i64>,
    pub token: String,
}

impl Config {
    pub fn new() -> Config {
        let toml = fs::read_to_string("/etc/bautista/config.toml")
            .expect("Failed to load /etc/bautista/config.toml");

        let value = toml
            .parse::<Value>()
            .expect("failed to parse /etc/bautista/config.toml");

        let toml: types::Toml =
            toml::from_str(&toml).expect("failed to parse /etc/bautista/config.toml");

        Config {
            bautista: Bautista {
                poll_seconds: toml.bautista.poll_seconds,
            },
            esios: Esios {
                token: toml.esios.token,
            },
            meross: Meross {
                bridge_path: toml
                    .meross
                    .bridge_path
                    .unwrap_or(String::from("/usr/bin/meross-bridge")),
                devices: toml.meross.devices,
                password: toml.meross.password,
                user: toml.meross.user,
            },
            telegram: Telegram {
                admin_user: toml.telegram.admin_user,
                // TODO: add admin_user to allowed_users if missing
                allowed_users: toml.telegram.allowed_users,
                token: toml.telegram.token,
            },
            toml: value,
        }
    }

    pub fn get_rule(&self, device: &str) -> Option<Rule> {
        let cfg = self.toml.get("device");

        if let None = cfg {
            return None;
        }

        let cfg = cfg.unwrap();

        let cfg = cfg.get(device);

        if let None = cfg {
            return None;
        }

        let cfg = cfg.unwrap();

        let rule_type = get_string(&cfg, "rule", device);

        match rule_type.as_str() {
            "cheap" => {
                let consecutive = get_bool(&cfg, "consecutive", device);
                let hours = get_integer(&cfg, "hours", device);

                Some(Rule::Cheap(RuleCheap::new(consecutive, hours)))
            }

            "heater" => {
                let pivot_hour = get_integer(&cfg, "pivot_hour", device);

                Some(Rule::Heater(RuleHeater::new(pivot_hour)))
            }

            _ => {
                panic!("Invalid rule type {} for device {}", &rule_type, device);
            }
        }
    }

    pub fn is_controlled(&self, device: &str) -> Option<bool> {
        let cfg = self.toml.get("device");

        if let None = cfg {
            return None;
        }

        let cfg = cfg.unwrap();

        let cfg = cfg.get(device);

        if let None = cfg {
            return None;
        }

        let cfg = cfg.unwrap();

        Some(get_bool(&cfg, "control", device))
    }
}

fn get_bool(cfg: &Value, key: &str, device: &str) -> bool {
    cfg.get(key)
        .expect(&format!("Parameter {} not found in device {}", key, device))
        .as_bool()
        .expect(&format!(
            "Parameter {} for device {} must be a boolean",
            key, device
        ))
}

fn get_integer(cfg: &Value, key: &str, device: &str) -> i64 {
    cfg.get(key)
        .expect(&format!("Parameter {} not found in device {}", key, device))
        .as_integer()
        .expect(&format!(
            "Parameter {} for device {} must be an integer",
            key, device
        ))
}

fn get_string(cfg: &Value, key: &str, device: &str) -> String {
    String::from(
        cfg.get(key)
            .expect(&format!("Parameter {} not found in device {}", key, device))
            .as_str()
            .expect(&format!(
                "Parameter {} for device {} must be a string",
                key, device
            )),
    )
}
