/*use axum::{extract::Extension, extract::Json};
use http::StatusCode;
use log::{error, info};*/

pub mod data;

pub struct State {
    pub db: mongodb::Client,
}

pub async fn index() {}

pub async fn sample() {}
