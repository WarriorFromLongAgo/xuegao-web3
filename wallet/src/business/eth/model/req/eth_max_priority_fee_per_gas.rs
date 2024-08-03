use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EthMaxPriorityFeePerGas {
    pub max_priority_fee_per_gas_hex: String,
    pub max_priority_fee_per_gas_dec: String,
}



