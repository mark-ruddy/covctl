use covalent_class_a::CovalentClient;
use log::{error, info};

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    let mut covalent_client = match CovalentClient::new("8217", "ckey_44f70254f9364489b8fddce0549")
    {
        Ok(client) => client,
        Err(e) => {
            error!("Failed to create covalent client: {}", e);
            return;
        }
    };
    /*
    match covalent_client
        .get_token_balances("0xf4024faad5fafd0755875e3161524c9c4e1a1111")
        .await
    {
        Ok(bal) => println!("addr: {}", bal.data.address), //  info!("address used: {}", bal.address),
        Err(e) => {
            error!("failure: {}", e);
        }
    }
    match covalent_client
        .get_transactions_for_address("0xf4024faad5fafd0755875e3161524c9c4e1a1111")
        .await
    {
        Ok(bal) => println!("first transaction hash: {}", bal.data.items[0].tx_hash), //  info!("address used: {}", bal.address),
        Err(e) => {
            error!("failure: {}", e);
        }
    }
    match covalent_client
        .get_transaction("0x269fad968de5baf8d324b64d0a19df72ccfc762b33e1760729633f4946e0c863")
        .await
    {
        Ok(bal) => println!("get transaciton hash: {}", bal.data.items[0].tx_hash), //  info!("address used: {}", bal.address),
        Err(e) => {
            error!("failure: {}", e);
        }
    }*/
    /*match covalent_client
        .get_historical_portfolio_value(
            "0x269fad968de5baf8d324b64d0a19df72ccfc762b33e1760729633f4946e0c863",
            None,
            None,
        )
        .await
    {
        Ok(bal) => println!("get ff: {}", bal.data.address), //  info!("address used: {}", bal.address),
        Err(e) => {
            error!("failure: {}", e);
        }
    }*/
    /*match covalent_client
        .get_log_events_by_contract(
            "0x5c74070fdea071359b86082bd9f9b3deaafbe32b",
            "10000000",
            "91321208",
            None,
            None,
        )
        .await
    {
        Ok(bal) => println!("get ff: {}", bal.data.updated_at), //  info!("address used: {}", bal.address),
        Err(e) => {
            error!("failure: {}", e);
        }
    }*/
    covalent_client.chain_id = "137".to_string();
    match covalent_client.get_all_contract_metadata(None, None).await {
        Ok(bal) => println!("get ff: {}", bal.data.updated_at), //  info!("address used: {}", bal.address),
        Err(e) => {
            error!("failure: {}", e);
        }
    }

    /*match covalent_client
        .get_transactions_for_address(
            "0x269fad968de5baf8d324b64d0a19df72ccfc762b33e1760729633f4946e0c863",
            None,
            None,
        )
        .await
    {
        Ok(bal) => println!("get ff: {}", bal.data.address), //  info!("address used: {}", bal.address),
        Err(e) => {
            error!("failure: {}", e);
        }
    }*/
}
