use crate::config::Config;
use crate::status::Status;
use crate::Result;
use reqwest::blocking::Client;
use serde::de::DeserializeOwned;
use std::cmp::max;
use std::collections::HashMap;
use thiserror::Error;

mod api;

#[derive(Debug, Error)]
pub enum TelegramError {
    #[error("Telegram API call failed")]
    CallFailed,
    /*
    #[error("data store disconnected")]
    Disconnect(#[from] io::Error),
    #[error("the data for key `{0}` is not available")]
    Redaction(String),
    #[error("invalid header (expected {expected:?}, found {found:?})")]
    InvalidHeader {
        expected: String,
        found: String,
    },
    */
}

pub struct Bot<'a> {
    client: Client,
    config: &'a Config,
    status: &'a mut Status,
}

#[derive(Debug)]
pub struct Message {
    pub user_id: i64,
    pub user_name: String,
    pub text: String,
}

impl<'a> Bot<'a> {
    pub fn new(config: &'a Config, status: &'a mut Status) -> Bot {
        Bot {
            client: Client::new(),
            config,
            status,
        }
    }

    pub fn get_new_messages(&mut self, timeout_seconds: i32) -> Result<Vec<Message>> {
        let reply: api::Reply<Vec<api::Update>> = self.get(
            "getUpdates",
            HashMap::from([
                (
                    "offset",
                    format!("{}", self.status.telegram.last_update_id() + 1),
                ),
                ("timeout", format!("{}", timeout_seconds)),
                ("allowed_updates", format!("[\"message\"]")),
            ]),
        )?;

        if !reply.ok {
            return Err(Box::new(TelegramError::CallFailed));
        }

        let mut msgs: Vec<Message> = Vec::new();

        for update in reply.result {
            self.status
                .telegram
                .set_last_update_id(max(self.status.telegram.last_update_id(), update.update_id))?;

            let msg = update.message.unwrap();
            let from = msg.from.unwrap();

            msgs.push(Message {
                user_id: from.id,
                user_name: from.first_name,
                text: msg.text.unwrap_or(String::new()),
            });
        }

        Ok(msgs)
    }

    fn get<T: DeserializeOwned>(&self, method: &str, params: HashMap<&str, String>) -> Result<T> {
        let mut url = format!(
            "https://api.telegram.org/bot{}/{}?",
            self.config.telegram.token, method
        );

        for name in params.keys() {
            url.push_str(name);
            url.push_str("=");
            url.push_str(&params.get(name).unwrap());
            url.push_str("&");
        }

        Ok(self.client.get(url).send()?.json()?)
    }
}
