#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddressTypeEnum {
    HotWallet,
    ColdWallet,
    CollectionWallet,
    UserAddress,
}

impl AddressTypeEnum {
    // 获取地址类型的描述
    pub fn chinese(&self) -> &'static str {
        match self {
            AddressTypeEnum::HotWallet => "热钱包地址",
            AddressTypeEnum::ColdWallet => "冷钱包地址",
            AddressTypeEnum::CollectionWallet => "归集钱包地址",
            AddressTypeEnum::UserAddress => "用户地址",
        }
    }

    // 获取地址类型的名称
    pub fn english(&self) -> &'static str {
        match self {
            AddressTypeEnum::HotWallet => "HotWallet",
            AddressTypeEnum::ColdWallet => "ColdWallet",
            AddressTypeEnum::CollectionWallet => "CollectionWallet",
            AddressTypeEnum::UserAddress => "UserAddress",
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
    use crate::business::eth::model::enums::address_type_enum::AddressTypeEnum;

    #[test]
    fn test_address_type_from_string() {
        let address_type = AddressTypeEnum::HotWallet;
        println!("Address Type: {:?}", address_type);
        println!("Name: {}", address_type.english());
        println!("Description: {}", address_type.chinese());
    }
}
