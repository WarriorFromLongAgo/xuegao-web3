use crate::business::chain_service::eth_sign_service::create_address;
use crate::business::model::doo::address_do::AddressDo;
use crate::business::model::enums::address_type_enum::AddressTypeEnum;
use crate::business::model::enums::coin_type_enum::CoinTypeEnum;
use crate::framework::util::time_chrono_util;

/// 创建一个批量的以太坊地址。
///
/// 这个函数根据给定的数量、地址类型和链 ID 生成多个以太坊地址，并返回一个包含这些地址信息的 `Vec`。
///
/// # 参数
///
/// - `count`: 生成的地址数量。
/// - `address_type`: 地址类型，决定生成的地址的具体处理方式。
/// - `chain_id`: 链 ID，用于标识生成的地址所关联的区块链网络。
///
/// # 返回值
///
/// 返回一个 `Vec`，包含生成的以太坊地址信息，每个信息包括私钥、公钥、地址和链 ID。
///
/// # 示例
///
/// ```
/// let count = 5;
/// let address_type = AddressType::Standard;
/// let chain_id = 1; // 例如以太坊主网
/// let addresses = create_batch_addresses(count, address_type, chain_id);
///
/// for address in addresses {
///     println!("Private Key: {}", address.private_key);
///     println!("Public Key: {}", address.public_key);
///     println!("Address: {}", address.address);
///     println!("Chain ID: {}", address.chain_id);
/// }
/// ```
pub fn create_batch_addresses(count: usize, address_type: AddressTypeEnum, coin_type: CoinTypeEnum) {
    let mut addresses_vec = Vec::with_capacity(count);

    for _ in 0..count {
        if (CoinTypeEnum::Ether == coin_type) {
            let address_info = create_address();
            addresses_vec.push(address_info);
        }
    }

    let mut add_do_vec = Vec::with_capacity(count);
    let mut balance_do_vec = Vec::with_capacity(count);

    for tempAddInfo in addresses_vec {
        let address_do = AddressDo {
            guid: None,
            user_uid: "user_uid".to_string(),
            address: tempAddInfo.address,
            address_type: address_type.english().to_string(),
            private_key: tempAddInfo.private_key,
            public_key: tempAddInfo.public_key,
            timestamp: time_chrono_util::now_time(),
        };
        add_do_vec.push(address_do);


    }
}
