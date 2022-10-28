use crate::prices::Prices;
use std::ops::Range;

pub fn get_cheapest_hours(
    hours: u32,
    consecutive: bool,
    prices: &Prices,
    range: Option<Range<u32>>,
) -> Vec<u32> {
    let range = range.unwrap_or(0..24);

    let prices = prices.today();

    let mut on_hours: Vec<u32> = Vec::new();

    if consecutive {
        if range.end - range.start < hours {
            for hour in range {
                on_hours.push(hour);
            }
        } else {
            let mut cheapest_hour: u32 = 0;
            let mut cheapest_sum = u32::MAX;

            for hour in range.start..range.end - hours {
                let mut sum: u32 = 0;

                for i in 0..hours {
                    sum += prices[(hour + i) as usize] as u32;
                }

                if sum < cheapest_sum {
                    cheapest_hour = hour;
                    cheapest_sum = sum;
                }
            }

            for i in 0..hours {
                on_hours.push(cheapest_hour + i);
            }
        }
    } else {
        let mut prices_hour: Vec<(u32, u32)> = Vec::new();

        for hour in 0..prices.len() {
            let price = prices[hour];

            prices_hour.push((price as u32, hour as u32));
        }

        prices_hour.sort_by(|l, r| l.0.cmp(&r.0));

        let prices_hour: Vec<&(u32, u32)> = prices_hour
            .iter()
            .filter(|ph| range.contains(&ph.1))
            .collect();

        for hour in 0..hours {
            let price_hour = prices_hour[hour as usize];

            on_hours.push(price_hour.1);
        }
    }

    on_hours
}
