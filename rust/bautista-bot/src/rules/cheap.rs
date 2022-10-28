use super::util::*;
use super::*;
use crate::prices::Prices;
use crate::rules::OnHours;

pub struct RuleCheap {
    consecutive: bool,
    hours: u32,
}

impl RuleCheap {
    pub fn new(consecutive: bool, hours: u32) -> RuleCheap {
        RuleCheap { consecutive, hours }
    }
}

impl Rule for RuleCheap {
    fn get_on_hours(&self, prices: &Prices, hours: Range<u32>) -> OnHours {
        OnHours::from_hours(get_cheapest_hours(
            self.hours,
            self.consecutive,
            prices,
            Some(hours),
        ))
    }
}
