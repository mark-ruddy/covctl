use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct BalanceItem {
    pub contract_decimals: i32,
    pub contract_name: String,
    pub contract_ticker_symbol: String,
    pub contact_address: String,
    pub supports_erc: Vec<String>,
    pub logo_url: String,
    pub last_transferred_at: String,
    #[serde(alias = "type")]
    pub balance_type: String,
    pub balance: String,
    pub balance_24h: String,
    pub quote_rate: f32,
    pub quote: f32,
}

#[derive(Deserialize, Debug)]
pub struct Balance {
    pub address: String,
    pub updated_at: String,
    pub next_update_at: String,
    pub quote_currency: String,
    pub chain_id: i32,
    #[serde(with = "serde_with::json::nested")]
    pub item: BalanceItem,
}
