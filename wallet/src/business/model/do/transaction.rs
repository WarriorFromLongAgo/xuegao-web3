use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Transaction {
    pub guid: i32,
    pub block_hash: String,
    pub block_number: i64,
    pub hash: String,
    pub from_address: String,
    pub to_address: String,
    pub token_address: String,
    pub fee: f64,
    pub amount: f64,
    pub status: i16,
    pub transaction_index: i64,
    pub tx_type: i16,
    pub timestamp: i64,
}

