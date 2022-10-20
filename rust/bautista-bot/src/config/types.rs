use serde::Deserialize;

#[derive(Deserialize)]
pub struct Bautista {
    pub poll_seconds: i32,
}

#[derive(Deserialize)]
pub struct Meross {
    pub user: String,
    pub password: String,
    pub bridge_path: Option<String>,
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
    pub meross: Meross,
    pub telegram: Telegram,
}
