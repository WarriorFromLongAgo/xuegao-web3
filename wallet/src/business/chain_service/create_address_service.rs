use crate::business::model::chain::address_info::AddressInfo;
use crate::business::model::enums::address_type_enum::AddressType;



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
pub fn create_batch_addresses(count: usize, address_type: AddressType, chain_id: u64) -> Vec<AddressInfo> {
    let mut addresses = Vec::with_capacity(count);

    for _ in 0..count {
        if ()


        let address_info = create_address(chain_id);
        // 根据 address_type 进行处理
        match address_type {
            AddressType::Standard => {
                // 这里可以加入标准地址类型的处理逻辑
                addresses.push(address_info);
            },
            AddressType::OtherType => {
                // 这里可以加入其他地址类型的处理逻辑
                addresses.push(address_info);
            },
        }
    }

    addresses
}
