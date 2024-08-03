use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressInfo {
    pub private_key: String,
    pub public_key: String,
    pub address: String,
}

