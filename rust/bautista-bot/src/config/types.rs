use serde::Deserialize;

#[derive(Deserialize)]
pub struct Bautista {
    pub poll_seconds: i32,
}

#[derive(Deserialize)]
pub struct Device {
    pub cheap: Option<RuleCheap>,
    pub control: Option<bool>,
    pub heater: Option<RuleHeater>,
    pub rule: String,
}

#[derive(Deserialize)]
pub struct Meross {
    pub bridge_path: Option<String>,
    pub devices: Vec<String>,
    pub password: String,
    pub user: String,
}

#[derive(Deserialize)]
pub struct RuleHeater {
    pub pivot_hour: i32,
}

#[derive(Deserialize)]
pub struct RuleCheap {
    pub hours: i32,
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
