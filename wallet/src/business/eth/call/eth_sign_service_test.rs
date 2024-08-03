#[cfg(test)]
mod tests {
    use ethers::abi::AbiEncode;
    use ethers::addressbook::Address;
    use crate::business::chain_service::eth_sign_service::{create_address, private_create_address};
    use crate::business::eth::call::eth_sign_service::{create_address, private_create_address};
    use crate::business::eth::model::resp::address_info::AddressInfo;
    use crate::business::model::chain::address_info::AddressInfo;

    #[test]
    fn test_create_address() {
        // 调用 create_address 方法并获取结果
        let address_info: AddressInfo = create_address();

        eprintln!("test_create_address {:?}", address_info);
        // 测试序列化
        let serialized = serde_json::to_string(&address_info).expect("Failed to serialize");
        eprintln!("test_create_address Serialized: {}", serialized);
    }

    #[test]
    fn test_address_zero() {
        // 调用 create_address 方法并获取结果
        let address_zero = Address::zero();

        // 创建一个空地址
        // 打印空地址
        // Empty address: 0x0000000000000000000000000000000000000000
        eprintln!("Empty address: {:?}", address_zero);
        eprintln!("Empty address: {:?}", serde_json::to_string(&address_zero));
        eprintln!("Empty address: {:?}", address_zero.to_string());
        // 获取完整的十六进制表示形式
        let empty_address_hex = format!("0x{}", address_zero.encode_hex());
        eprintln!("Empty address (encode_hex): {:?}", empty_address_hex);
        eprintln!("Empty address length: {:?}", empty_address_hex.len());

        // 获取完整的十六进制表示形式
        let empty_address_hex = format!("0x{}", hex::encode(address_zero.as_bytes()));
        eprintln!("Empty address (encode_hex): {}", empty_address_hex);
        eprintln!("Empty address length: {}", empty_address_hex.len());

        eprintln!("================================================================");

        let (private_key_hex, public_key_hex, address) = private_create_address();
        eprintln!(" address: {:?}", address);
        eprintln!(" address: {:?}", serde_json::to_string(&address));
        eprintln!(" address: {:?}", address.to_string());
        eprintln!(" address: {:?}", address.encode_hex());
        eprintln!(" address: {:?}", address.encode_hex().len());

    }
}