use super::util::*;
use super::*;
use crate::prices::Prices;

pub struct RuleHeater {
    hours: u32,
    /** The hour after when water must be hot */
    pivot_hour: u32,
}

impl RuleHeater {
    pub fn new(pivot_hour: u32, hours: u32) -> RuleHeater {
        RuleHeater { hours, pivot_hour }
    }
}

impl Rule for RuleHeater {
    fn get_on_hours(&self, prices: &Prices, hours: Range<u32>) -> OnHours {
        let mut cheapest_hours = get_cheapest_hours(self.hours, false, prices, Some(hours));

        // TODO: exclude pivot_hours not in range
        for hour in get_cheapest_hours(self.hours, false, prices, Some(0..self.pivot_hour)) {
            if !cheapest_hours.contains(&hour) {
                cheapest_hours.push(hour);
            }
        }

        OnHours::from_hours(cheapest_hours)
    }
}
