use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct ApiError {
    pub error: bool,
    pub error_message: Option<String>,
    pub error_code: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct ApiPagination {
    pub has_more: bool,
    pub page_number: Option<String>,
    pub page_size: Option<i32>,
    pub total_count: Option<i32>,
}

// BALANCES
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct BalanceItem {
    pub contract_decimals: i32,
    pub contract_name: String,
    pub contract_ticker_symbol: String,
    pub contract_address: String,
    pub supports_erc: Option<Vec<String>>,
    pub logo_url: String,
    pub last_transferred_at: Option<String>,
    #[serde(alias = "type")]
    pub balance_type: String,
    pub balance: String,
    pub balance_24h: Option<String>,
    pub quote_rate: Option<f32>,
    pub quote_rate_24h: Option<f32>,
    pub quote: f32,
    pub quote_24h: Option<f32>,
    #[serde(skip_deserializing, skip_serializing)]
    nft_data: Option<Vec<()>>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct Balances {
    pub address: String,
    pub updated_at: String,
    pub next_update_at: String,
    pub quote_currency: String,
    pub chain_id: i64,
    pub items: Vec<BalanceItem>,
    #[serde(flatten)]
    pub pagination: Option<ApiPagination>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct BalancesData {
    pub data: Balances,
    #[serde(flatten)]
    pub error: ApiError,
}
// END

// TOKEN HOLDER
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct TokenHolderItem {
    pub contract_decimals: i32,
    pub contract_name: String,
    pub contract_ticket_symbol: String,
    pub contract_address: String,
    pub supports_erc: Option<Vec<String>>,
    pub logo_url: String,
    pub address: String,
    pub balance: i64,
    pub total_supply: i32,
    pub block_height: i64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct TokenHolders {
    pub updated_at: String,
    pub items: Vec<TokenHolderItem>,
    #[serde(flatten)]
    pub pagination: Option<ApiPagination>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct TokenHoldersData {
    pub data: TokenHolders,
    #[serde(flatten)]
    pub error: ApiError,
}
// END

// TRANSACTIONS
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct TransactionItem {
    pub block_signed_at: String,
    pub block_height: i32,
    pub tx_hash: String,
    pub tx_offset: i32,
    pub successful: bool,
    pub from_address: String,
    pub from_address_label: Option<String>,
    pub to_address: String,
    pub to_address_label: Option<String>,
    pub value: String,
    pub value_quote: f64,
    pub gas_offered: i64,
    pub gas_spent: i64,
    pub gas_price: i64,
    pub fees_paid: String,
    pub gas_quote: f64,
    pub gas_quote_rate: f64,
    #[serde(skip_deserializing, skip_serializing)]
    pub log_events: Vec<()>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct Transactions {
    pub address: String,
    pub updated_at: String,
    pub next_update_at: String,
    pub quote_currency: String,
    pub chain_id: i64,
    pub items: Vec<TransactionItem>,
    #[serde(flatten)]
    pub pagination: Option<ApiPagination>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct TransactionsData {
    pub data: Transactions,
    #[serde(flatten)]
    pub error: ApiError,
}
// END

// TRANSACTION
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct Transaction {
    pub updated_at: String,
    pub items: Vec<TransactionItem>,
    #[serde(flatten)]
    pub pagination: Option<ApiPagination>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct TransactionData {
    pub data: Transaction,
    #[serde(flatten)]
    pub error: ApiError,
}
// END

// HISTORICAL PORTFOLIO
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct HoldingsPrice {
    balance: String,
    quote: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct Holdings {
    timestamp: String,
    quote_rate: Option<f64>,
    open: HoldingsPrice,
    high: HoldingsPrice,
    low: HoldingsPrice,
    close: HoldingsPrice,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct HistoricalPortfolioItem {
    pub contract_decimals: i64,
    pub contract_name: String,
    pub contract_ticker_symbol: String,
    pub contract_address: String,
    pub supports_erc: Option<Vec<String>>,
    pub logo_url: String,
    pub holdings: Vec<Holdings>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct HistoricalPortfolio {
    pub address: String,
    pub updated_at: String,
    pub next_update_at: String,
    pub quote_currency: String,
    pub chain_id: i64,
    pub items: Vec<HistoricalPortfolioItem>,
    #[serde(flatten)]
    pub pagination: Option<ApiPagination>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct HistoricalPortfolioData {
    pub data: HistoricalPortfolio,
    #[serde(flatten)]
    pub error: ApiError,
}
// END
