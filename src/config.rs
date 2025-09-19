use std::env;
use dotenv::dotenv;

pub struct Config {
    pub rpc_url: String,
    pub dex_a: String,
    pub dex_b: String,
    pub pair: (String, String),
    pub threshold: f64,
    pub trade_size: f64,
}

impl Config {
    pub fn load() -> Self {
        // Load environment variables from a .env file
        dotenv().ok();

        // Get the values from the environment, using default if missing
        let rpc_url = env::var("RPC_URL").expect("RPC_URL not set in .env");
        let dex_a = env::var("DEX_A").expect("DEX_A not set in .env");
        let dex_b = env::var("DEX_B").expect("DEX_B not set in .env");

        let pair = (
            env::var("TOKEN_A").expect("TOKEN_A not set in .env"),
            env::var("TOKEN_B").expect("TOKEN_B not set in .env"),
        );

        let threshold: f64 = env::var("THRESHOLD")
            .expect("THRESHOLD not set in .env")
            .parse()
            .expect("Invalid threshold value");

        let trade_size: f64 = env::var("TRADE_SIZE")
            .expect("TRADE_SIZE not set in .env")
            .parse()
            .expect("Invalid trade_size value");

        Config {
            rpc_url,
            dex_a,
            dex_b,
            pair,
            threshold,
            trade_size,
        }
    }
}
