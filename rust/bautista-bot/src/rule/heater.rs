use super::util::get_cheapest_hours;
use super::RuleEval;
use crate::prices::PowerPrices;
use chrono::Timelike;
use chrono::{DateTime, Local};

pub struct RuleHeater {
    hours: i64,
    on_at: Option<Vec<bool>>,
    pivot_hour: i64,
}

impl RuleHeater {
    pub fn new(pivot_hour: i64, hours: i64) -> RuleHeater {
        RuleHeater {
            hours,
            on_at: None,
            pivot_hour,
        }
    }
}

impl RuleEval for RuleHeater {
    fn eval(&mut self, now: &DateTime<Local>) -> Option<bool> {
        match &self.on_at {
            None => None,
            Some(on_at) => Some(*on_at.get(now.hour() as usize).unwrap()),
        }
    }

    fn is_consumed(&self) -> bool {
        false
    }

    fn update_prices(&mut self, prices: &PowerPrices) -> () {
        match get_cheapest_hours(self.hours, false, prices) {
            None => {
                self.on_at = None;
            }

            Some(cheapest_hours) => {
                let mut on_at: Vec<bool> = vec![false; 24];

                for hour in cheapest_hours {
                    on_at[hour as usize] = true;
                }

                self.on_at = Some(on_at);
            }
        };
    }
}
