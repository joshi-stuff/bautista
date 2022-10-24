use super::RuleEval;
use crate::prices::PowerPrices;
use chrono::{DateTime, Local};

pub struct RuleHeater {
    pivot_hour: i64,
}

impl RuleHeater {
    pub fn new(pivot_hour: i64) -> RuleHeater {
        RuleHeater { pivot_hour }
    }
}

impl RuleEval for RuleHeater {
    fn eval(&mut self, now: &DateTime<Local>) -> Option<bool> {
        Some(true)
    }

    fn is_consumed(&self) -> bool {
        false
    }

    fn update_prices(&mut self, _prices: &PowerPrices) -> () {}
}
