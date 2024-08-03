use ethers::types::Address;
use rust_decimal::Decimal;
use sqlx::PgPool;
use xuegao_fmk::util::time_chrono_util;
use crate::business::eth::call::eth_sign_service::create_address;
use crate::business::eth::model::enums::address_type_enum::AddressTypeEnum;
use crate::business::eth::model::enums::coin_type_enum::CoinTypeEnum;
use crate::business::wallet::model::doo::address_do::AddressDo;
use crate::business::wallet::model::doo::balance_do::BalanceDo;

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
pub async fn create_batch_addresses(pool: &PgPool, count: usize, address_type: AddressTypeEnum, coin_type: CoinTypeEnum) {
    let now = time_chrono_util::now_time();

    let mut addresses_vec = Vec::with_capacity(count);

    for _ in 0..count {
        if (CoinTypeEnum::Ether == coin_type) {
            let address_info = create_address();
            addresses_vec.push(address_info);
        }
    }

    let mut add_do_vec = Vec::with_capacity(count);
    let mut balance_do_vec = Vec::with_capacity(count);

    // addresses_vec.iter();
    // addresses_vec.iter_mut();
    // addresses_vec.into_iter();

    for tempAddInfo in addresses_vec.into_iter() {
        let address_do = AddressDo {
            guid: None,
            user_uid: "user_uid".to_string(),
            address: tempAddInfo.address.clone(),
            address_type: address_type.english().to_string(),
            private_key: tempAddInfo.private_key,
            public_key: tempAddInfo.public_key,
            timestamp: now,
        };
        add_do_vec.push(address_do);

        let balance_do = BalanceDo {
            guid: None,
            address: tempAddInfo.address.clone(),
            address_type: address_type.english().to_string(),
            token_address: format!("0x{}", hex::encode(Address::zero().as_bytes())), // 根据实际情况设置代币地址
            balance: Decimal::new(0, 0),
            lock_balance: Decimal::new(0, 0),
            timestamp: now,
        };
        balance_do_vec.push(balance_do);
    }

    // eprintln!("add_do_vec {:?}", add_do_vec);
    // eprintln!("balance_do_vec {:?}", balance_do_vec);

    for temp_address_do in add_do_vec {
        let insert_result = temp_address_do.insert(pool).await;
        match insert_result {
            Ok(inserted_address) => {
                // 处理成功插入的地址
                println!("Inserted address: {:?}", inserted_address);
            }
            Err(err) => {
                // 处理插入错误
                panic!("Failed temp_address_do: {}", err);
            }
        }
    }

    for temp_balance_do in balance_do_vec {
        let insert_result = temp_balance_do.insert(pool).await;
        match insert_result {
            Ok(inserted_balance) => {
                // 处理成功插入的地址
                println!("Inserted address: {:?}", inserted_balance);
            }
            Err(err) => {
                // 处理插入错误
                panic!("Failed temp_balance_do: {}", err);
            }
        }
    }
}

pub async fn create_batch_addresses_test(pool: &PgPool) {
    create_batch_addresses(pool, 3, AddressTypeEnum::UserAddress, CoinTypeEnum::Ether).await;
    create_batch_addresses(pool, 1, AddressTypeEnum::HotWallet, CoinTypeEnum::Ether).await;
    create_batch_addresses(pool, 1, AddressTypeEnum::CollectionWallet, CoinTypeEnum::Ether).await;
    create_batch_addresses(pool, 1, AddressTypeEnum::ColdWallet, CoinTypeEnum::Ether).await;
    return;
}


