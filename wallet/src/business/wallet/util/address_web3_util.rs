use ethers::types::Address;

pub fn is_zero_address(address: &str) -> bool {
    // 将输入地址转换为标准 20 字节长度
    let normalized_address = if address.len() < 42 {
        format!("0x{:0>40}", &address[2..])
    } else {
        address.to_string()
    };
    let parsed_address: Address = normalized_address.parse().unwrap();
    parsed_address == Address::zero()
}

#[test]
fn test_is_zero_address() {
    // 示例地址
    let addr1 = "0x0000";
    let addr2 = "0x0000000000";
    let addr3 = "0x1234567890abcdef1234567890abcdef12345678";

    println!("Is addr1 zero address? {}", is_zero_address(addr1)); // true
    println!("Is addr2 zero address? {}", is_zero_address(addr2)); // false
    println!("Is addr3 zero address? {}", is_zero_address(addr3)); // false
}