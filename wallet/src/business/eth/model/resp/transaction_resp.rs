use serde::{Deserialize, Serialize};

use crate::business::eth::model::enums::transaction_type_enum::TransactionTypeEnum;
use crate::business::wallet::util::hex_dec_util::hex_to_dec;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Transaction {
    #[serde(rename = "blockHash")]
    pub block_hash: String,
    #[serde(rename = "blockNumber")]
    pub block_number: String,
    #[serde(skip)]
    pub block_number_dec: u64,
    #[serde(skip)]
    pub block_number_hex: String,
    pub from: String,
    pub gas: String,
    #[serde(skip)]
    pub gas_dec: String,
    #[serde(skip)]
    pub gas_hex: String,
    #[serde(rename = "gasPrice")]
    pub gas_price: String,
    #[serde(skip)]
    pub gas_price_dec: u64,
    #[serde(skip)]
    pub gas_price_hex: String,
    #[serde(rename = "maxFeePerGas")]
    pub max_fee_per_gas: Option<String>,
    #[serde(skip)]
    pub max_fee_per_gas_dec: u64,
    #[serde(skip)]
    pub max_fee_per_gas_hex: String,
    #[serde(rename = "maxPriorityFeePerGas")]
    pub max_priority_fee_per_gas: Option<String>,
    #[serde(skip)]
    pub max_priority_fee_per_gas_dec: u64,
    #[serde(skip)]
    pub max_priority_fee_per_gas_hex: String,
    pub hash: String,
    pub input: String,
    // pub nonce: String,
    // to 地址为空，是创建合约事件
    pub to: Option<String>,
    // #[serde(rename = "transactionIndex")]
    // pub transaction_index: String,
    // pub value: String,
    #[serde(rename = "type")]
    pub tx_type: String,
    #[serde(skip)]
    pub tx_type_dec: u64,
    #[serde(skip)]
    pub tx_type_hex: String,
    // TransactionTypeEnum
    #[serde(skip)]
    pub tx_type_enum: TransactionTypeEnum,

    #[serde(rename = "chainId")]
    pub chain_id: Option<String>,
    #[serde(skip)]
    pub chain_id_dec: u64,
    #[serde(skip)]
    pub chain_id_hex: String,
    pub v: Option<String>,
    pub r: Option<String>,
    pub s: Option<String>,
    // #[serde(rename = "yParity")]
    // pub y_parity: String,
}

impl Transaction {
    pub fn deal_dec_hex(&mut self) {
        let temp_block_number = hex_to_dec(self.block_number.clone());
        self.block_number_dec = temp_block_number.unwrap();
        self.block_number_hex = self.block_number.clone();

        let temp_gas_price = hex_to_dec(self.gas_price.clone());
        self.gas_price_dec = temp_gas_price.unwrap();
        self.gas_price_hex = self.gas_price.clone();

        if let Some(max_fee_per_gas) = self.max_fee_per_gas.clone() {
            let temp_max_fee_per_gas = hex_to_dec(max_fee_per_gas.clone());
            self.max_fee_per_gas_dec = temp_max_fee_per_gas.unwrap();
            self.max_fee_per_gas_hex = max_fee_per_gas.clone();
        }

        if let Some(max_priority_fee_per_gas) = self.max_priority_fee_per_gas.clone() {
            let temp_max_priority_fee_per_gas = hex_to_dec(max_priority_fee_per_gas.clone());
            self.max_priority_fee_per_gas_dec = temp_max_priority_fee_per_gas.unwrap();
            self.max_priority_fee_per_gas_hex = max_priority_fee_per_gas.clone();
        }

        let temp_tx_type = hex_to_dec(self.tx_type.clone());
        self.tx_type_dec = temp_tx_type.unwrap();
        self.tx_type_hex = self.tx_type.clone();
        self.tx_type_enum = TransactionTypeEnum::hex_2_enum(self.tx_type.clone().as_str());

        if let Some(chain_id) = self.chain_id.clone() {
            let temp_chain_id = hex_to_dec(chain_id.clone());
            self.chain_id_dec = temp_chain_id.unwrap();
            self.chain_id_hex = chain_id.clone();
        }
    }
}
