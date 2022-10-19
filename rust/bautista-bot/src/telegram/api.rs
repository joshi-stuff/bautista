use serde::Deserialize;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Update {
    pub update_id: i64,
    pub message: Option<Message>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Message {
    pub message_id: i64,
    pub from: Option<User>,
    pub date: i32,
    pub chat: Chat,
    pub text: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Chat {
    pub id: i64,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct User {
    pub id: i64,
    pub is_bot: bool,
    pub first_name: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Reply<T> {
    pub ok: bool,
    pub result: T,
}
