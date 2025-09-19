use web3::transports::Http;
use web3::Web3;
use anyhow::Result;

// Add the RpcClient struct definition (missing in your code)
#[derive(Debug, Clone)]
pub struct RpcClient {
    pub url: String,
}

impl RpcClient {
    pub fn new(url: &str) -> Self {
        RpcClient { url: url.to_string() }
    }

    // Replace the call_contract method with get_web3 method
    pub async fn get_web3(&self) -> Result<Web3<Http>> {
        let http = Http::new(&self.url)?;
        Ok(Web3::new(http))
    }

    // // Optional: Keep call_contract if you need it for other purposes
    // pub async fn call_contract(&self, contract: &str, data: &[u8]) -> Result<Vec<u8>> {
    //     let web3 = self.get_web3().await?;
    //     let contract_address: web3::types::H160 = contract.parse()?;
        
    //     let tx = web3::types::TransactionRequest {
    //         to: Some(contract_address),
    //         data: Some(data.into()),
    //         ..Default::default()
    //     };

    //     let result = web3.eth().call(tx, None).await?;
    //     Ok(result.0) // Return raw bytes instead of string
    // }
}