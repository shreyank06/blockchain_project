use crate::{rpc::RpcClient, dex::Dex};
use web3::types::{H160, U256};
use web3::contract::Contract;
use anyhow::Result;

pub async fn fetch_price(client: &RpcClient, dex: &Dex, pair: &(String, String)) -> Result<f64> {
    let web3 = client.get_web3().await?;
    
    println!("Parsing router address: {}", dex.router_address);
    let router_address: H160 = dex.router_address.parse()
        .map_err(|e| anyhow::anyhow!("Failed to parse router address {}: {}", dex.router_address, e))?;
    
    println!("Parsing token A: {}", pair.0);
    let token_a: H160 = pair.0.parse()
        .map_err(|e| anyhow::anyhow!("Failed to parse token A address {}: {}", pair.0, e))?;
    
    println!("Parsing token B: {}", pair.1);
    let token_b: H160 = pair.1.parse()
        .map_err(|e| anyhow::anyhow!("Failed to parse token B address {}: {}", pair.1, e))?;

    let abi = include_bytes!("../abis/uniswap_v2_router.json");
    
    let contract = Contract::from_json(web3.eth(), router_address, abi)
        .map_err(|e| anyhow::anyhow!("Failed to create contract: {}", e))?;

    let amount_in = U256::exp10(18); // 1 token (18 decimals)
    
    let amounts_out: Vec<U256> = contract
        .query("getAmountsOut", (amount_in, vec![token_a, token_b]), None, web3::contract::Options::default(), None)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to query getAmountsOut: {}", e))?;

    let price = amounts_out[1].as_u128() as f64 / 1e6; // Adjust for USDC decimals (6)
    Ok(price)
}