use log::info;
use reqwest::Response;
use std::env;
use std::error::Error;

pub mod resources;

async fn make_request(url: &str) -> Result<Response, Box<dyn Error>> {
    info!("Sending API request to: {}", url);
    let resp = reqwest::get(url).await?;
    Ok(resp)
}

/// Get the Covalent API key from environment variables
fn get_env_api_key() -> Result<String, Box<dyn Error>> {
    match env::var("COVALENT_SIFTER_API_KEY") {
        Ok(val) => Ok(val),
        Err(e) => match e {
            std::env::VarError::NotPresent => {
                Err("Required environment variable {} is not present".into())
            }
            std::env::VarError::NotUnicode(_) => {
                Err("Environment variable {} is not valid unicode".into())
            }
        },
    }
}

pub struct CovalentClient {
    pub base_url: String,
    pub chain_id: String,
    pub api_key: String,
}

impl CovalentClient {
    /// Create a new CovalentClient bound to a certain chain_id
    /// ## Klaytn Client Example
    /// ```
    /// let klaytn_client = CovalentClient::new("8127", "<YOUR_API_KEY>").unwrap();
    /// ```
    pub fn new(chain_id: &str, api_key: &str) -> Result<CovalentClient, Box<dyn Error>> {
        Ok(CovalentClient {
            base_url: "https://api.covalenthq.com/v1".to_string(),
            chain_id: chain_id.to_string(),
            api_key: api_key.to_string(),
        })
    }

    /// Create a new CovalentClient bound to a certain chain_id
    /// take the environment variable COVALENT_SIFTER_API_KEY
    pub fn new_env_api_key(chain_id: &str) -> Result<CovalentClient, Box<dyn Error>> {
        Ok(CovalentClient {
            base_url: "https://api.covalenthq.com/v1".to_string(),
            chain_id: chain_id.to_string(),
            api_key: get_env_api_key()?,
        })
    }

    /// Get token balance information for an address
    pub async fn get_token_balances(
        &self,
        address: &str,
    ) -> Result<resources::Balances, Box<dyn Error>> {
        let resp = make_request(
            format!(
                "/{}/address/{}/balances_v2/?key={}",
                self.chain_id, address, self.api_key
            )
            .as_str(),
        )
        .await?;
        let resource: resources::Balances = resp.json().await?;
        Ok(resource)
    }

    /// Get token holders at a block height for an address
    pub async fn get_token_holders_any_bh(
        &self,
        address: &str,
    ) -> Result<resources::TokenHolders, Box<dyn Error>> {
        let resp = make_request(
            format!(
                "/{}/address/{}/token_holders/?key={}",
                self.chain_id, address, self.api_key
            )
            .as_str(),
        )
        .await?;
        let resource: resources::TokenHolders = resp.json().await?;
        Ok(resource)
    }

    /// Get transactions for an address
    pub async fn get_transactions_for_address(
        &self,
        address: &str,
    ) -> Result<resources::Transactions, Box<dyn Error>> {
        let resp = make_request(
            format!(
                "/{}/address/{}/transactions_v2/?key={}",
                self.chain_id, address, self.api_key
            )
            .as_str(),
        )
        .await?;
        let resource: resources::Transactions = resp.json().await?;
        Ok(resource)
    }

    pub async fn get_transaction(
        &self,
        tx_hash: &str,
    ) -> Result<resources::Transactions, Box<dyn Error>> {
        let resp = make_request(
            format!(
                "/{}/address/{}/transaction_v2/?key={}",
                self.chain_id, tx_hash, self.api_key
            )
            .as_str(),
        )
        .await?;
        let resource: resources::Transactions = resp.json().await?;
        Ok(resource)
    }
}
