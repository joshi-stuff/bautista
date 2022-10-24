use crate::prices::PowerPrices;

pub fn get_cheapest_hours(hours: i64, consecutive: bool, prices: &PowerPrices) -> Option<Vec<i64>> {
    if let None = prices.today() {
        None
    } else {
        let prices = prices.today().unwrap();

        let mut on_at: Vec<i64> = Vec::new();

        if consecutive {
            let mut cheapest_hour: i64 = 0;
            let mut cheapest_sum = i64::MAX;

            for hour in 0..24 - hours {
                let mut sum = 0;

                for i in 0..hours {
                    sum += prices[(hour + i) as usize];
                }

                if sum < cheapest_sum {
                    cheapest_hour = hour;
                    cheapest_sum = sum;
                }
            }

            for i in 0..hours {
                on_at.push(cheapest_hour + i);
            }
        } else {
            let mut prices_hour: Vec<(i64, i64)> = Vec::new();

            for hour in 0..prices.len() {
                let price = prices[hour];

                prices_hour.push((price, hour as i64));
            }

            prices_hour.sort_by(|l, r| l.0.cmp(&r.0));

            for hour in 0..hours {
                let price_hour = prices_hour[hour as usize];

                on_at.push(price_hour.1);
            }
        }

        Some(on_at)
    }
}
