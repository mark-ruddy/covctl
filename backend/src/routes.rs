/*use axum::{extract::Extension, extract::Json};
use http::StatusCode;
use log::{error, info};*/
use std::error::Error;

pub mod data;

/// State is shared across handlers
pub struct State {
    pub db: mongodb::Client,
}

pub async fn index() {}
