use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Balance {
    pub guid: i32,
    pub address: String,
    pub address_type: i16,
    pub token_address: String,
    pub balance: f64,
    pub lock_balance: f64,
    pub timestamp: i64,
}


