use clap::{Parser, Subcommand};
use covalent_class_a::CovalentClient;
use log::error;
use serde_json::to_string_pretty;

#[derive(Subcommand, Debug)]
enum Action {
    /// Token balances for an address
    TokenBalances(AddrPageFlag),
    /// Historicial portfolio value for an address
    HistoricalPortfolioValue(AddrPageFlag),
    /// Token Transfers given an address and the contract address
    TokenTransfers(AddrPageTokenContractFlag),
    /// Token holders at any block height for an address
    TokenHoldersAnyBh(AddrPageFlag),
    /// Changes in token holders between two block heights
    ChangesInTokenHolders(AddrPageBetweenBlocksFlag),
    /// Transactions for an address
    TransactionsForAddress(AddrPageFlag),
    /// Data on a single transaction given a transaction hash
    Transaction(TxHashPageFlag),
    /// Data on a block given a block height
    Block(BlockHeightPageFlag),
    /// Block heights given a start and end date
    BlockHeights(StartEndDatePageFlag),
    /// Log events by contract address within a start and end date
    LogEventsByContract(ContractAddrPageBetweenBlocksFlag),
    /// Log events by topic hashes
    LogEventsByTopicHashes(TopicSenderPageBetweenBlocksFlag),
    /// All contract metadata
    AllContractMetadata(Pagination),
    /// All chains
    AllChains(QuoteCurrencyPageFlag),
    /// All chain statuses
    AllChainStatuses(QuoteCurrencyPageFlag),
}

#[derive(Parser, Debug)]
struct Pagination {
    /// Number of items in a single page
    #[clap(long)]
    page_size: Option<String>,
    /// Start with items on this page
    #[clap(long)]
    page_number: Option<String>,
}

#[derive(Parser, Debug)]
struct AddrPageFlag {
    /// The wallet address
    #[clap(long)]
    addr: String,
    #[clap(flatten)]
    page: Pagination,
}

#[derive(Parser, Debug)]
struct StartEndBlockFlag {
    /// The starting block
    #[clap(long)]
    starting_block: String,
    /// The ending block
    #[clap(long)]
    ending_block: String,
}

#[derive(Parser, Debug)]
struct StartEndDatePageFlag {
    /// The start date in YYYY-MM-DD format
    #[clap(long)]
    start_date: String,
    /// The end date in YYYY-MM-DD format
    #[clap(long)]
    end_date: String,
    #[clap(flatten)]
    page: Pagination,
}

#[derive(Parser, Debug)]
struct AddrPageTokenContractFlag {
    /// The contract or token address
    #[clap(long)]
    contract_addr: String,
    #[clap(flatten)]
    addr_page: AddrPageFlag,
}

#[derive(Parser, Debug)]
struct TxHashPageFlag {
    /// The transaction hash
    #[clap(long)]
    tx_hash: String,
    #[clap(flatten)]
    page: Pagination,
}

#[derive(Parser, Debug)]
struct AddrPageBetweenBlocksFlag {
    #[clap(flatten)]
    addr_page: AddrPageFlag,
    #[clap(flatten)]
    blocks: StartEndBlockFlag,
    #[clap(flatten)]
    page: Pagination,
}

#[derive(Parser, Debug)]
struct BlockHeightPageFlag {
    /// The block height
    #[clap(long)]
    block_height: String,
    #[clap(flatten)]
    page: Pagination,
}

#[derive(Parser, Debug)]
struct ContractAddrPageBetweenBlocksFlag {
    #[clap(long)]
    contract_addr: String,
    #[clap(flatten)]
    blocks: StartEndBlockFlag,
    #[clap(flatten)]
    page: Pagination,
}

#[derive(Parser, Debug)]
struct TopicSenderPageBetweenBlocksFlag {
    /// The topic hash - comma-separated to provide multiple
    #[clap(long)]
    topic_hash: String,
    /// The senders address
    #[clap(long)]
    sender_addr: String,
    #[clap(flatten)]
    blocks: StartEndBlockFlag,
    #[clap(flatten)]
    page: Pagination,
}

#[derive(Parser, Debug)]
struct QuoteCurrencyPageFlag {
    /// The quote currency format
    #[clap(long, default_value = "USD")]
    quote_currency: String,
    #[clap(flatten)]
    page: Pagination,
}

#[derive(Parser)]
struct Args {
    #[clap(subcommand)]
    action: Action,
    /// The chain ID to query - the default is "8217" which is the Klaytn Mainnet
    #[clap(short, long, default_value = "8217")]
    chain_id: String,
    /// Your Covalent API key - if not set it will use environment variable COVALENT_API_KEY
    #[clap(short, long)]
    api_key: Option<String>,
}

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    let args = Args::parse();

    // If an API key is passed by CLI flag then create a client using that
    // Otherwise attempt to get the API key from an environment variable COVALENT_API_KEY
    let client = match args.api_key {
        Some(api_key) => match CovalentClient::new(&args.chain_id, &api_key) {
            Ok(client) => client,
            Err(e) => {
                error!(
                    "Failed to create covalent client with CLI flag API key: {}",
                    e
                );
                return;
            }
        },
        None => match CovalentClient::new_env_api_key(&args.chain_id) {
            Ok(client) => client,
            Err(e) => {
                error!(
                    "Failed to create covalent client from environment variable API key: {}",
                    e
                );
                return;
            }
        },
    };

    match args.action {
        Action::TokenBalances(flags) => {
            match client
                .get_token_balances(&flags.addr, flags.page.page_size, flags.page.page_number)
                .await
            {
                Ok(balances) => match to_string_pretty(&balances) {
                    Ok(balances) => println!("{}", balances),
                    Err(e) => error!("Failed to format balances: {}", e),
                },
                Err(e) => error!("Failed to get token balances: {}", e),
            }
        }
        Action::HistoricalPortfolioValue(flags) => {
            match client
                .get_historical_portfolio_value(
                    &flags.addr,
                    flags.page.page_size,
                    flags.page.page_number,
                )
                .await
            {
                Ok(historical_values) => match to_string_pretty(&historical_values) {
                    Ok(historical_values) => println!("{}", historical_values),
                    Err(e) => error!("Failed to format historical values: {}", e),
                },
                Err(e) => error!("Failed to get historical values: {}", e),
            }
        }
        Action::TokenTransfers(flags) => {
            match client
                .get_token_transfers(
                    &flags.addr_page.addr,
                    &flags.contract_addr,
                    flags.addr_page.page.page_size,
                    flags.addr_page.page.page_number,
                )
                .await
            {
                Ok(token_transfers) => match to_string_pretty(&token_transfers) {
                    Ok(token_transfers) => println!("{}", token_transfers),
                    Err(e) => error!("Failed to format token transfers: {}", e),
                },
                Err(e) => error!("Failed to get token transfers: {}", e),
            }
        }
        Action::TokenHoldersAnyBh(flags) => {
            match client
                .get_token_holders_any_bh(&flags.addr, flags.page.page_size, flags.page.page_number)
                .await
            {
                Ok(token_holders) => match to_string_pretty(&token_holders) {
                    Ok(balances) => println!("{}", balances),
                    Err(e) => error!("Failed to format token holders at any block height: {}", e),
                },
                Err(e) => error!("Failed to get token holders at any block height: {}", e),
            }
        }
        Action::ChangesInTokenHolders(flags) => {
            match client
                .get_changes_in_token_holders(
                    &flags.addr_page.addr,
                    &flags.blocks.starting_block,
                    &flags.blocks.ending_block,
                    flags.addr_page.page.page_size,
                    flags.addr_page.page.page_number,
                )
                .await
            {
                Ok(token_holders) => match to_string_pretty(&token_holders) {
                    Ok(balances) => println!("{}", balances),
                    Err(e) => error!("Failed to format token holders at any block height: {}", e),
                },
                Err(e) => error!("Failed to get token holders at any block height: {}", e),
            }
        }
        Action::TransactionsForAddress(flags) => {
            match client
                .get_transactions_for_address(
                    &flags.addr,
                    flags.page.page_size,
                    flags.page.page_number,
                )
                .await
            {
                Ok(transactions) => match to_string_pretty(&transactions) {
                    Ok(transactions) => println!("{}", transactions),
                    Err(e) => error!("Failed to format transactions: {}", e),
                },
                Err(e) => error!("Failed to get transactions for address: {}", e),
            }
        }
        Action::Transaction(flags) => {
            match client
                .get_transaction(&flags.tx_hash, flags.page.page_size, flags.page.page_number)
                .await
            {
                Ok(transaction) => match to_string_pretty(&transaction) {
                    Ok(transaction) => println!("{}", transaction),
                    Err(e) => error!("Failed to format transaction: {}", e),
                },
                Err(e) => error!("Failed to get transaction data: {}", e),
            }
        }
        Action::Block(flags) => {
            match client
                .get_a_block(
                    &flags.block_height,
                    flags.page.page_size,
                    flags.page.page_number,
                )
                .await
            {
                Ok(block) => match to_string_pretty(&block) {
                    Ok(block) => println!("{}", block),
                    Err(e) => error!("Failed to format block: {}", e),
                },
                Err(e) => error!("Failed to get block data: {}", e),
            }
        }
        Action::BlockHeights(flags) => {
            match client
                .get_block_heights(
                    &flags.start_date,
                    &flags.end_date,
                    flags.page.page_size,
                    flags.page.page_number,
                )
                .await
            {
                Ok(heights) => match to_string_pretty(&heights) {
                    Ok(heights) => println!("{}", heights),
                    Err(e) => error!("Failed to format block heights: {}", e),
                },
                Err(e) => error!("Failed to get block heights data: {}", e),
            }
        }
        Action::LogEventsByContract(flags) => {
            match client
                .get_log_events_by_contract(
                    &flags.contract_addr,
                    &flags.blocks.starting_block,
                    &flags.blocks.ending_block,
                    flags.page.page_size,
                    flags.page.page_number,
                )
                .await
            {
                Ok(log_events) => match to_string_pretty(&log_events) {
                    Ok(log_events) => println!("{}", log_events),
                    Err(e) => error!("Failed to format log events: {}", e),
                },
                Err(e) => error!("Failed to get log event data: {}", e),
            }
        }
        Action::LogEventsByTopicHashes(flags) => {
            match client
                .get_log_events_by_topic_hashes(
                    &flags.topic_hash,
                    &flags.sender_addr,
                    &flags.blocks.starting_block,
                    &flags.blocks.ending_block,
                    flags.page.page_size,
                    flags.page.page_number,
                )
                .await
            {
                Ok(log_events) => match to_string_pretty(&log_events) {
                    Ok(log_events) => println!("{}", log_events),
                    Err(e) => error!("Failed to format log events: {}", e),
                },
                Err(e) => error!("Failed to get log event data: {}", e),
            }
        }
        Action::AllContractMetadata(flags) => {
            match client
                .get_all_contract_metadata(flags.page_size, flags.page_number)
                .await
            {
                Ok(metadata) => match to_string_pretty(&metadata) {
                    Ok(metadata) => println!("{}", metadata),
                    Err(e) => error!("Failed to format metadata: {}", e),
                },
                Err(e) => error!("Failed to get log metadata: {}", e),
            }
        }
        Action::AllChains(flags) => {
            match client
                .get_all_chains(
                    &flags.quote_currency,
                    flags.page.page_size,
                    flags.page.page_number,
                )
                .await
            {
                Ok(chains) => match to_string_pretty(&chains) {
                    Ok(chains) => println!("{}", chains),
                    Err(e) => error!("Failed to format chains: {}", e),
                },
                Err(e) => error!("Failed to get chains data: {}", e),
            }
        }
        Action::AllChainStatuses(flags) => {
            match client
                .get_all_chains(
                    &flags.quote_currency,
                    flags.page.page_size,
                    flags.page.page_number,
                )
                .await
            {
                Ok(chain_statuses) => match to_string_pretty(&chain_statuses) {
                    Ok(chain_statuses) => println!("{}", chain_statuses),
                    Err(e) => error!("Failed to format chain statuses: {}", e),
                },
                Err(e) => error!("Failed to get chain statuses data: {}", e),
            }
        }
    }
}
