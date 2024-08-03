use ethers::signers::{LocalWallet, Signer};
use ethers::types::Address;
use crate::business::eth::model::resp::address_info::AddressInfo;

pub fn private_create_address() -> (String, String, Address) {
    // 生成随机钱包
    let wallet = LocalWallet::new(&mut rand::thread_rng());

    // 获取私钥
    let private_key = wallet.signer().to_bytes();
    let private_key_hex = hex::encode(private_key);
    eprintln!("create_address Private Key: 0x{}", private_key_hex);

    // 获取公钥
    let public_key = wallet.signer().verifying_key().to_encoded_point(false);
    let public_key_bytes = public_key.as_bytes();
    let public_key_hex = hex::encode(public_key_bytes);
    eprintln!("create_address Public Key: 0x{}", public_key_hex);

    // 获取地址
    let address: Address = wallet.address();
    eprintln!("create_address Address: {:?}", serde_json::to_string(&address));
    return (private_key_hex, public_key_hex, address);
}

pub fn create_address() -> AddressInfo {
    let (private_key_hex, public_key_hex, address) = private_create_address();

    AddressInfo {
        private_key: private_key_hex,
        public_key: public_key_hex,
        address: format!("{:?}", address), // 格式化地址为 `0x` 前缀形式
    }
}

