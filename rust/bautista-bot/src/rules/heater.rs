use super::util::get_cheapest_hours;
use super::RuleEval;
use crate::prices::Prices;
use chrono::{DateTime, Local, Timelike};

pub struct RuleHeater {
    hours: u32,
    on_at: Option<Vec<bool>>,
    /** The hour after when water must be hot */
    pivot_hour: u32,
}

impl RuleHeater {
    pub fn new(pivot_hour: u32, hours: u32) -> RuleHeater {
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

    fn update_prices(&mut self, prices: &Prices) -> () {
        let cheapest_hours = get_cheapest_hours(self.hours, false, prices, None);
        let before_pivot_cheapest_hours =
            get_cheapest_hours(self.hours, false, prices, Some(0..self.pivot_hour));

        // Update on_at vector
        let mut on_at: Vec<bool> = vec![false; 24];

        for hour in before_pivot_cheapest_hours {
            on_at[hour as usize] = true;
        }
        for hour in cheapest_hours {
            on_at[hour as usize] = true;
        }

        self.on_at = Some(on_at);
    }
}
