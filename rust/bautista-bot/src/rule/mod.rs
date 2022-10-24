use crate::prices::PowerPrices;
use crate::*;
use chrono::{DateTime, Local};
use std::collections::HashMap;

mod cheap;
mod heater;

pub use cheap::RuleCheap;
pub use heater::RuleHeater;

pub trait RuleEval {
    fn eval(&mut self, now: &DateTime<Local>) -> Option<bool>;
    fn is_consumed(&self) -> bool;
    fn update_prices(&mut self, prices: &PowerPrices) -> ();
}

pub struct DeviceRules {
    map: HashMap<String, Vec<Rule>>,
}

impl DeviceRules {
    pub fn new(cfg: &Config) -> DeviceRules {
        let mut device_rules = DeviceRules {
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

    pub fn update_prices(&mut self, prices: &PowerPrices) -> () {
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
