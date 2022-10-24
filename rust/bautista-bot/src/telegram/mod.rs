use crate::*;
use reqwest::blocking::Client;
use serde::de::DeserializeOwned;
use std::cmp::max;
use std::collections::HashMap;
use thiserror::Error;
use urlencoding::encode;

mod api;

#[derive(Debug, Error)]
pub enum TelegramError {
    #[error("Telegram API call failed")]
    CallFailed,
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

    pub fn send_message(&self, user_id: i64, text: &str) -> Result<()> {
        let reply: api::Reply<api::Message> = self.get(
            "sendMessage",
            HashMap::from([
                ("chat_id", format!("{}", user_id)),
                ("text", String::from(text)),
            ]),
        )?;

        if !reply.ok {
            return Err(Box::new(TelegramError::CallFailed));
        }

        Ok(())
    }

    fn get<T: DeserializeOwned>(&self, method: &str, params: HashMap<&str, String>) -> Result<T> {
        let mut url = format!(
            "https://api.telegram.org/bot{}/{}?",
            self.config.telegram.token, method
        );

        for name in params.keys() {
            let value = encode(&params.get(name).unwrap());

            url.push_str(name);
            url.push_str("=");
            url.push_str(&value);
            url.push_str("&");
        }

        //dbg!(&url);

        Ok(self.client.get(url).send()?.json()?)
    }
}
