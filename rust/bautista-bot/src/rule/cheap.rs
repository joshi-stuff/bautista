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

            let mut on_at: Vec<bool> = vec![false; 24];

            if self.consecutive {
                let mut cheapest_hour: i64 = 0;
                let mut cheapest_sum = i64::MAX;

                for hour in 0..24 - self.hours {
                    let mut sum = 0;

                    for i in 0..self.hours {
                        sum += prices[(hour + i) as usize];
                    }

                    eprintln!("> {} {}", &hour, &sum);

                    if sum < cheapest_sum {
                        cheapest_hour = hour;
                        cheapest_sum = sum;
                    }
                }

                for i in 0..self.hours {
                    on_at[(cheapest_hour + i) as usize] = true;
                }
            } else {
                prices.sort();

                for i in 0..self.hours {
                    on_at[prices[i as usize] as usize] = true;
                }
            }

            self.on_at = Some(on_at);
        }
    }
}
