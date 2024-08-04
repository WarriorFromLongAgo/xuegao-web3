use crate::config::constant::chain;

pub fn input_data_2_method_id(data: String) -> String {
    // MethodID 是前 4 字节（8 个十六进制字符）
    data.get(0..10).unwrap_or_default().to_string()
}

pub fn input_data_2_to_address(data: String) -> String {
    // 目标地址在数据中的位置是从第 10 字符开始，长度为 32 字节（64 个十六进制字符）
    // 目标地址在数据中的位置是从第 10 字符开始，长度为 32 字节（64 个十六进制字符）
    let address_hex = &data[10..74];
    eprintln!("[xuegao-web3][token_util][input_data_2_to_address][address_hex={}]", address_hex);
    let trimmed_address_hex = address_hex.trim_start_matches('0');
    eprintln!("[xuegao-web3][token_util][input_data_2_to_address][trimmed_address_hex={}]", trimmed_address_hex);
    format!("{}{}", chain::ADDRESS_STR_0X, trimmed_address_hex)
}

pub fn input_data_2_amount(data: String) -> u128 {
    // 目标地址在数据中的位置是从第 10 字符开始，长度为 32 字节（64 个十六进制字符）
    let value_hex = &data[74..];
    eprintln!("[xuegao-web3][token_util][input_data_2_amount][value_hex={}]", value_hex);
    u128::from_str_radix(value_hex, 16).unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use crate::business::wallet::util::token_util::{input_data_2_amount, input_data_2_method_id, input_data_2_to_address};

    #[test]
    fn test_dec_to_hex() {
        // 示例数据
        let input_data = "0xa9059cbb000000000000000000000000fc99d43446007e2d621ad6c04610b8387bef02a400000000000000000000000000000000000000000000000000000000014fb180";

        // 提取 MethodID、目标地址和转账数量
        let method_id = input_data_2_method_id(input_data.to_string());
        let to_address = input_data_2_to_address(input_data.to_string());
        let value = input_data_2_amount(input_data.to_string());

        // 打印结果
        println!("MethodID: {}", method_id);
        println!("To Address: {:?}", to_address);
        println!("Value: {}", value);
    }
}
