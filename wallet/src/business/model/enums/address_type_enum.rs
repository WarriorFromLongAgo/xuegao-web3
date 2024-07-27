
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddressType {
    HotWallet,
    ColdWallet,
    AggregationWallet,
    UserAddress,
}

impl AddressType {
    // 获取地址类型的描述
    pub fn description(&self) -> &'static str {
        match self {
            AddressType::HotWallet => "热钱包地址",
            AddressType::ColdWallet => "冷钱包地址",
            AddressType::AggregationWallet => "归集钱包地址",
            AddressType::UserAddress => "用户地址",
        }
    }

    // 获取地址类型的名称
    pub fn name(&self) -> &'static str {
        match self {
            AddressType::HotWallet => "HotWallet",
            AddressType::ColdWallet => "ColdWallet",
            AddressType::AggregationWallet => "AggregationWallet",
            AddressType::UserAddress => "UserAddress",
        }
    }

    // // 将 AddressType 转换为数字值
    // pub fn as_u8(&self) -> u8 {
    //     match self {
    //         AddressType::HotWallet => 1,
    //         AddressType::ColdWallet => 2,
    //         AddressType::AggregationWallet => 3,
    //         AddressType::UserAddress => 0,
    //     }
    // }

    // 从数字值获取 AddressType
    // pub fn from_u8(value: u8) -> Option<Self> {
    //     match value {
    //         1 => Some(AddressType::HotWallet),
    //         2 => Some(AddressType::ColdWallet),
    //         3 => Some(AddressType::AggregationWallet),
    //         0 => Some(AddressType::UserAddress),
    //         _ => None,
    //     }
    // }
}
#[cfg(test)]
mod tests {
    use crate::business::model::enums::address_type_enum::AddressType;

    #[test]
    fn test_address_type_from_string() {
        let address_type = AddressType::HotWallet;
        println!("Address Type: {:?}", address_type);
        println!("Name: {}", address_type.name());
        println!("Description: {}", address_type.description());
    }
}
