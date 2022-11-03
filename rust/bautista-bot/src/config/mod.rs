use std::collections::HashMap;
use std::fs;
use toml::Value;

mod types;

pub struct Config {
    pub bautista: Bautista,
    pub esios: Esios,
    pub meross: Meross,
    pub rules: HashMap<String, Rule>,
    pub telegram: Telegram,
    pub toml: Value,
}

pub struct Bautista {
    pub poll_seconds: u32,
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

pub struct Rule {
    device: String,
    toml: Value,
}

impl Rule {
    pub fn rule_type(&self) -> String {
        self.get_string("rule")
    }

    pub fn get_bool(&self, key: &str) -> bool {
        self.toml
            .get(key)
            .expect(&format!(
                "Parameter {} not found in device {}",
                key, &self.device
            ))
            .as_bool()
            .expect(&format!(
                "Parameter {} for device {} must be a boolean",
                key, &self.device
            ))
    }

    pub fn get_string(&self, key: &str) -> String {
        String::from(
            self.toml
                .get(key)
                .expect(&format!(
                    "Parameter {} not found in device {}",
                    key, &self.device
                ))
                .as_str()
                .expect(&format!(
                    "Parameter {} for device {} must be a string",
                    key, &self.device
                )),
        )
    }

    pub fn get_u32(&self, key: &str) -> u32 {
        self.toml
            .get(key)
            .expect(&format!(
                "Parameter {} not found in device {}",
                key, &self.device
            ))
            .as_integer()
            .expect(&format!(
                "Parameter {} for device {} must be an unsigned 32-bit integer",
                key, &self.device
            )) as u32
    }
}

pub struct Telegram {
    pub admin_user: i64,
    pub allowed_users: Vec<i64>,
    pub token: String,
}

impl<'a> Config {
    pub fn new() -> Config {
        let toml = fs::read_to_string("/etc/bautista/config.toml")
            .expect("Failed to load /etc/bautista/config.toml");

        let value = toml
            .parse::<Value>()
            .expect("failed to parse /etc/bautista/config.toml");

        let toml: types::Toml =
            toml::from_str(&toml).expect("failed to parse /etc/bautista/config.toml");

        let mut rules = HashMap::new();

        for device in &toml.meross.devices {
            if let Some(cfg) = value.get("device") {
                if let Some(cfg) = cfg.get(device) {
                    rules.insert(
                        device.clone(),
                        Rule {
                            device: device.to_string(),
                            toml: cfg.clone(),
                        },
                    );
                }
            }
        }

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
                    .unwrap_or(String::from("/usr/lib/bautista/meross-bridge-launcher")),
                devices: toml.meross.devices,
                password: toml.meross.password,
                user: toml.meross.user,
            },
            rules,
            telegram: Telegram {
                admin_user: toml.telegram.admin_user,
                allowed_users: toml.telegram.allowed_users,
                token: toml.telegram.token,
            },
            toml: value,
        }
    }

    pub fn get_rule(&self, device: &str) -> Option<&Rule> {
        self.rules.get(device)
    }

    pub fn is_controlled(&self, device: &str) -> bool {
        if let Some(rule) = self.rules.get(device) {
            rule.get_bool("control")
        } else {
            false
        }
    }
}
