use self::pdll::PddlReply;
use crate::*;
use chrono::prelude::*;
use reqwest::blocking::Client;
use std::fmt::{self, Display, Formatter};
use std::result;

//mod esios;
mod pdll;

pub struct Prices {
    client: Client,
    last_update: Date<Local>,
    // TODO: make prices u64
    today_prices: Vec<i64>,
    token: String,
    tomorrow_prices: Option<Vec<i64>>,
}

impl Prices {
    pub fn new(cfg: &Config) -> Prices {
        Prices {
            client: Client::new(),
            last_update: Local.ymd(1980, 1, 1),
            today_prices: vec![0; 24],
            token: String::from(&cfg.esios.token),
            tomorrow_prices: None,
        }
    }

    pub fn today(&self) -> Vec<i64> {
        self.today_prices.clone()
    }

    pub fn tomorrow(&self) -> Option<Vec<i64>> {
        self.tomorrow_prices.clone()
    }

    pub fn update(&mut self) -> Result<bool> {
        if self.token == "" {
            let today = Local::now();
            let now = today.time();

            if self.last_update == today.date() || (now.hour() == 0 && now.minute() < 3) {
                return Ok(false);
            }

            let reply: PddlReply = self
                .client
                .get("https://api.preciodelaluz.org/v1/prices/all?zone=PCB")
                .send()?
                .json()?;

            let mut today_prices: Vec<i64> = Vec::new();

            today_prices.push((reply.h00.price * 100.0) as i64);
            today_prices.push((reply.h01.price * 100.0) as i64);
            today_prices.push((reply.h02.price * 100.0) as i64);
            today_prices.push((reply.h03.price * 100.0) as i64);
            today_prices.push((reply.h04.price * 100.0) as i64);
            today_prices.push((reply.h05.price * 100.0) as i64);
            today_prices.push((reply.h06.price * 100.0) as i64);
            today_prices.push((reply.h07.price * 100.0) as i64);
            today_prices.push((reply.h08.price * 100.0) as i64);
            today_prices.push((reply.h09.price * 100.0) as i64);
            today_prices.push((reply.h10.price * 100.0) as i64);
            today_prices.push((reply.h11.price * 100.0) as i64);
            today_prices.push((reply.h12.price * 100.0) as i64);
            today_prices.push((reply.h13.price * 100.0) as i64);
            today_prices.push((reply.h14.price * 100.0) as i64);
            today_prices.push((reply.h15.price * 100.0) as i64);
            today_prices.push((reply.h16.price * 100.0) as i64);
            today_prices.push((reply.h17.price * 100.0) as i64);
            today_prices.push((reply.h18.price * 100.0) as i64);
            today_prices.push((reply.h19.price * 100.0) as i64);
            today_prices.push((reply.h20.price * 100.0) as i64);
            today_prices.push((reply.h21.price * 100.0) as i64);
            today_prices.push((reply.h22.price * 100.0) as i64);
            today_prices.push((reply.h23.price * 100.0) as i64);

            self.today_prices = today_prices;
            self.last_update = today.date();

            Ok(true)
        } else {
            todo!("Implement e·sios");

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
            */
        }
    }
}

impl Display for Prices {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> result::Result<(), fmt::Error> {
        // TODO: show 🟢, 🟡, and 🔴 next to prices

        fmt.write_str("· Hoy:\n")?;

        for i in 0..24 {
            fmt.write_str(&format!(
                "    {}:00  {} €/KWh\n",
                i,
                self.today_prices[i] as f32 / 100000.0
            ))?;
        }

        if let Some(tomorrow_prices) = &self.tomorrow_prices {
            fmt.write_str("· Mañana:\n")?;

            for i in 0..24 {
                fmt.write_str(&format!(
                    "    {}:00  {} €/KWh\n",
                    i,
                    tomorrow_prices[i] as f32 / 100000.0
                ))?;
            }
        }

        Ok(())
    }
}
