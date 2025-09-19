use crate::arbitrage::Opportunity;

pub fn simulate(opportunity: &Opportunity, trade_size: f64) -> f64 {
    let gross_profit = (opportunity.sell_price - opportunity.buy_price) * trade_size;
    
    // More realistic gas cost estimation (in USDC)
    let gas_cost = match trade_size {
        size if size > 10000.0 => 5.0,   // Large trades: $5 gas
        size if size > 1000.0 => 2.0,    // Medium trades: $2 gas
        _ => 1.0,                        // Small trades: $1 gas
    };
    
    gross_profit - gas_cost
}