use clap::{Parser, Subcommand};
use covalent_class_a::CovalentClient;
use log::error;
use serde_json::to_string_pretty;

#[derive(Subcommand, Debug)]
enum Action {
    TokenBalances(AddrPageFlag),
    HistoricalPortfolioValue(AddrPageFlag),
    TokenHoldersAnyBh(AddrPageFlag),
    TransactionsForAddress(AddrPageFlag),
    Transaction(TxHashPageFlag),
}

#[derive(Parser, Debug)]
struct Pagination {
    #[clap(long)]
    page_size: Option<String>,
    #[clap(long)]
    page_number: Option<String>,
}

#[derive(Parser, Debug)]
struct AddrPageFlag {
    #[clap(short, long)]
    addr: String,
    #[clap(flatten)]
    page: Pagination,
}

#[derive(Parser, Debug)]
struct TxHashPageFlag {
    #[clap(short, long)]
    tx_hash: String,
    #[clap(flatten)]
    page: Pagination,
}

#[derive(Parser)]
struct Args {
    #[clap(subcommand)]
    action: Action,
}

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    let args = Args::parse();

    let client = match CovalentClient::new_env_api_key("8217") {
        Ok(client) => client,
        Err(e) => {
            error!("Failed to create covalent client: {}", e);
            return;
        }
    };

    match args.action {
        Action::TokenBalances(flags) => {
            match client.get_token_balances(&flags.addr, None, None).await {
                Ok(balances) => match to_string_pretty(&balances) {
                    Ok(balances) => println!("{}", balances),
                    Err(e) => error!("Failed to format balances: {}", e),
                },
                Err(e) => {
                    error!("Failed to get token balances: {}", e);
                }
            }
        }
        _ => error!("Unknown action"),
    }
}
