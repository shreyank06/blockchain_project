mod config;
mod rpc;
mod dex;
mod price;
mod arbitrage;
mod profit;
mod db;

use config::Config;
use dex::Dex;
use sqlx::SqlitePool;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1. Load configuration
    let config = config::Config::load();

    // 2. Initialize database - handle the Result
    let db_url = "sqlite:opportunities.db";
    let pool: SqlitePool = db::init_db(db_url).await?;

    // 3. Create Dex instances
    let dex_a = Dex::new("DEX_A", &config.dex_a);
    let dex_b = Dex::new("DEX_B", &config.dex_b);

    // 4. Setup RPC client
    let client = rpc::RpcClient::new(&config.rpc_url);

    // 5. Run for 10 iterations with 5 seconds of wait time between each
    for _ in 0..10 {
        match fetch_and_check_prices(&client, &dex_a, &dex_b, &config, &pool).await {
            Ok(_) => println!("Price check completed successfully"),
            Err(e) => eprintln!("Error in price check: {}", e),
        }

        // Wait for 5 seconds before the next iteration
        sleep(Duration::from_secs(5)).await;
    }

    Ok(())
}


async fn fetch_and_check_prices(
    client: &rpc::RpcClient,
    dex_a: &Dex,
    dex_b: &Dex,
    config: &Config,
    pool: &SqlitePool,
) -> anyhow::Result<()> {
    // Fetch prices and handle errors
    let price_a = price::fetch_price(client, dex_a, &config.pair).await?;
    let price_b = price::fetch_price(client, dex_b, &config.pair).await?;

    println!("Price on {}: {:.6}", dex_a.name, price_a);
    println!("Price on {}: {:.6}", dex_b.name, price_b);

    // Now price_a and price_b are f64, not Result<f64>
    if let Some(opportunity) = arbitrage::detect(price_a, price_b, config.threshold) {
        let profit = profit::simulate(&opportunity, config.trade_size);
        
        // Log opportunity - now handles Result
        db::log_opportunity(pool, &opportunity, profit).await?;
        println!("✅ Arbitrage opportunity found! Profit: ${:.2}", profit);
    } else {
        println!("❌ No arbitrage opportunity");
    }

    Ok(())
}
