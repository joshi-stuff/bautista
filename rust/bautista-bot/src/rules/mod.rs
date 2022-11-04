use crate::prices::Prices;
use crate::*;
use chrono::{DateTime, Local, Timelike};
use std::collections::HashMap;
use std::ops::Range;
use thiserror::Error;

mod cheap;
mod heater;
mod util;

pub use cheap::RuleCheap;
pub use heater::RuleHeater;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Cannot update prices")]
    UpdatePricesFailed(#[from] prices::Error),
}

pub trait Rule {
    fn get_on_hours(&self, prices: &Prices, hours: Range<u32>) -> OnHours;
}

#[derive(Clone)]
pub struct OnHours {
    on_hours: Vec<bool>,
}

impl OnHours {
    fn from_hours(hours: Vec<u32>) -> OnHours {
        let mut on_hours = vec![false; 24];

        for hour in hours {
            on_hours[hour as usize] = true;
        }

        OnHours { on_hours }
    }

    pub fn on_at(&self, hour: u32) -> bool {
        self.on_hours[hour as usize]
    }
}

pub struct Rules {
    device_rules: HashMap<String, Box<dyn Rule>>,
    get_on_hours_0_24: Option<HashMap<String, OnHours>>,
    prices: Prices,
}

impl Rules {
    pub fn new(cfg: &Config) -> Rules {
        let mut device_rules = HashMap::new();

        for device in &cfg.meross.devices {
            if let Some(rule) = cfg.get_rule(device) {
                let rule: Box<dyn Rule> = match rule.rule_type().as_str() {
                    "cheap" => {
                        let consecutive = rule.get_bool("consecutive");
                        let hours = rule.get_u32("hours");

                        Box::new(RuleCheap::new(consecutive, hours))
                    }

                    "heater" => {
                        let pivot_hour = rule.get_u32("pivot_hour");
                        let hours = rule.get_u32("hours");

                        Box::new(RuleHeater::new(pivot_hour, hours))
                    }

                    _ => {
                        panic!("Invalid rule type for device {}", device);
                    }
                };

                device_rules.insert(device.clone(), rule);
            }
        }

        Rules {
            device_rules,
            get_on_hours_0_24: None,
            prices: Prices::new(&cfg),
        }
    }

    /**
     * Return a map of results per each device.
     *
     * The map guarantees one result per device, but it can be missing for
     * devices for which no rule was applied.
     *
     * Otherwise it will be a boolean with the desired status of the device
     * according to the rules.
     */
    pub fn eval(&self, now: &DateTime<Local>) -> HashMap<String, Option<bool>> {
        let mut result: HashMap<String, Option<bool>> = HashMap::new();

        let on_hours = self.get_on_hours(0..24);

        for (device, on_hours) in on_hours {
            result.insert(device.clone(), Some(on_hours.on_at(now.hour())));
        }

        result
    }

    /**
     * Return a map of devices to vectors containing hours when devices must be
     * turned on.
     */
    pub fn get_on_hours(&self, hours: Range<u32>) -> HashMap<String, OnHours> {
        // Lookup cached value
        if hours.start == 0 && hours.end == 24 {
            if let Some(get_on_hours) = &self.get_on_hours_0_24 {
                return get_on_hours.clone();
            }
        }

        // If not cached, calculate it
        let mut result: HashMap<String, OnHours> = HashMap::new();

        for device in self.devices() {
            let rule = self.device_rules.get(&device).unwrap();

            let on_hours = rule.get_on_hours(&self.prices, hours.start..hours.end);

            result.insert(device, on_hours);
        }

        result
    }

    pub fn prices(&self) -> &Prices {
        &self.prices
    }

    pub fn update_prices(&mut self) -> Result<bool, Error> {
        let updated = self.prices.update()?;

        if updated {
            self.get_on_hours_0_24 = Some(self.get_on_hours(0..24));
        }

        Ok(updated)
    }

    fn devices(&self) -> Vec<String> {
        let devices: Vec<&String> = self.device_rules.keys().collect();

        let devices: Vec<String> = devices.iter().map(|key| String::from(*key)).collect();

        devices
    }
}
