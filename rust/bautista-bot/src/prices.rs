use crate::*;
use reqwest::blocking::Client;
use std::fmt::{self, Display, Formatter};
use std::result;
use thiserror::Error;

const GEOID: i64 = 8741;
const URL: &str = "https://api.esios.ree.es/indicators/1001";

#[derive(Debug, Error)]
pub enum EsiosError {
    #[error("E·sios API call failed: {0}")]
    CallFailed(String),
}

pub struct PowerPrices {
    client: Client,
    today: Option<Vec<i64>>,
    token: String,
    tomorrow: Option<Vec<i64>>,
}

impl PowerPrices {
    pub fn new(cfg: &Config) -> PowerPrices {
        PowerPrices {
            client: Client::new(),
            today: None,
            token: String::from(&cfg.esios.token),
            tomorrow: None,
        }
    }

    pub fn today(&self) -> Option<Vec<i64>> {
        self.today.clone()
    }

    pub fn tomorrow(&self) -> Option<Vec<i64>> {
        self.tomorrow.clone()
    }

    pub fn update(&mut self) -> Result<bool> {
        if let Some(_) = self.today {
            return Ok(false);
        }

        let mut today = vec![09936; 24];

        today[2] = 08345;
        today[5] = 08341;
        today[8] = 08342;
        today[11] = 09000;

        self.today = Some(today);

        Ok(true)
        /*
        let response = self
            .client
            .get(URL)
            .header(
                "Accept",
                "application/json; application/vnd.esios-api-v2+json",
            )
            .header("Content-Type", "application/json")
            .header("Host", "api.esios.ree.es")
            .header("Authorization", &format!("Token token=\"{}\"", self.token))
            .send()?;

        let status = response.status();

        if !status.is_success() {
            return Err(Box::new(EsiosError::CallFailed(status.to_string())));
        }

        // TODO: read reply

        Ok(false)
        */
    }
}

impl Display for PowerPrices {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> result::Result<(), fmt::Error> {
        if let Some(today) = &self.today {
            fmt.write_str("· Hoy:\n")?;

            for i in 0..24 {
                fmt.write_str(&format!(
                    "    {}:00  {} €/KWh\n",
                    i,
                    today[i] as f32 / 100000.0
                ))?;
            }
        }

        if let Some(tomorrow) = &self.tomorrow {
            fmt.write_str("· Mañana:\n")?;

            for i in 0..24 {
                fmt.write_str(&format!(
                    "    {}:00  {} €/KWh\n",
                    i,
                    tomorrow[i] as f32 / 100000.0
                ))?;
            }
        }

        Ok(())
    }
}
