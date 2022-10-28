use super::*;
use crate::telegram::Message;

pub struct HelloCommand {}

impl HelloCommand {
    pub fn new() -> HelloCommand {
        HelloCommand {}
    }
}

impl Command for HelloCommand {
    fn run(&self, msg: &Message, _rules: &Rules) -> Result<Option<String>> {
        let text = &msg.text;

        if !text.starts_with("/hola") {
            return Ok(None);
        }

        Ok(Some(format!("Hola {} 👋", &msg.user_name)))
    }
}
