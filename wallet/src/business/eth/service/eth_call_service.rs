use log::info;

use crate::business::eth::call::eth_call;
use crate::business::eth::model::enums::block_status_enum::BlockStatusEnum;
use crate::business::eth::model::enums::transaction_type_enum::TransactionTypeEnum;
use crate::business::eth::model::resp::eth_block_resp::EthBlock;
use crate::business::eth::model::resp::transaction_resp::Transaction;
use crate::business::wallet::util::hex_dec_util::dec_to_hex;
use crate::config::constant::block::DEFAULT_BLOCKS_STEP;

pub async fn chain_scan(start_block_number: u64, end_block_number: u64) -> Vec<EthBlock> {
    let mut eth_block_vec: Vec<EthBlock> = Vec::with_capacity(DEFAULT_BLOCKS_STEP as usize);

    for i in start_block_number..end_block_number {
        let temp_hex_block_number = dec_to_hex(i);
        let temp_block_status_enum = BlockStatusEnum::Number(temp_hex_block_number);
        let eth_block_result = eth_call::get_block_by_block_number(temp_block_status_enum).await;
        eth_block_vec.push(eth_block_result);
    }
    return eth_block_vec;
}

// 如果tx的to地址为空，那么过滤
// 如果tx的type=4，那么过滤
pub async fn filter_block(input: Vec<EthBlock>) -> Vec<EthBlock> {
    if input.is_empty() {
        info!("[xuegao-web3][eth_call_service][filter_block][input is null]");
        return Vec::new();
    }
    let mut filtered_blocks = Vec::new();

    for mut temp_block in input {
        let temp_tx_vec = temp_block.transactions.clone();
        if temp_tx_vec.is_empty() {
            filtered_blocks.push(temp_block.clone());
            continue;
        }

        let filtered_tx_vec: Vec<Transaction> = temp_tx_vec
            .into_iter()
            .filter(|tx| {
                if tx.to.is_none() {
                    return false;
                }
                if tx.tx_type_enum == TransactionTypeEnum::Blob {
                    return false;
                }
                true
            }).collect();

        // 更新区块中的交易列表
        temp_block.transactions = filtered_tx_vec;

        filtered_blocks.push(temp_block);
    }
    info!("[xuegao-web3][eth_call_service][filter_block][filtered_blocks={:?}]", serde_json::to_string(&filtered_blocks));
    return filtered_blocks;
}

