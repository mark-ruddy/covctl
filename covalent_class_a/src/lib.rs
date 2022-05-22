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
    let required_env = "COVALENT_API_KEY";
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
    page_size: Option<String>,
    page_number: Option<String>,
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
    ///     let klaytn_client = covalent_class_a::CovalentClient::new_env_api_key("8217").unwrap();
    ///     let balances: covalent_class_a::resources::BalancesData = klaytn_client.get_token_balances("0xf4024faad5fafd0755875e3161524c9c4e1a1111", None, None).await.unwrap();
    ///     println!("Address: {}", balances.data.address);
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
    /// takes the environment variable COVALENT_API_KEY
    /// ```
    /// #[tokio::main]
    /// async fn main() {
    /// // In shell: export COVALENT_API_KEY = <YOUR_API_KEY>
    ///     let klaytn_client = covalent_class_a::CovalentClient::new_env_api_key("8217").unwrap();
    /// // Make a get_token_balances request with pagination options page_size set to 10 and page_number set to 1
    ///     let balances: covalent_class_a::resources::BalancesData = klaytn_client.get_token_balances("0xf4024faad5fafd0755875e3161524c9c4e1a1111", Some("10".to_string()), Some("1".to_string())).await.unwrap();
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
        page_size: Option<String>,
        page_number: Option<String>,
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
        page_size: Option<String>,
        page_number: Option<String>,
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

    /// Get ERC20 Token Transfers for an address and token contract address
    pub async fn get_token_transfers(
        &self,
        addr: &str,
        contract_addr: &str,
        page_size: Option<String>,
        page_number: Option<String>,
    ) -> Result<resources::TokenTransfersData, Box<dyn Error>> {
        let mut endpoint = format!(
            "{}/{}/address/{}/transfers_v2/?contract-address={}&key={}",
            self.base_url, self.chain_id, addr, contract_addr, self.api_key
        );
        endpoint = add_pagination_params(endpoint, page_size, page_number);

        let resp = make_request(&endpoint).await?;
        let resource: resources::TokenTransfersData = resp.json().await?;
        Ok(resource)
    }

    /// Get token holders at a block height for an address
    pub async fn get_token_holders_any_bh(
        &self,
        addr: &str,
        page_size: Option<String>,
        page_number: Option<String>,
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

    /// Get changes in token holders between 2 block heights
    pub async fn get_changes_in_token_holders(
        &self,
        addr: &str,
        starting_block: &str,
        ending_block: &str,
        page_size: Option<String>,
        page_number: Option<String>,
    ) -> Result<resources::ChangesInTokenHoldersData, Box<dyn Error>> {
        let mut endpoint = format!(
            "{}/{}/tokens/{}/token_holders_changes/?starting-block={}&ending-block={}&key={}",
            self.base_url, self.chain_id, addr, starting_block, ending_block, self.api_key
        );
        endpoint = add_pagination_params(endpoint, page_size, page_number);

        let resp = make_request(&endpoint).await?;
        let resource: resources::ChangesInTokenHoldersData = resp.json().await?;
        Ok(resource)
    }

    /// Get transactions for an address
    pub async fn get_transactions_for_address(
        &self,
        addr: &str,
        page_size: Option<String>,
        page_number: Option<String>,
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
        page_size: Option<String>,
        page_number: Option<String>,
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

    /// Get information on a block given a block height
    pub async fn get_a_block(
        &self,
        block_height: &str,
        page_size: Option<String>,
        page_number: Option<String>,
    ) -> Result<resources::BlockData, Box<dyn Error>> {
        let mut endpoint = format!(
            "{}/{}/block_v2/{}/?key={}",
            self.base_url, self.chain_id, block_height, self.api_key
        );
        endpoint = add_pagination_params(endpoint, page_size, page_number);

        let resp = make_request(&endpoint).await?;
        let resource: resources::BlockData = resp.json().await?;
        Ok(resource)
    }

    /// Get block heights given a start and end date
    pub async fn get_block_heights(
        &self,
        start_date: &str,
        end_date: &str,
        page_size: Option<String>,
        page_number: Option<String>,
    ) -> Result<resources::BlockData, Box<dyn Error>> {
        let mut endpoint = format!(
            "{}/{}/block_v2/{}/{}/?key={}",
            self.base_url, self.chain_id, start_date, end_date, self.api_key
        );
        endpoint = add_pagination_params(endpoint, page_size, page_number);

        let resp = make_request(&endpoint).await?;
        let resource: resources::BlockData = resp.json().await?;
        Ok(resource)
    }

    /// Get log events by contract address within a start and end block
    pub async fn get_log_events_by_contract(
        &self,
        contract_addr: &str,
        starting_block: &str,
        ending_block: &str,
        page_size: Option<String>,
        page_number: Option<String>,
    ) -> Result<resources::LogEventsGenericData, Box<dyn Error>> {
        let mut endpoint = format!(
            "{}/{}/events/address/{}/token_holders_changes/?starting-block={}&ending-block={}&key={}",
            self.base_url, self.chain_id, contract_addr, starting_block, ending_block, self.api_key
        );
        endpoint = add_pagination_params(endpoint, page_size, page_number);

        let resp = make_request(&endpoint).await?;
        let resource: resources::LogEventsGenericData = resp.json().await?;
        Ok(resource)
    }

    /// Get log events by topic hash(es)
    pub async fn get_log_events_by_topic_hashes(
        &self,
        topic_hash: &str,
        sender_addr: &str,
        starting_block: &str,
        ending_block: &str,
        page_size: Option<String>,
        page_number: Option<String>,
    ) -> Result<resources::LogEventsGenericData, Box<dyn Error>> {
        let mut endpoint = format!(
            "{}/{}/events/topics/{}/?starting-block={}&ending-block={}&sender-address={}&key={}",
            self.base_url,
            self.chain_id,
            topic_hash,
            starting_block,
            ending_block,
            sender_addr,
            self.api_key
        );
        endpoint = add_pagination_params(endpoint, page_size, page_number);

        let resp = make_request(&endpoint).await?;
        let resource: resources::LogEventsGenericData = resp.json().await?;
        Ok(resource)
    }

    /// Get all contract metadata
    pub async fn get_all_contract_metadata(
        &self,
        page_size: Option<String>,
        page_number: Option<String>,
    ) -> Result<resources::ContractMetadataData, Box<dyn Error>> {
        let mut endpoint = format!(
            "{}/{}/tokens/tokenlists/all/?key={}",
            self.base_url, self.chain_id, self.api_key
        );
        endpoint = add_pagination_params(endpoint, page_size, page_number);

        let resp = make_request(&endpoint).await?;
        let resource: resources::ContractMetadataData = resp.json().await?;
        Ok(resource)
    }

    /// Get all chains
    pub async fn get_all_chains(
        &self,
        quote_currency: &str,
        page_size: Option<String>,
        page_number: Option<String>,
    ) -> Result<resources::AllChainData, Box<dyn Error>> {
        let mut endpoint = format!(
            "{}/chains/?quote-currency={}&key={}",
            self.base_url, quote_currency, self.api_key
        );
        endpoint = add_pagination_params(endpoint, page_size, page_number);

        let resp = make_request(&endpoint).await?;
        let resource: resources::AllChainData = resp.json().await?;
        Ok(resource)
    }

    /// Get all chain statuses
    pub async fn get_all_chain_statuses(
        &self,
        quote_currency: &str,
        page_size: Option<String>,
        page_number: Option<String>,
    ) -> Result<resources::AllChainStatusesData, Box<dyn Error>> {
        let mut endpoint = format!(
            "{}/chains/status/?quote-currency={}&key={}",
            self.base_url, quote_currency, self.api_key
        );
        endpoint = add_pagination_params(endpoint, page_size, page_number);

        let resp = make_request(&endpoint).await?;
        let resource: resources::AllChainStatusesData = resp.json().await?;
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
        static ref KLAYTN_CONTRACT_DAI: String =
            "0x5c74070fdea071359b86082bd9f9b3deaafbe32b".to_string();
        static ref KLAYTN_STARTING_BLOCK: String = "91321199".to_string();
        static ref KLAYTN_ENDING_BLOCK: String = "91321208".to_string();
        static ref SAMPLE_START_DATE: String = "2022-05-18".to_string();
        static ref SAMPLE_END_DATE: String = "2022-05-20".to_string();
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
        let historical_portfolio_value = client
            .get_historical_portfolio_value(
                &KLAYTN_ADDR,
                Some("10".to_string()),
                Some("1".to_string()),
            )
            .await
            .expect("Should receive valid historical portfolio value");
        assert_eq!(8217, historical_portfolio_value.data.chain_id);
    }

    #[tokio::test]
    async fn test_get_token_transfers() {
        let client = setup_klaytn_client();
        let token_transfers = client
            .get_token_transfers(&KLAYTN_ADDR, &KLAYTN_CONTRACT_DAI, None, None)
            .await
            .expect("Should receive valid token transfers");
        assert_eq!(KLAYTN_ADDR.as_str(), token_transfers.data.address);
    }

    #[tokio::test]
    async fn test_get_token_holders_any_bh() {
        let client = setup_klaytn_client();
        let holders = client
            .get_token_holders_any_bh(&KLAYTN_ADDR, None, None)
            .await
            .expect("Should receive valid token holder");
        assert!(!holders.error.error);
    }

    #[tokio::test]
    async fn test_get_changes_in_token_holders() {
        let client = setup_klaytn_client();
        let holders_changes = client
            .get_changes_in_token_holders(
                &KLAYTN_ADDR,
                &KLAYTN_STARTING_BLOCK,
                &KLAYTN_ENDING_BLOCK,
                None,
                None,
            )
            .await
            .expect("Should receive valid token holder");
        assert!(!holders_changes.error.error)
    }

    #[tokio::test]
    async fn test_get_transactions_for_address() {
        let client = setup_klaytn_client();
        let transactions = client
            .get_transactions_for_address(&KLAYTN_ADDR, None, None)
            .await
            .expect("Should receive valid transactions");
        assert_eq!(KLAYTN_ADDR.as_str(), transactions.data.address);
    }

    #[tokio::test]
    async fn test_get_transaction() {
        let client = setup_klaytn_client();
        let transaction = client
            .get_transaction(&KLAYTN_TX_HASH, None, None)
            .await
            .expect("Should receive valid transaction");
        assert_eq!(137042, transaction.data.items[0].transaction.gas_spent);
    }

    #[tokio::test]
    async fn test_get_a_block() {
        let client = setup_klaytn_client();
        let block = client
            .get_a_block(&KLAYTN_STARTING_BLOCK, None, None)
            .await
            .expect("Should receive valid balance");
        assert!(!block.error.error)
    }

    #[tokio::test]
    async fn test_get_block_heights() {
        let client = setup_klaytn_client();
        let block = client
            .get_block_heights(&SAMPLE_START_DATE, &SAMPLE_END_DATE, None, None)
            .await
            .expect("Should receive valid balance");
        assert!(!block.error.error)
    }

    #[tokio::test]
    async fn test_get_log_events_by_contract() {
        let mut client = setup_klaytn_client();
        // using the example from the covalnet docs for this on the ethereum mainnet chain_id
        client.chain_id = "1".to_string();
        let log_events = client
            .get_log_events_by_contract(
                "0xc0da01a04c3f3e0be433606045bb7017a7323e38",
                "12115107",
                "12240004",
                None,
                None,
            )
            .await
            .expect("Should receive valid log events");
        assert!(log_events.error.error)
    }

    #[tokio::test]
    async fn test_get_log_events_by_topic_hashes() {
        let mut client = setup_klaytn_client();
        // using the example from the covalnet docs for this on the ethereum mainnet chain_id
        client.chain_id = "1".to_string();
        let log_events = client
            .get_log_events_by_topic_hashes(
                "0x804c9b842b2748a22bb64b345453a3de7ca54a6ca45ce00d415894979e22897a",
                "0x7d2768dE32b0b80b7a3454c06BdAc94A69DDc7A9",
                "12500000",
                "12500100",
                None,
                None,
            )
            .await
            .expect("Should receive valid log events");
        assert!(!log_events.error.error)
    }

    #[tokio::test]
    async fn test_get_contract_metadata() {
        let mut client = setup_klaytn_client();
        // using the example from the covalnet docs for this on the matic mainnet chain_id
        client.chain_id = "137".to_string();
        let metadata = client
            .get_all_contract_metadata(None, None)
            .await
            .expect("Should receive valid contract metadata");
        assert!(!metadata.error.error)
    }

    #[tokio::test]
    async fn test_get_all_chains() {
        let client = setup_klaytn_client();
        let chains = client
            .get_all_chains("USD", None, None)
            .await
            .expect("Should receive valid chains");
        assert!(!chains.error.error)
    }

    #[tokio::test]
    async fn test_get_all_chain_statuses() {
        let client = setup_klaytn_client();
        let chains = client
            .get_all_chain_statuses("USD", None, None)
            .await
            .expect("Should receive valid chain statuses");
        assert!(!chains.error.error)
    }
}
