use self::meross::MerossBridge;
use crate::*;
use std::collections::HashMap;
use thiserror::Error;

mod meross;

#[derive(Error, Debug)]
pub enum DevicesError {
    #[error("Meross bridge returned error: {0}")]
    BridgeError(String),
}

pub struct Devices<'a> {
    bridge: MerossBridge,
    cfg: &'a Config,
    map: HashMap<String, Option<bool>>,
}

impl<'a> Devices<'a> {
    pub fn new(cfg: &'a Config) -> Devices<'a> {
        let mut map: HashMap<String, Option<bool>> = HashMap::new();

        for device in &cfg.meross.devices {
            map.insert(String::from(device), None);
        }

        Devices {
            bridge: MerossBridge::new(cfg).expect("error starting meross-bridge"),
            cfg,
            map,
        }
    }

    /**
     * Returns a HashMap containing the devices that have changed and the
     * result.
     */
    pub fn commit(&mut self) -> HashMap<String, Result<bool>> {
        let mut result: HashMap<String, Result<bool>> = HashMap::new();

        let map = self.map.clone();

        for (device, on) in map.iter() {
            if !self.cfg.is_controlled(device) {
                continue;
            }

            if let Some(on) = on {
                //eprintln!("Commit {} {}", device, on);

                let status;

                match self.send("STATUS", device) {
                    Err(err) => {
                        result.insert(String::from(device), Err(err));
                        continue;
                    }

                    Ok(reply) => {
                        status = reply == "ON";
                    }
                }

                if *on == status {
                    continue;
                }

                let cmd = if *on { "TURNON" } else { "TURNOFF" };

                if let Err(err) = self.send(cmd, device) {
                    result.insert(String::from(device), Err(err));

                    continue;
                }

                result.insert(String::from(device), Ok(*on));
            }
        }

        result
    }

    pub fn update(&mut self, update: &HashMap<String, Option<bool>>) -> () {
        for (device, on) in update.iter() {
            if let Some(on) = on {
                //eprintln!("Toggle {} {}", device, on);

                self.map.insert(String::from(device), Some(*on));
            }
        }
    }

    fn send(&mut self, cmd: &str, msg: &str) -> Result<String> {
        self.bridge.send_text(&format!("{} {}", cmd, msg))?;

        match self.bridge.get_reply() {
            Err(err) => Err(err),

            Ok(reply) => {
                if reply.starts_with("> OK ") {
                    Ok(String::from(&reply[5..]))
                } else {
                    Err(Box::new(DevicesError::BridgeError(String::from(
                        &reply[8..],
                    ))))
                }
            }
        }
    }
}
