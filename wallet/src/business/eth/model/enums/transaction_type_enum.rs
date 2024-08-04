use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq,Clone)]
pub enum TransactionTypeEnum {
    Legacy,
    AccessList,
    DynamicFee,
    Blob,
}

impl Default for TransactionTypeEnum {
    fn default() -> Self {
        TransactionTypeEnum::Legacy
    }
}

impl TransactionTypeEnum {
    pub fn hex_2_enum(hex: &str) -> Self {
        match hex {
            "0x0" => TransactionTypeEnum::Legacy,
            "0x1" => TransactionTypeEnum::AccessList,
            "0x2" => TransactionTypeEnum::DynamicFee,
            "0x3" => TransactionTypeEnum::Blob,
            _ => panic!("TransactionTypeEnum不存在 {}", hex),
        }
    }

    pub fn enum_to_hex(&self) -> &str {
        match self {
            TransactionTypeEnum::Legacy => "0x0",
            TransactionTypeEnum::AccessList => "0x1",
            TransactionTypeEnum::DynamicFee => "0x2",
            TransactionTypeEnum::Blob => "0x3"
        }
    }

    pub fn dec_2_enum(dev: u8) -> Self {
        match dev {
            0 => TransactionTypeEnum::Legacy,
            1 => TransactionTypeEnum::AccessList,
            2 => TransactionTypeEnum::DynamicFee,
            3 => TransactionTypeEnum::Blob,
            _ => panic!("TransactionTypeEnum不存在 {}", dev),
        }
    }

    pub fn enum_to_dec(&self) -> u8 {
        match self {
            TransactionTypeEnum::Legacy => 0,
            TransactionTypeEnum::AccessList => 1,
            TransactionTypeEnum::DynamicFee => 2,
            TransactionTypeEnum::Blob => 3
        }
    }

    pub fn english(&self) -> &str {
        match self {
            TransactionTypeEnum::Legacy => "Legacy",
            TransactionTypeEnum::AccessList => "AccessList",
            TransactionTypeEnum::DynamicFee => "DynamicFee",
            TransactionTypeEnum::Blob => "Blob",
        }
    }

    pub fn enum_to_eip(&self) -> &str {
        match self {
            TransactionTypeEnum::Legacy => "Legacy",
            TransactionTypeEnum::AccessList => "EIP2930",
            TransactionTypeEnum::DynamicFee => "EIP1559",
            TransactionTypeEnum::Blob => "EIP4844",
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_address_type_from_string() {

    }
}

