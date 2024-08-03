use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EthGasPrice {
    pub gas_price_hex: String,
    pub gas_price_dec: String,
}



