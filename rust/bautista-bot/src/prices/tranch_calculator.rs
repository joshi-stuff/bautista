use super::Tranch;

pub struct TranchCalculator {
    p0: f32,
    p25: f32,
    p75: f32,
    p100: f32,
}

impl TranchCalculator {
    pub fn new(prices: &Vec<f32>) -> TranchCalculator {
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

    pub fn get_tranch(&self, price: f32) -> Tranch {
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
