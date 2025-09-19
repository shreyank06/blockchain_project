pub struct Opportunity {
    pub buy_dex: String,
    pub sell_dex: String,
    pub buy_price: f64,
    pub sell_price: f64,
}

pub fn detect(price_a: f64, price_b: f64, threshold: f64) -> Option<Opportunity> {
    if (price_b - price_a) > threshold {
        Some(Opportunity {
            buy_dex: "DEX_A".to_string(),
            sell_dex: "DEX_B".to_string(),
            buy_price: price_a,
            sell_price: price_b,
        })
    } else if (price_a - price_b) > threshold {
        Some(Opportunity {
            buy_dex: "DEX_B".to_string(),
            sell_dex: "DEX_A".to_string(),
            buy_price: price_b,
            sell_price: price_a,
        })
    } else {
        None
    }
}
