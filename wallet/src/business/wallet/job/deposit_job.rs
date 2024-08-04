use std::time::Duration;
use ethers::types::u256_from_f64_saturating;
use ethers::utils::ParseUnits::U256;
use log::info;
use sqlx::PgPool;
use tokio::time::interval;

use xuegao_fmk::util::time_chrono_util;

use crate::business::eth::model::enums::address_type_enum::AddressTypeEnum;
use crate::business::eth::service::eth_call_service;
use crate::business::wallet::model::doo::address_do::AddressDo;
use crate::business::wallet::model::doo::block_do::BlockDo;
use crate::business::wallet::model::doo::token_do::TokenDo;
use crate::config::constant::{chain, chain_eth};
use crate::config::constant::block::{DEFAULT_BLOCKS_NUMBER, DEFAULT_BLOCKS_STEP};

pub async fn deposit_job(pool: &PgPool) {
    // 充值 job
    // 先获取最大的区块高度，
    // 然后再获取上一次的扫链高度，+200 获取到所有的数据
    // 解析数据，拆分出block 和 tx，
    // 如果tx中，to的地址不是自己系统的数据，那么不要处理。

    let mut interval = interval(Duration::from_secs(5));
    loop {
        interval.tick().await;
        // 这里可以放定时任务逻辑，例如打印日志或执行某些操作
        let now = time_chrono_util::now_time();
        info!("[xuegao-web3][deposit_job][deposit_job][now={}]", now);

        let block_vec_result: Result<BlockDo, sqlx::Error> = BlockDo::list_order_by_number_limit_1(&pool).await;

        let mut db_latest_block_number: u64 = 0;

        if block_vec_result.is_err() {
            let block_vec_result_error = block_vec_result.err().unwrap();
            info!("[xuegao-web3][deposit_job][deposit_job][block_vec_result_error={}]", block_vec_result_error);
            // 处理具体的 sqlx::Error 类型
            match block_vec_result_error {
                sqlx::Error::RowNotFound => {
                    db_latest_block_number = DEFAULT_BLOCKS_NUMBER;
                }
                _ => {
                    // 处理其他 sqlx::Error 错误类型
                    info!("[xuegao-web3][deposit_job][deposit_job][other err]");
                }
            }
        } else {
            let block_do = block_vec_result.unwrap();
            // 数据库的数据是已存在的，所以要 + 1，数据库的数据，是经过扫描的
            db_latest_block_number = block_do.number as u64 + 1;
        }

        let end_block_number = db_latest_block_number + u64::from(DEFAULT_BLOCKS_STEP);
        let block_vec = eth_call_service::chain_scan(db_latest_block_number, end_block_number).await;
        info!("[xuegao-web3][deposit_job][deposit_job][block_vec={:?}]", serde_json::to_string(&block_vec));
        let block_vec = eth_call_service::filter_block(block_vec).await;
        if block_vec.is_empty() {
            info!("[xuegao-web3][deposit_job][deposit_job][block_vec is null]");
            return;
        }

        // 获取用户地址
        let address_set_result = AddressDo::list_address(pool, AddressTypeEnum::UserAddress.english()).await;
        if address_set_result.is_err() {
            let address_set_err = address_set_result.err().unwrap();
            info!("[xuegao-web3][deposit_job][deposit_job][address_set_err={}]", address_set_err);
            return;
        }
        let address_set = address_set_result.unwrap();
        info!("[xuegao-web3][deposit_job][deposit_job][address_set={:?}]", serde_json::to_string(&address_set));
        if address_set.is_empty() {
            info!("[xuegao-web3][deposit_job][deposit_job][address_set is empty]");
            continue;
        }

        let token_map_result = TokenDo::list_return_map(pool).await;
        if token_map_result.is_err() {
            let token_map_err = token_map_result.err().unwrap();
            info!("[xuegao-web3][deposit_job][deposit_job][token_map_err={}]", token_map_err);
            return;
        }
        let token_map = token_map_result.unwrap();
        info!("[xuegao-web3][deposit_job][deposit_job][token_map={:?}]", serde_json::to_string(&token_map));
        if token_map.is_empty() {
            info!("[xuegao-web3][deposit_job][deposit_job][token_map is empty]");
            continue;
        }

        for temp_block in block_vec {
            let temp_tx_vec = temp_block.transactions.clone();

            for transaction in temp_tx_vec {
                let to_address_option = transaction.to;
                let to_address = to_address_option.unwrap();

                if token_map.contains_key(&to_address) {
                    // 如果 tokens 不为 nil，说明这是一个与 token 相关的交易

                    // 将交易数据编码为十六进制字符串
                    let input_data = hex::encode(transaction.input.as_bytes());
                    info!("[xuegao-web3][deposit_job][deposit_job][input_data={}]", input_data);
                    // 如果输入数据长度小于 138 个字符，跳过处理
                    if input_data.len() < 138 {
                        continue;
                    }
                    // 检查输入数据的前 10 个字符是否为函数选择器 "0xa9059cbb"，如果不是，跳过
                    // 注意这里只取前8位，这里是 method_id
                    if &input_data[..8] != chain_eth::ERC20_TRANSFER_METHOD_ID {
                        continue;
                    }
                    // 从输入数据中提取目标地址，并转换为 Address
                    let to_address = format!("{}{}", chain::ADDRESS_STR_0X, &input_data[34..74]);

                    // 提取输入数据中的数值部分，并去掉前导的零
                    let trim_hex = input_data[74..138].trim_start_matches('0');
                    // 如果数值部分为空，跳过处理
                    if trim_hex.is_empty() {
                        continue;
                    }
                    // let u : u128
                    //
                    //
                    // let raw_value = U256::from_str(&format!("{}{}", chain::ADDRESS_STR_0X, trim_hex)).unwrap_or_default();
                    // // 使用 biguint 类型
                    // let dec_value = raw_value.to_biguint();
                    //
                    // let token_address = to_address;
                    // // 处理 token_address 和 dec_value
                    //
                    // // 将大整数转换为 decimal.Decimal 对象，并获取 BigInt 值
                    // decValue = decimal.NewFromBigInt(rawValue, 0).BigInt()

                } else {}
            }
        }

        info!("[xuegao-web3][deposit_job][deposit_job][deposit_job 处理结束 {}]", time_chrono_util::now());
    }
}



