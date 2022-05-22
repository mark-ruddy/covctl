use axum::{extract, extract::Extension, response};
use covalent_class_a::{resources, CovalentClient};
use http::StatusCode;
use log::info;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub mod data;

/// State is shared across handlers
pub struct State {
    pub db: mongodb::Client,
    pub covalent: CovalentClient,
}

#[derive(Serialize, Deserialize)]
pub struct AddrJson {
    pub addr: String,
}

#[derive(Serialize, Deserialize)]
pub struct TransactionJson {
    pub transaction: String,
}

pub async fn get_token_balances(
    extract::Json(payload): extract::Json<AddrJson>,
    Extension(state): Extension<Arc<State>>,
) -> Result<response::Json<resources::BalancesData>, StatusCode> {
    match state.covalent.get_token_balances(&payload.addr).await {
        Ok(balances) => Ok(response::Json(balances)),
        Err(e) => {
            info!("failed API request to get_token_balances: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn get_token_holders_any_bh(
    extract::Json(payload): extract::Json<AddrJson>,
    Extension(state): Extension<Arc<State>>,
) -> Result<response::Json<resources::TokenHoldersData>, StatusCode> {
    match state.covalent.get_token_holders_any_bh(&payload.addr).await {
        Ok(token_holders) => Ok(response::Json(token_holders)),
        Err(e) => {
            info!("failed API request to get_token_holders_any_bh: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn get_transactions_for_address(
    extract::Json(payload): extract::Json<AddrJson>,
    Extension(state): Extension<Arc<State>>,
) -> Result<response::Json<resources::TransactionsData>, StatusCode> {
    match state
        .covalent
        .get_transactions_for_address(&payload.addr)
        .await
    {
        Ok(transactions) => Ok(response::Json(transactions)),
        Err(e) => {
            info!("failed API request to get_transactions_for_address: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn get_transaction(
    extract::Json(payload): extract::Json<TransactionJson>,
    Extension(state): Extension<Arc<State>>,
) -> Result<response::Json<resources::TransactionData>, StatusCode> {
    match state.covalent.get_transaction(&payload.transaction).await {
        Ok(transaction) => Ok(response::Json(transaction)),
        Err(e) => {
            info!("failed API request to get_transactions: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
