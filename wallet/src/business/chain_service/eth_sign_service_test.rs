#[cfg(test)]
mod tests {
    use crate::business::chain_service::eth_sign_service::{create_address, AddressInfo};

    #[test]
    fn test_create_address() {
        // 调用 create_address 方法并获取结果
        let address_info: AddressInfo = create_address();

        eprintln!("test_create_address {:?}", address_info);
        // 测试序列化
        let serialized = serde_json::to_string(&address_info).expect("Failed to serialize");
        eprintln!("test_create_address Serialized: {}", serialized);
    }
}