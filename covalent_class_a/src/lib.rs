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
    let required_env = "COVALENT_SIFTER_API_KEY";
    match env::var(required_env) {
        Ok(val) => Ok(val),
        Err(e) => match e {
            std::env::VarError::NotPresent => Err(format!(
                "Required environment variable {} is not present",
                required_env
            )
            .into()),
            std::env::VarError::NotUnicode(_) => {
                Err(format!("Environment variable {} is not valid unicode", required_env).into())
            }
        },
    }
}
fn add_pagination_params(
    mut endpoint: String,
    page_size: Option<&str>,
    page_number: Option<&str>,
) -> String {
    if page_size.is_some() {
        endpoint = format!("{}&page-size={}", endpoint, page_size.unwrap())
    }
    if page_number.is_some() {
        endpoint = format!("{}&page-number={}", endpoint, page_number.unwrap())
    }
    endpoint
}

#[derive(Clone)]
pub struct CovalentClient {
    pub base_url: String,
    pub chain_id: String,
    pub api_key: String,
}

impl CovalentClient {
    /// Create a new CovalentClient bound to a crtain chain_id
    /// ## Klaytn Client Example
    /// ```
    /// #[tokio::main]
    /// async fn main() {
    /// let klaytn_client = covalent_class_a::CovalentClient::new_env_api_key("8217").unwrap();
    /// let balances: covalent_class_a::resources::BalancesData = klaytn_client.get_token_balances("0xf4024faad5fafd0755875e3161524c9c4e1a1111", None, None).await.unwrap();
    /// println!("Address: {}", balances.data.address);
    /// }
    /// ```
    pub fn new(chain_id: &str, api_key: &str) -> Result<CovalentClient, Box<dyn Error>> {
        Ok(CovalentClient {
            base_url: "https://api.covalenthq.com/v1".to_string(),
            chain_id: chain_id.to_string(),
            api_key: api_key.to_string(),
        })
    }

    /// Create a new CovalentClient bound to a certain chain_id
    /// takes the environment variable COVALENT_SIFTER_API_KEY
    /// ```
    /// #[tokio::main]
    /// async fn main() {
    /// // In shell: export COVALENT_SIFTER_API_KEY = <YOUR_API_KEY>
    /// let klaytn_client = covalent_class_a::CovalentClient::new_env_api_key("8217").unwrap();
    /// // Make a get_token_balances request with pagination options page_size set to 10 and page_number set to 1
    /// let balances: covalent_class_a::resources::BalancesData = klaytn_client.get_token_balances("0xf4024faad5fafd0755875e3161524c9c4e1a1111", Some("10"), Some("1")).await.unwrap();
    /// }
    /// ```
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
        addr: &str,
        page_size: Option<&str>,
        page_number: Option<&str>,
    ) -> Result<resources::BalancesData, Box<dyn Error>> {
        let mut endpoint = format!(
            "{}/{}/address/{}/balances_v2/?key={}",
            self.base_url, self.chain_id, addr, self.api_key
        );
        endpoint = add_pagination_params(endpoint, page_size, page_number);

        let resp = make_request(&endpoint).await?;
        let resource: resources::BalancesData = resp.json().await?;
        Ok(resource)
    }

    /// Get historicial portfolio values for an address
    pub async fn get_historical_portfolio_value(
        &self,
        addr: &str,
        page_size: Option<&str>,
        page_number: Option<&str>,
    ) -> Result<resources::HistoricalPortfolioData, Box<dyn Error>> {
        let mut endpoint = format!(
            "{}/{}/address/{}/portfolio_v2/?key={}",
            self.base_url, self.chain_id, addr, self.api_key
        );
        endpoint = add_pagination_params(endpoint, page_size, page_number);

        let resp = make_request(&endpoint).await?;
        let resource: resources::HistoricalPortfolioData = resp.json().await?;
        Ok(resource)
    }

    /// Get token holders at a block height for an address
    pub async fn get_token_holders_any_bh(
        &self,
        addr: &str,
        page_size: Option<&str>,
        page_number: Option<&str>,
    ) -> Result<resources::TokenHoldersData, Box<dyn Error>> {
        let mut endpoint = format!(
            "{}/{}/tokens/{}/token_holders/?key={}",
            self.base_url, self.chain_id, addr, self.api_key
        );
        endpoint = add_pagination_params(endpoint, page_size, page_number);

        let resp = make_request(&endpoint).await?;
        let resource: resources::TokenHoldersData = resp.json().await?;
        Ok(resource)
    }

    /// Get transactions for an address
    pub async fn get_transactions_for_address(
        &self,
        addr: &str,
        page_size: Option<&str>,
        page_number: Option<&str>,
    ) -> Result<resources::TransactionsData, Box<dyn Error>> {
        let mut endpoint = format!(
            "{}/{}/address/{}/transactions_v2/?key={}",
            self.base_url, self.chain_id, addr, self.api_key
        );
        endpoint = add_pagination_params(endpoint, page_size, page_number);

        let resp = make_request(&endpoint).await?;
        let resource: resources::TransactionsData = resp.json().await?;
        Ok(resource)
    }

    /// Get information on a single transaction
    pub async fn get_transaction(
        &self,
        tx_hash: &str,
        page_size: Option<&str>,
        page_number: Option<&str>,
    ) -> Result<resources::TransactionData, Box<dyn Error>> {
        let mut endpoint = format!(
            "{}/{}/transaction_v2/{}/?key={}",
            self.base_url, self.chain_id, tx_hash, self.api_key
        );
        endpoint = add_pagination_params(endpoint, page_size, page_number);

        let resp = make_request(&endpoint).await?;
        let resource: resources::TransactionData = resp.json().await?;
        Ok(resource)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lazy_static::lazy_static;

    lazy_static! {
        static ref KLAYTN_ADDR: String = "0xf4024faad5fafd0755875e3161524c9c4e1a1111".to_string();
        static ref KLAYTN_TX_HASH: String =
            "0x269fad968de5baf8d324b64d0a19df72ccfc762b33e1760729633f4946e0c863".to_string();
    }

    fn setup_klaytn_client() -> CovalentClient {
        CovalentClient::new_env_api_key("8217").expect("Failed to create Klaytn Covalent client")
    }

    #[tokio::test]
    async fn test_get_token_balances() {
        let client = setup_klaytn_client();
        let balance = client
            .get_token_balances(&KLAYTN_ADDR, None, None)
            .await
            .expect("Should receive valid balance");
        assert_eq!(KLAYTN_ADDR.as_str(), balance.data.address);
    }

    #[tokio::test]
    async fn test_get_historical_portfolio_value() {
        let client = setup_klaytn_client();
        let _historical_portfolio_value = client
            .get_historical_portfolio_value(&KLAYTN_ADDR, Some("10"), Some("1"))
            .await
            .expect("Should receive valid historical portfolio value");
    }

    #[tokio::test]
    async fn test_get_token_holders_any_bh() {
        let client = setup_klaytn_client();
        let _holders = client
            .get_token_holders_any_bh(&KLAYTN_ADDR, None, None)
            .await
            .expect("Should receive valid token holder");
    }

    #[tokio::test]
    async fn test_get_transactions_for_address() {
        let client = setup_klaytn_client();
        let transactions = client
            .get_transactions_for_address(&KLAYTN_ADDR, None, None)
            .await
            .expect("Should receive valid balance");
        assert_eq!(KLAYTN_ADDR.as_str(), transactions.data.address);
    }

    #[tokio::test]
    async fn test_get_transaction() {
        let client = setup_klaytn_client();
        let _transaction = client
            .get_transaction(&KLAYTN_TX_HASH, None, None)
            .await
            .expect("Should receive valid balance");
    }
}
