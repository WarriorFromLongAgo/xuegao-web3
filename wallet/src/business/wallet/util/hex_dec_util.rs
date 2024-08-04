pub fn hex_to_dec(hex_str: String) -> Result<u64, std::num::ParseIntError> {
    let cleaned_hex = hex_str.trim_start_matches("0x");
    u64::from_str_radix(cleaned_hex, 16)
}


pub fn dec_to_hex(decimal: u64) -> String {
    format!("{:X}", decimal)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_to_dec() {
        let hex_str = "14fb180".to_string();
        let decimal = hex_to_dec(hex_str).expect("Failed to convert hex to dec");
        eprintln!("[xuegao-web3][hex_dec_util][test_hex_to_dec][decimal={}]", decimal);
    }

    #[test]
    fn test_dec_to_hex() {
        let decimal = 26;
        let hex_str = dec_to_hex(decimal);
        eprintln!("[xuegao-web3][hex_dec_util][test_dec_to_hex][hex_str={}]", hex_str);
    }
}
