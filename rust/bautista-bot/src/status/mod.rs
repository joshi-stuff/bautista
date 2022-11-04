use crate::Result;
use std::error;
use std::fs;
use std::sync::{Arc, RwLock};
use thiserror::Error;

mod types;

const DEFAULT_PARSED_STATUS_TOML: types::Toml = types::Toml {
    telegram: types::Telegram { last_update_id: 0 },
};
const DEFAULT_STATUS_TOML: &str = "\
[telegram]
last_update_id = 0
";

#[derive(Debug, Error)]
pub enum Error {
    #[error("Cannot save application persistent status")]
    IOFailed(#[source] Box<dyn error::Error>),
}

pub struct Status {
    pub telegram: Telegram,
}

struct Store {
    toml: types::Toml,
}

pub struct Telegram {
    store: Arc<RwLock<Store>>,
}

impl Status {
    pub fn new() -> Status {
        let toml = fs::read_to_string("/run/bautista/status.toml")
            .unwrap_or(String::from(DEFAULT_STATUS_TOML));

        let toml: types::Toml = toml::from_str(&toml).unwrap_or(DEFAULT_PARSED_STATUS_TOML);

        let store = Arc::new(RwLock::new(Store { toml }));

        Status {
            telegram: Telegram {
                store: Arc::clone(&store),
            },
        }
    }
}

impl Store {
    fn save(&self) -> Result<(), Error> {
        let toml: String = toml::to_string_pretty(&self.toml)
            .or_else(|err| Err(Error::IOFailed(Box::new(err))))?;

        fs::write("/run/bautista/status.toml", &toml)
            .or_else(|err| Err(Error::IOFailed(Box::new(err))))?;

        Ok(())
    }
}

impl Telegram {
    pub fn last_update_id(&self) -> i64 {
        let store = self.store.read().unwrap();

        store.toml.telegram.last_update_id
    }

    pub fn set_last_update_id(&mut self, last_update_id: i64) -> Result<(), Error> {
        let mut store = self.store.write().unwrap();

        store.toml.telegram.last_update_id = last_update_id;

        store.save()?;

        Ok(())
    }
}
