use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Toml {
    pub telegram: Telegram,
}

#[derive(Deserialize, Serialize)]
pub struct Telegram {
    pub last_update_id: i64,
}
