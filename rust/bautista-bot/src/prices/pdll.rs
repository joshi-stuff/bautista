use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PddlReply {
    #[serde(alias = "00-01")]
    pub h00: PddlPrice,
    #[serde(alias = "01-02")]
    pub h01: PddlPrice,
    #[serde(alias = "02-03")]
    pub h02: PddlPrice,
    #[serde(alias = "03-04")]
    pub h03: PddlPrice,
    #[serde(alias = "04-05")]
    pub h04: PddlPrice,
    #[serde(alias = "05-06")]
    pub h05: PddlPrice,
    #[serde(alias = "06-07")]
    pub h06: PddlPrice,
    #[serde(alias = "07-08")]
    pub h07: PddlPrice,
    #[serde(alias = "08-09")]
    pub h08: PddlPrice,
    #[serde(alias = "09-10")]
    pub h09: PddlPrice,
    #[serde(alias = "10-11")]
    pub h10: PddlPrice,
    #[serde(alias = "11-12")]
    pub h11: PddlPrice,
    #[serde(alias = "12-13")]
    pub h12: PddlPrice,
    #[serde(alias = "13-14")]
    pub h13: PddlPrice,
    #[serde(alias = "14-15")]
    pub h14: PddlPrice,
    #[serde(alias = "15-16")]
    pub h15: PddlPrice,
    #[serde(alias = "16-17")]
    pub h16: PddlPrice,
    #[serde(alias = "17-18")]
    pub h17: PddlPrice,
    #[serde(alias = "18-19")]
    pub h18: PddlPrice,
    #[serde(alias = "19-20")]
    pub h19: PddlPrice,
    #[serde(alias = "20-21")]
    pub h20: PddlPrice,
    #[serde(alias = "21-22")]
    pub h21: PddlPrice,
    #[serde(alias = "22-23")]
    pub h22: PddlPrice,
    #[serde(alias = "23-24")]
    pub h23: PddlPrice,
}

#[derive(Debug, Deserialize)]
pub struct PddlPrice {
    pub date: String,
    pub price: f64,
}
