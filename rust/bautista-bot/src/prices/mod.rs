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
    today_prices: Vec<Price>,
    token: String,
    tomorrow_prices: Option<Vec<Price>>,
}

#[derive(Clone)]
pub enum Tranch {
    High,
    Med,
    Low,
}

impl Tranch {
    fn get_icon(&self) -> &str {
        match self {
            Tranch::High => "🔴",
            Tranch::Med => "🟡",
            Tranch::Low => "🟢",
        }
    }
}

struct TranchCalculator {
    p0: f32,
    p25: f32,
    p75: f32,
    p100: f32,
}

impl TranchCalculator {
    fn new(prices: &Vec<f32>) -> TranchCalculator {
        let p0 = prices
            .iter()
            .min_by(|l, r| l.partial_cmp(&r).unwrap())
            .unwrap();

        let p100 = prices
            .iter()
            .max_by(|l, r| l.partial_cmp(&r).unwrap())
            .unwrap();

        let p25 = p0 + ((p100 - p0) / 4.);

        let p75 = p100 - (p25 - p0);

        TranchCalculator {
            p0: *p0,
            p25,
            p75,
            p100: *p100,
        }
    }

    fn get_tranch(&self, price: f32) -> Tranch {
        if price < self.p0 {
            panic!("Price too low: {}", &price);
        } else if price < self.p25 {
            Tranch::Low
        } else if price < self.p75 {
            Tranch::Med
        } else if price <= self.p100 {
            Tranch::High
        } else {
            panic!("Price too high: {}", &price);
        }
    }
}

#[derive(Clone)]
pub struct Price {
    pub euros_per_kwh: f32,
    pub tranch: Tranch,
}

impl Price {
    fn new(euros_per_kwh: f32, tranch: Tranch) -> Price {
        Price {
            euros_per_kwh,
            tranch,
        }
    }
}

impl Display for Price {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> result::Result<(), fmt::Error> {
        fmt.write_str(&format!(
            "{} {}",
            self.tranch.get_icon(),
            self.euros_per_kwh
        ))
    }
}

impl Prices {
    pub fn new(cfg: &Config) -> Prices {
        Prices {
            client: Client::new(),
            last_update: Local.ymd(1980, 1, 1),
            today_prices: vec![Price::new(0., Tranch::Med); 24],
            token: String::from(&cfg.esios.token),
            tomorrow_prices: None,
        }
    }

    pub fn today(&self) -> Vec<Price> {
        self.today_prices.clone()
    }

    pub fn tomorrow(&self) -> Option<Vec<Price>> {
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

            if as_date(&reply.h00.date) == self.last_update {
                return Ok(false);
            }

            let mut today_prices: Vec<f32> = Vec::new();

            today_prices.push(reply.h00.price as f32 / 1000.0);
            today_prices.push(reply.h01.price as f32 / 1000.0);
            today_prices.push(reply.h02.price as f32 / 1000.0);
            today_prices.push(reply.h03.price as f32 / 1000.0);
            today_prices.push(reply.h04.price as f32 / 1000.0);
            today_prices.push(reply.h05.price as f32 / 1000.0);
            today_prices.push(reply.h06.price as f32 / 1000.0);
            today_prices.push(reply.h07.price as f32 / 1000.0);
            today_prices.push(reply.h08.price as f32 / 1000.0);
            today_prices.push(reply.h09.price as f32 / 1000.0);
            today_prices.push(reply.h10.price as f32 / 1000.0);
            today_prices.push(reply.h11.price as f32 / 1000.0);
            today_prices.push(reply.h12.price as f32 / 1000.0);
            today_prices.push(reply.h13.price as f32 / 1000.0);
            today_prices.push(reply.h14.price as f32 / 1000.0);
            today_prices.push(reply.h15.price as f32 / 1000.0);
            today_prices.push(reply.h16.price as f32 / 1000.0);
            today_prices.push(reply.h17.price as f32 / 1000.0);
            today_prices.push(reply.h18.price as f32 / 1000.0);
            today_prices.push(reply.h19.price as f32 / 1000.0);
            today_prices.push(reply.h20.price as f32 / 1000.0);
            today_prices.push(reply.h21.price as f32 / 1000.0);
            today_prices.push(reply.h22.price as f32 / 1000.0);
            today_prices.push(reply.h23.price as f32 / 1000.0);

            let tranch_calculator = TranchCalculator::new(&today_prices);

            self.today_prices = today_prices
                .iter()
                .map(|euros_per_kwh| {
                    Price::new(*euros_per_kwh, tranch_calculator.get_tranch(*euros_per_kwh))
                })
                .collect();
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
        fmt.write_str("· Hoy:\n")?;

        for i in 0..24 {
            let price = &self.today_prices[i];

            fmt.write_str(&format!("    {}:00  {} €/KWh\n", i, &price))?;
        }

        if let Some(tomorrow_prices) = &self.tomorrow_prices {
            fmt.write_str("· Mañana:\n")?;

            for i in 0..24 {
                let price = &tomorrow_prices[i];

                fmt.write_str(&format!("    {}:00  {} €/KWh\n", i, &price))?;
            }
        }

        Ok(())
    }
}

fn as_date(pdll_date: &str) -> Date<Local> {
    let parts: Vec<&str> = pdll_date.split("-").collect();

    let today = Local::today();

    let day: u32 = parts[0].parse().unwrap_or(today.day());
    let month: u32 = parts[1].parse().unwrap_or(today.month());
    let year: i32 = parts[2].parse().unwrap_or(today.year());

    Local.ymd(year, month, day)
}
