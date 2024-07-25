use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Address {
    pub guid: i32,
    pub user_uid: String,
    pub address: String,
    pub address_type: i16,
    pub private_key: String,
    pub public_key: String,
    pub timestamp: i64,
}