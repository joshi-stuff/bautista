use std::fs;

mod types;

pub struct Config {
    pub bautista: Bautista,
    pub meross: Meross,
    pub telegram: Telegram,
}

pub struct Bautista {
    pub poll_seconds: i32,
}

pub struct Meross {
    pub user: String,
    pub password: String,
    pub bridge_path: String,
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

        let toml: types::Toml =
            toml::from_str(&toml).expect("failed to parse /etc/bautista/config.toml");

        Config {
            bautista: Bautista {
                poll_seconds: toml.bautista.poll_seconds,
            },
            meross: Meross {
                user: toml.meross.user,
                password: toml.meross.password,
                bridge_path: toml
                    .meross
                    .bridge_path
                    .unwrap_or(String::from("/usr/bin/meross-bridge")),
            },
            telegram: Telegram {
                admin_user: toml.telegram.admin_user,
                // TODO: add admin_user to allowed_users if missing
                allowed_users: toml.telegram.allowed_users,
                token: toml.telegram.token,
            },
        }
    }
}
