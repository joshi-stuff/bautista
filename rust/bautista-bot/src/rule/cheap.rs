use super::RuleEval;
use crate::prices::PowerPrices;
use chrono::{DateTime, Local, Timelike};

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
        if let None = prices.today() {
            self.on_at = None;
        } else {
            let mut prices = prices.today().unwrap();

            if self.consecutive {
                // TODO: Implement cheap consecutive rule
                self.on_at = None
            } else {
                let mut on_at: Vec<bool> = vec![false; 24];

                prices.sort();

                for i in 0..self.hours {
                    on_at[prices[i as usize] as usize] = true;
                }

                self.on_at = Some(on_at);
            }
        }
    }
}
