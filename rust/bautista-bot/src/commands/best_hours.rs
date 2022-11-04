use super::*;
use crate::telegram::Message;
use crate::util::format_on_hours::*;
use chrono::prelude::*;
use std::cmp::min;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {}

pub struct BestHoursCommand {}

impl BestHoursCommand {
    pub fn new() -> BestHoursCommand {
        BestHoursCommand {}
    }
}

impl Command for BestHoursCommand {
    fn run(&self, msg: &Message, rules: &Rules) -> Result<Option<String>, super::Error> {
        let text = &msg.text;

        if !text.starts_with("/horario") {
            return Ok(None);
        }

        let text = &text[8..].trim();

        let from_hour = if text.len() > 0 {
            text.parse().unwrap_or(0)
        } else {
            let now = Local::now().time();

            if now.minute() <= 15 {
                now.hour()
            } else {
                now.hour() + 1
            }
        };

        let from_hour = min(23, from_hour);

        Ok(Some(format!(
            "Los mejores horarios para encender cada dispositivo a partir de las {}:00 son:\n{}",
            from_hour,
            format_on_hours(&rules.get_on_hours(from_hour..24))
        )))
    }
}
