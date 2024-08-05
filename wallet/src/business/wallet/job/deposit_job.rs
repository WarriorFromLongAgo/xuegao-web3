use std::time::Duration;

use log::info;
use rust_decimal::Decimal;
use rust_decimal::prelude::Zero;
use sqlx::PgPool;
use tokio::time::interval;

use xuegao_fmk::util::time_chrono_util;

use crate::business::eth::model::enums::address_type_enum::AddressTypeEnum;
use crate::business::eth::service::eth_call_service;
use crate::business::wallet::model::doo::address_do::AddressDo;
use crate::business::wallet::model::doo::block_do::BlockDo;
use crate::business::wallet::model::doo::token_do::TokenDo;
use crate::business::wallet::util::token_util;
use crate::config::constant;
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

        let block_vec_result: Result<BlockDo, sqlx::Error> =
            BlockDo::list_order_by_number_limit_1(&pool).await;

        let mut db_latest_block_number: u64 = 0;

        if block_vec_result.is_err() {
            let block_vec_result_error = block_vec_result.err().unwrap();
            info!(
                "[xuegao-web3][deposit_job][deposit_job][block_vec_result_error={}]",
                block_vec_result_error
            );
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
        let block_vec =
            eth_call_service::chain_scan(db_latest_block_number, end_block_number).await;
        info!(
            "[xuegao-web3][deposit_job][deposit_job][block_vec={:?}]",
            serde_json::to_string(&block_vec)
        );
        let block_vec = eth_call_service::filter_block(block_vec).await;
        if block_vec.is_empty() {
            info!("[xuegao-web3][deposit_job][deposit_job][block_vec is null]");
            return;
        }

        // 获取用户地址
        let address_set_result =
            AddressDo::list_address(pool, AddressTypeEnum::UserAddress.english()).await;
        if address_set_result.is_err() {
            let address_set_err = address_set_result.err().unwrap();
            panic!(
                "[xuegao-web3][deposit_job][deposit_job][address_set_err={}",
                address_set_err
            );
        }
        let address_set = address_set_result.unwrap();
        info!(
            "[xuegao-web3][deposit_job][deposit_job][address_set={:?}]",
            serde_json::to_string(&address_set)
        );
        if address_set.is_empty() {
            info!("[xuegao-web3][deposit_job][deposit_job][address_set is empty]");
            continue;
        }

        let token_map_result = TokenDo::list_return_map(pool).await;
        if token_map_result.is_err() {
            let token_map_err = token_map_result.err().unwrap();
            panic!(
                "[xuegao-web3][deposit_job][deposit_job][token_map_err={}",
                token_map_err
            );
        }
        let token_map = token_map_result.unwrap();
        info!(
            "[xuegao-web3][deposit_job][deposit_job][token_map={:?}]",
            serde_json::to_string(&token_map)
        );
        if token_map.is_empty() {
            info!("[xuegao-web3][deposit_job][deposit_job][token_map is empty]");
            continue;
        }

        for temp_block in block_vec {
            let temp_tx_vec = temp_block.transactions.clone();

            let mut to_address = "";
            let mut token_address = "";
            let mut transfer_fee = Decimal::zero();

            for transaction in temp_tx_vec {
                let to_address_option = transaction.to;
                let to_address = to_address_option.unwrap();

                if token_map.contains_key(&to_address) {
                    // 如果 tokens 不为 nil，说明这是一个与 token 相关的交易

                    let tx_input = transaction.input.clone();
                    info!(
                        "[xuegao-web3][deposit_job][deposit_job][tx_input={}]",
                        tx_input
                    );

                    let is_token_transfer = token_util::input_data_is_token_transfer(tx_input.clone());
                    if !is_token_transfer {
                        info!("[xuegao-web3][deposit_job][deposit_job][is_token_transfer={}]", is_token_transfer);
                        continue;
                    }
                    let method_id = token_util::input_data_2_method_id(tx_input.clone());
                    if method_id != chain_eth::ERC20_TRANSFER_METHOD_ID {
                        info!("[xuegao-web3][deposit_job][deposit_job][method_id not token_transfer]");
                        continue;
                    }
                    let amount = token_util::input_data_2_amount(tx_input.clone());
                    let to_address = token_util::input_data_2_to_address(tx_input.clone());

                } else {}
            }
        }

        info!(
            "[xuegao-web3][deposit_job][deposit_job][deposit_job 处理结束 {}]",
            time_chrono_util::now()
        );
    }
}
