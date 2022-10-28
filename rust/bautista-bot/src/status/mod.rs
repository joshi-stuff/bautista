use crate::Result;
use std::fs;
use std::sync::{Arc, RwLock};

mod types;

pub struct Status {
    pub telegram: Telegram,
}

pub struct Telegram {
    store: Arc<RwLock<Store>>,
}

struct Store {
    toml: types::Toml,
}

impl Status {
    pub fn new() -> Status {
        let toml = match fs::read_to_string("/run/bautista/status.toml") {
            Err(_) => String::from(
                "
                [telegram]
                last_update_id = 0
                ",
            ),
            Ok(toml) => toml,
        };

        let toml: types::Toml = toml::from_str(&toml).unwrap();

        let store = Arc::new(RwLock::new(Store { toml }));

        Status {
            telegram: Telegram {
                store: Arc::clone(&store),
            },
        }
    }
}

impl Store {
    fn save(&self) -> Result<()> {
        let toml: String = toml::to_string_pretty(&self.toml)?;

        fs::write("/run/bautista/status.toml", &toml)?;

        Ok(())
    }
}

impl Telegram {
    pub fn last_update_id(&self) -> i64 {
        let store = self.store.read().unwrap();

        store.toml.telegram.last_update_id
    }

    pub fn set_last_update_id(&mut self, last_update_id: i64) -> Result<()> {
        let mut store = self.store.write().unwrap();

        store.toml.telegram.last_update_id = last_update_id;

        store.save()?;

        Ok(())
    }
}
