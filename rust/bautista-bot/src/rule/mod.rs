use crate::*;
use std::collections::HashMap;

mod cheap;
mod heater;

pub use cheap::RuleCheap;
pub use heater::RuleHeater;

pub trait RuleEval {
    fn eval(&self) -> Option<bool>;
    fn is_consumed(&self) -> bool;
}

#[derive(Debug)]
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
    pub fn eval(&mut self) -> HashMap<String, Option<bool>> {
        let mut result: HashMap<String, Option<bool>> = HashMap::new();

        for (device, rules) in self.map.iter() {
            result.insert(String::from(device), None);

            for rule in rules.iter().rev() {
                let rule = as_rule_eval(rule);

                match rule.eval() {
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
        let devices: Vec<&String> = self.map.keys().collect();

        let devices: Vec<String> = devices.iter().map(|key| String::from(*key)).collect();

        for device in devices {
            let rules = self.map.get_mut(&device).unwrap();

            for i in (0..rules.len()).rev() {
                let rule = as_rule_eval(rules.get(i).unwrap());

                if rule.is_consumed() {
                    rules.remove(i);
                }
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
}

pub fn as_rule_eval(rule: &Rule) -> &dyn RuleEval {
    match rule {
        Rule::Heater(rule) => rule,
        Rule::Cheap(rule) => rule,
    }
}
