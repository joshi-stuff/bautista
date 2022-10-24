use super::util::get_cheapest_hours;
use super::RuleEval;
use crate::prices::PowerPrices;
use chrono::Timelike;
use chrono::{DateTime, Local};

pub struct RuleCheap {
    consecutive: bool,
    hours: i64,
    on_at: Option<Vec<bool>>,
}

impl RuleCheap {
    pub fn new(consecutive: bool, hours: i64) -> RuleCheap {
        RuleCheap {
            consecutive,
            hours,
            on_at: None,
        }
    }
}

impl RuleEval for RuleCheap {
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
        match get_cheapest_hours(self.hours, self.consecutive, prices) {
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
