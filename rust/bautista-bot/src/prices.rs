use crate::*;
use chrono::{Date, Local};
use reqwest::blocking::Client;
use serde::Deserialize;
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
    today_date: Option<Date<Local>>,
    token: String,
    tomorrow: Option<Vec<i64>>,
}

#[derive(Debug, Deserialize)]
struct ApiReply {
    #[serde(alias = "00-01")]
    h00: ApiPrice,
    #[serde(alias = "01-02")]
    h01: ApiPrice,
    #[serde(alias = "02-03")]
    h02: ApiPrice,
    #[serde(alias = "03-04")]
    h03: ApiPrice,
    #[serde(alias = "04-05")]
    h04: ApiPrice,
    #[serde(alias = "05-06")]
    h05: ApiPrice,
    #[serde(alias = "06-07")]
    h06: ApiPrice,
    #[serde(alias = "07-08")]
    h07: ApiPrice,
    #[serde(alias = "08-09")]
    h08: ApiPrice,
    #[serde(alias = "09-10")]
    h09: ApiPrice,
    #[serde(alias = "10-11")]
    h10: ApiPrice,
    #[serde(alias = "11-12")]
    h11: ApiPrice,
    #[serde(alias = "12-13")]
    h12: ApiPrice,
    #[serde(alias = "13-14")]
    h13: ApiPrice,
    #[serde(alias = "14-15")]
    h14: ApiPrice,
    #[serde(alias = "15-16")]
    h15: ApiPrice,
    #[serde(alias = "16-17")]
    h16: ApiPrice,
    #[serde(alias = "17-18")]
    h17: ApiPrice,
    #[serde(alias = "18-19")]
    h18: ApiPrice,
    #[serde(alias = "19-20")]
    h19: ApiPrice,
    #[serde(alias = "20-21")]
    h20: ApiPrice,
    #[serde(alias = "21-22")]
    h21: ApiPrice,
    #[serde(alias = "22-23")]
    h22: ApiPrice,
    #[serde(alias = "23-24")]
    h23: ApiPrice,
}

#[derive(Debug, Deserialize)]
struct ApiPrice {
    price: f64,
}

impl PowerPrices {
    pub fn new(cfg: &Config) -> PowerPrices {
        PowerPrices {
            client: Client::new(),
            today: None,
            today_date: None,
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
        if let Some(today_date) = self.today_date {
            if Local::now().date() == today_date {
                return Ok(false);
            }
        }

        let reply: ApiReply = self
            .client
            .get("https://api.preciodelaluz.org/v1/prices/all?zone=PCB")
            .send()?
            .json()?;

        let mut today: Vec<i64> = Vec::new();

        today.push((reply.h00.price * 100.0) as i64);
        today.push((reply.h01.price * 100.0) as i64);
        today.push((reply.h02.price * 100.0) as i64);
        today.push((reply.h03.price * 100.0) as i64);
        today.push((reply.h04.price * 100.0) as i64);
        today.push((reply.h05.price * 100.0) as i64);
        today.push((reply.h06.price * 100.0) as i64);
        today.push((reply.h07.price * 100.0) as i64);
        today.push((reply.h08.price * 100.0) as i64);
        today.push((reply.h09.price * 100.0) as i64);
        today.push((reply.h10.price * 100.0) as i64);
        today.push((reply.h11.price * 100.0) as i64);
        today.push((reply.h12.price * 100.0) as i64);
        today.push((reply.h13.price * 100.0) as i64);
        today.push((reply.h14.price * 100.0) as i64);
        today.push((reply.h15.price * 100.0) as i64);
        today.push((reply.h16.price * 100.0) as i64);
        today.push((reply.h17.price * 100.0) as i64);
        today.push((reply.h18.price * 100.0) as i64);
        today.push((reply.h19.price * 100.0) as i64);
        today.push((reply.h20.price * 100.0) as i64);
        today.push((reply.h21.price * 100.0) as i64);
        today.push((reply.h22.price * 100.0) as i64);
        today.push((reply.h23.price * 100.0) as i64);

        self.today = Some(today);
        self.today_date = Some(Local::now().date());

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
