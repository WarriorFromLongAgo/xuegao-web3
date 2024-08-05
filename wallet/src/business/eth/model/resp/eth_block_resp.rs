use serde::{Deserialize, Serialize};

use crate::business::eth::model::resp::transaction_resp::Transaction;
use crate::business::wallet::util::hex_dec_util::hex_to_dec;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EthBlock {
    #[serde(rename = "baseFeePerGas")]
    pub base_fee_per_gas: String,
    #[serde(skip)]
    pub base_fee_per_gas_dec: u64,
    #[serde(skip)]
    pub base_fee_per_gas_hex: String,
    #[serde(rename = "blobGasUsed")]
    pub blob_gas_used: String,
    #[serde(skip)]
    pub blob_gas_used_dec: u64,
    #[serde(skip)]
    pub blob_gas_used_hex: String,
    // pub difficulty: String,
    // pub excess_blob_gas: String,
    // pub extra_data: String,
    #[serde(rename = "gasLimit")]
    pub gas_limit: String,
    #[serde(skip)]
    pub gas_limit_dec: u64,
    #[serde(skip)]
    pub gas_limit_hex: String,

    #[serde(rename = "gasUsed")]
    pub gas_used: String,
    #[serde(skip)]
    pub gas_used_dec: u64,
    #[serde(skip)]
    pub gas_used_hex: String,


    #[serde(rename = "hash")]
    pub hash: String,
    #[serde(rename = "number")]
    pub number: String,
    #[serde(skip)]
    pub number_dec: u64,
    #[serde(skip)]
    pub number_hex: String,

    #[serde(rename = "parentHash")]
    pub parent_hash: String,
    #[serde(rename = "timestamp")]
    pub timestamp: String,
    #[serde(skip)]
    pub timestamp_dec: u64,
    #[serde(skip)]
    pub timestamp_hex: String,

    pub transactions: Vec<Transaction>,
}

impl EthBlock {
    pub fn deal_dec_hex(&mut self) {
        let temp_base_fee_per_gas = hex_to_dec(self.base_fee_per_gas.clone());
        self.base_fee_per_gas_dec = temp_base_fee_per_gas.unwrap();
        self.base_fee_per_gas_hex = self.base_fee_per_gas.clone();

        let temp_blob_gas_used = hex_to_dec(self.blob_gas_used.clone());
        self.blob_gas_used_dec = temp_blob_gas_used.unwrap();
        self.blob_gas_used_hex = self.blob_gas_used.clone();

        let temp_gas_limit = hex_to_dec(self.gas_limit.clone());
        self.gas_limit_dec = temp_gas_limit.unwrap();
        self.gas_limit_hex = self.gas_limit.clone();

        let temp_gas_used = hex_to_dec(self.gas_used.clone());
        self.gas_used_dec = temp_gas_used.unwrap();
        self.gas_used_hex = self.gas_used.clone();

        let temp_number = hex_to_dec(self.number.clone());
        self.number_dec = temp_number.unwrap();
        self.number_hex = self.number.clone();

        let temp_number = hex_to_dec(self.number.clone());
        self.number_dec = temp_number.unwrap();
        self.number_hex = self.number.clone();

        let temp_timestamp = hex_to_dec(self.timestamp.clone());
        self.timestamp_dec = temp_timestamp.unwrap();
        self.timestamp_hex = self.timestamp.clone();

        for temp_tx in &mut self.transactions {
            temp_tx.deal_dec_hex();
        }
    }

}