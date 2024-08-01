
#[cfg(test)]
mod address_test {
    use ethers::abi::AbiEncode;
    use ethers::types::Address;

    #[test]
    fn test_create_address() {
        // 创建一个空地址
        let empty_address = Address::zero();
        // 打印空地址
        // Empty address: 0x0000000000000000000000000000000000000000
        eprintln!("Empty address: {:?}", empty_address);
        eprintln!("Empty address: {:?}", empty_address.to_string());
        eprintln!("Empty address: {:?}", empty_address.encode_hex());
        eprintln!("Empty address: {:?}", empty_address.encode_hex().len());
    }

}