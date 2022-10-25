use crate::prices::Prices;
use crate::*;
use chrono::{DateTime, Datelike, Local, TimeZone};
use std::collections::HashMap;

mod cheap;
mod heater;
mod util;

pub use cheap::RuleCheap;
pub use heater::RuleHeater;

pub trait RuleEval {
    fn eval(&mut self, now: &DateTime<Local>) -> Option<bool>;
    fn is_consumed(&self) -> bool;
    fn update_prices(&mut self, prices: &Prices) -> ();
}

pub struct DeviceRules<'a> {
    cfg: &'a Config,
    map: HashMap<String, Vec<Rule>>,
}

impl<'a> DeviceRules<'a> {
    pub fn new(cfg: &'a Config) -> DeviceRules<'a> {
        let mut device_rules = DeviceRules {
            cfg,
            map: HashMap::new(),
        };

        for device in &cfg.meross.devices {
            if let Some(rule) = cfg.get_rule(device) {
                device_rules.insert(device, rule);
            }
        }

        device_rules
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
    pub fn eval(&mut self, now: &DateTime<Local>) -> HashMap<String, Option<bool>> {
        let mut result: HashMap<String, Option<bool>> = HashMap::new();

        for device in self.devices() {
            result.insert(String::from(&device), None);

            let rules = self.map.get_mut(&device).unwrap();

            for i in (0..rules.len()).rev() {
                let rule = as_mut_rule_eval(rules.get_mut(i).unwrap());

                match rule.eval(now) {
                    None => {}

                    Some(on) => {
                        result.insert(String::from(device), Some(on));
                        break;
                    }
                }
            }
        }

        result
    }

    /**
     * Return a vector with hours when uncontrolled devices must be turned on.
     */
    pub fn get_uncontrolled_on_hours(&mut self) -> HashMap<String, Vec<i64>> {
        let mut result: HashMap<String, Vec<i64>> = HashMap::new();

        let now = Local::now();

        for device in self.devices() {
            match self.cfg.is_controlled(&device) {
                None => continue,
                Some(controlled) => {
                    if controlled {
                        continue;
                    };
                }
            };

            let mut on_hours = Vec::new();

            let rules = self.map.get_mut(&device).unwrap();

            if rules.len() != 1 {
                panic!("Uncontrolled device {} can only have one rule", device);
            }

            let rule = as_mut_rule_eval(&mut rules[0]);

            for hour in 0..24 {
                let date_time = Local
                    .ymd(now.year(), now.month(), now.day())
                    .and_hms(hour, 0, 0);

                if let Some(on) = rule.eval(&date_time) {
                    if on {
                        on_hours.push(hour as i64);
                    }
                }
            }

            result.insert(device, on_hours);
        }

        result
    }

    pub fn remove_consumed(&mut self) -> () {
        for device in self.devices() {
            let rules = self.map.get_mut(&device).unwrap();

            for i in (0..rules.len()).rev() {
                let rule = as_rule_eval(rules.get(i).unwrap());

                if rule.is_consumed() {
                    rules.remove(i);
                }
            }
        }
    }

    pub fn update_prices(&mut self, prices: &Prices) -> () {
        for device in self.devices() {
            let rules = self.map.get_mut(&device).unwrap();

            for i in 0..rules.len() {
                let rule = as_mut_rule_eval(rules.get_mut(i).unwrap());

                rule.update_prices(prices);
            }
        }
    }

    fn insert(&mut self, device: &str, rule: Rule) -> () {
        if !self.map.contains_key(device) {
            self.map.insert(String::from(device), Vec::new());
        }

        let rules = self.map.get_mut(device).unwrap();

        rules.push(rule);
    }

    fn devices(&self) -> Vec<String> {
        let devices: Vec<&String> = self.map.keys().collect();
        let devices: Vec<String> = devices.iter().map(|key| String::from(*key)).collect();

        devices
    }
}

pub fn as_mut_rule_eval(rule: &mut Rule) -> &mut dyn RuleEval {
    match rule {
        Rule::Heater(rule) => rule,
        Rule::Cheap(rule) => rule,
    }
}

pub fn as_rule_eval(rule: &Rule) -> &dyn RuleEval {
    match rule {
        Rule::Heater(rule) => rule,
        Rule::Cheap(rule) => rule,
    }
}
