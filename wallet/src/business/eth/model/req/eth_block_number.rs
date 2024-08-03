use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EthBlockNumber {
    pub block_number_hex: String,
    pub block_number_dec: String,
}



