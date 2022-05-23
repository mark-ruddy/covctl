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
pub struct WalletBalanceItem {
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
    pub quote_rate: Option<f64>,
    pub quote_rate_24h: Option<f64>,
    pub quote: f64,
    pub quote_24h: Option<f64>,
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
    pub items: Vec<WalletBalanceItem>,
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
pub struct LogDecodedParams {
    name: String,
    #[serde(alias = "type")]
    pub param_type: String,
    pub indexed: bool,
    pub decoded: bool,
    // value is usually a String but can sometimes be a Vector(JS sequence/list)
    // for now avoiding using serde on it because of the type changing
    #[serde(skip_serializing, skip_deserializing)]
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct LogDecoded {
    pub name: String,
    pub signature: String,
    pub params: Option<Vec<LogDecodedParams>>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct LogEventItem {
    pub block_signed_at: String,
    pub block_height: i64,
    pub tx_offset: i64,
    pub log_offset: i64,
    pub tx_hash: String,
    pub raw_log_topics: Option<Vec<String>>,
    pub sender_contract_decimals: i32,
    pub sender_name: Option<String>,
    pub sender_contract_ticker_symbol: Option<String>,
    pub sender_address: String,
    pub sender_address_label: Option<String>,
    pub sender_logo_url: Option<String>,
    pub raw_log_data: Option<String>,
    pub decoded: Option<LogDecoded>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct BaseTransaction {
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
    pub fees_paid: Option<String>,
    pub gas_quote: f64,
    pub gas_quote_rate: f64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct BlockTransactionWithLogEvents {
    #[serde(flatten)]
    pub transaction: BaseTransaction,
    pub log_events: Option<Vec<LogEventItem>>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct Transactions {
    pub address: String,
    pub updated_at: String,
    pub next_update_at: String,
    pub quote_currency: String,
    pub chain_id: i64,
    pub items: Vec<BlockTransactionWithLogEvents>,
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
    pub items: Vec<BlockTransactionWithLogEvents>,
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
    pub contract_decimals: i32,
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

// ERC20 TOKEN TRANSFERS
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct MethodCallsForTransfers {
    pub sender_address: String,
    pub method: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct TokenTransferItem {
    pub block_signed_at: String,
    pub tx_hash: String,
    pub from_address: String,
    pub from_address_label: Option<String>,
    pub to_address: String,
    pub to_address_label: Option<String>,
    pub contract_decimals: i32,
    pub contract_name: String,
    pub contract_ticker_symbol: String,
    pub contract_address: String,
    pub logo_url: String,
    pub transfer_type: String,
    pub delta: f64,
    pub balance: Option<f64>,
    pub quote_rate: Option<f64>,
    pub delta_quote: Option<f64>,
    pub balance_quote: Option<f64>,
    pub method_calls: Option<Vec<MethodCallsForTransfers>>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct BlockTransactionWithContractTransfers {
    #[serde(flatten)]
    pub transaction: BaseTransaction,
    pub transfers: Vec<TokenTransferItem>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct TokenTransfers {
    pub address: String,
    pub updated_at: String,
    pub next_update_at: String,
    pub quote_currency: String,
    pub chain_id: i64,
    pub items: Vec<BlockTransactionWithContractTransfers>,
    #[serde(flatten)]
    pub pagination: Option<ApiPagination>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct TokenTransfersData {
    pub data: TokenTransfers,
    #[serde(flatten)]
    pub error: ApiError,
}
// END

// CHANGES IN TOKEN HOLDERS
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct ChangesInTokenHoldersItem {
    pub token_holder: String,
    pub prev_balance: String,
    pub prev_block_height: i64,
    pub next_balance: String,
    pub next_block_height: i64,
    pub diff: i64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct ChangesInTokenHolders {
    pub updated_at: String,
    pub items: Vec<ChangesInTokenHoldersItem>,
    #[serde(flatten)]
    pub pagination: Option<ApiPagination>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct ChangesInTokenHoldersData {
    pub data: ChangesInTokenHolders,
    #[serde(flatten)]
    pub error: ApiError,
}
// END

// GET A BLOCK
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct BlockItem {
    signed_at: String,
    height: i32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct Block {
    updated_at: String,
    pub items: Vec<BlockItem>,
    #[serde(flatten)]
    pub pagination: Option<ApiPagination>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct BlockData {
    pub data: Block,
    #[serde(flatten)]
    pub error: ApiError,
}
// END

// LOG EVENTS GENERIC
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct LogEventsGeneric {
    pub updated_at: String,
    pub items: Vec<LogEventItem>,
    #[serde(flatten)]
    pub pagination: Option<ApiPagination>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct LogEventsGenericData {
    pub data: Option<LogEventsGeneric>,
    #[serde(flatten)]
    pub error: ApiError,
}
// END

// CONTRACT METADATA
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct ContractMetadataItem {
    pub contract_decimals: i32,
    pub contract_name: Option<String>,
    pub contract_ticker_symbol: Option<String>,
    pub contract_address: String,
    pub supports_erc: Option<Vec<String>>,
    pub logo_url: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct ContractMetadata {
    pub updated_at: String,
    // NOTE: maybe slight bug in unified API? will report this in my submission
    // items is returned inside a doubled up list like [[ ... ]]
    // requires 2 Vec<Vec<>> to deserialize
    pub items: Vec<Vec<ContractMetadataItem>>,
    #[serde(flatten)]
    pub pagination: Option<ApiPagination>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct ContractMetadataData {
    pub data: ContractMetadata,
    #[serde(flatten)]
    pub error: ApiError,
}
// END

// ALL CHAINS
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct GenericChainInfoDisplay {
    pub name: String,
    pub chain_id: String,
    pub is_testnet: bool,
    pub db_schema_name: String,
    pub label: String,
    pub logo_url: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct AllChain {
    pub updated_at: String,
    pub items: Vec<GenericChainInfoDisplay>,
    #[serde(flatten)]
    pub pagination: Option<ApiPagination>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct AllChainData {
    pub data: AllChain,
    #[serde(flatten)]
    pub error: ApiError,
}
// END

// All CHAINS STATUSES
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct GenericChainInfoStatusDisplay {
    pub name: String,
    pub chain_id: String,
    pub is_testnet: bool,
    pub logo_url: String,
    pub synced_block_height: i32,
    pub synced_blocked_signed_at: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct AllChainStatuses {
    pub updated_at: String,
    pub items: Vec<GenericChainInfoStatusDisplay>,
    #[serde(flatten)]
    pub pagination: Option<ApiPagination>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct AllChainStatusesData {
    pub data: AllChainStatuses,
    #[serde(flatten)]
    pub error: ApiError,
}
// END
