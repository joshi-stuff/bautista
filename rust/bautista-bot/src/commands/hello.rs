use super::*;
use crate::telegram::Message;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {}

pub struct HelloCommand {}

impl HelloCommand {
    pub fn new() -> HelloCommand {
        HelloCommand {}
    }
}

impl Command for HelloCommand {
    fn run(&self, msg: &Message, _rules: &Rules) -> Result<Option<String>, super::Error> {
        let text = &msg.text;

        if !text.starts_with("/hola") {
            return Ok(None);
        }

        Ok(Some(format!("Hola {} 👋", &msg.user_name)))
    }
}
