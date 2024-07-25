use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Token {
    pub guid: i32,
    pub token_address: String,
    pub decimal: i16,
    pub token_name: String,
    pub collect_amount: f64,
    pub timestamp: i64,
}