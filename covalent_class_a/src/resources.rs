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
    pub chain_id: i64,
    #[serde(with = "serde_with::json::nested")]
    pub items: Vec<BalanceItem>,
}

#[derive(Deserialize, Debug)]
pub struct TokenHolderItem {
    pub contract_decimals: i32,
    pub contract_name: String,
    pub contract_ticket_symbol: String,
    pub contract_address: String,
    #[serde(skip_deserializing)]
    pub supports_erc: Vec<()>,
    pub logo_url: String,
    pub address: String,
    pub balance: i64,
    pub total_supply: i32,
    pub block_height: i64,
}

#[derive(Deserialize, Debug)]
pub struct TokenHolder {
    pub updated_at: String,
    #[serde(with = "serde_with::json::nested")]
    pub items: Vec<TokenHolderItem>,
}

#[derive(Deserialize, Debug)]
pub struct TransactionItem {
    pub block_signed_at: String,
    pub block_height: i32,
    pub tx_hash: String,
    pub tx_offset: i32,
    pub successful: bool,
    pub from_address: String,
    pub from_address_label: String,
    pub to_address: String,
    pub to_address_label: String,
    pub value: i64,
    pub value_quote: i64,
    pub gas_offered: i64,
    pub gas_spent: i64,
    pub gas_price: i64,
    pub fees_paid: i64,
    pub gas_quote: i64,
    pub gas_quote_rate: i64,
    #[serde(skip_deserializing)]
    pub log_events: Vec<()>,
}

#[derive(Deserialize, Debug)]
pub struct Transaction {
    pub address: String,
    pub updated_at: String,
    pub next_update_at: String,
    pub quote_currency: String,
    pub chain_id: i64,
    #[serde(with = "serde_with::json::nested")]
    pub items: Vec<TransactionItem>,
}
