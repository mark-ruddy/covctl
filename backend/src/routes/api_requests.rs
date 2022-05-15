use covalent_class_a::Balance;
use log::info;
use reqwest::Response;
use serde::Deserialize;
use std::env;
use std::error::Error;

async fn make_request(url: &str) -> Result<Response, Box<dyn Error>> {
    info!("Sending API request to: {}", url);
    let resp = reqwest::get(url).await?;
    Ok(resp)
}

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

struct CovalentClient {
    base_url: String,
    api_key: String,
    chain_id: i32,
}

impl CovalentClient {
    fn new_klaytn() -> Result<CovalentClient, Box<dyn Error>> {
        Ok(CovalentClient {
            base_url: "https://api.covalenthq.com/v1/1".to_string(),
            api_key: get_env_api_key()?,
            chain_id: 8127,
        })
    }
}

/*pub fn get_token_balances(chain_id: i32, address: &str) -> Result<Balance, Box<dyn Error>> {
    let resp = make_request(format!(""));
    Ok(())
}*/
