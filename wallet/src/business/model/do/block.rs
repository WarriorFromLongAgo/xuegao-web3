use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Block {
    pub hash: String,
    pub parent_hash: String,
    pub number: i64,
    pub timestamp: i64,
    pub rlp_bytes: String,
}