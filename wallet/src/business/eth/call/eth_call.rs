use std::string::ToString;

use actix_web::Error;
use log::info;
use serde_json::Value;

use xuegao_fmk::util::req_util;

use crate::business::eth::model::enums::block_status_enum::BlockStatusEnum;
use crate::business::eth::model::req::eth_block_number::EthBlockNumber;
use crate::business::eth::model::req::eth_gas_price::EthGasPrice;
use crate::business::eth::model::req::eth_max_priority_fee_per_gas::EthMaxPriorityFeePerGas;
use crate::business::eth::model::resp::eth_block_resp::EthBlock;
use crate::business::wallet::util::hex_dec_util::hex_to_dec;

const ETH_JSON_RPC_URL: &str = "https://go.getblock.io/b54672ce0d524fd9b9aca108594d11b4";


/// 获取区块最新的高度
/// response_string {"id":1,"jsonrpc":"2.0","result":"0x137bfc9"}
// test_eth_block_number "20430793"
pub async fn latest_block_number() -> EthBlockNumber {
    let params = Vec::new();
    let resp: Result<Value, Error> = req_util::HttpUtil::send_json_rpc(ETH_JSON_RPC_URL, "eth_blockNumber", params).await;

    match resp {
        Ok(response) => {
            let response_string = response.to_string();
            eprintln!("response_string {}", response_string);

            // 提取 result 字段
            let result_str = match response.get("result").and_then(Value::as_str) {
                Some(hex_str) => hex_str,
                None => panic!("Invalid response format, 'result' field not found or not a string"),
            };
            eprintln!("response_string result {}", result_str);

            // 去掉前缀 "0x" 并转换为十进制
            let hex_str = &result_str[2..];
            let decimal_value = u64::from_str_radix(hex_str, 16)
                .expect("Failed to convert hex to decimal"); // 使用 expect 替代 unwrap，提供错误信息
            eprintln!("eth_block_number {}", decimal_value);

            EthBlockNumber {
                block_number_hex: result_str.to_string(),
                block_number_dec: decimal_value.to_string(),
            }
        }
        Err(err) => {
            eprintln!("Failed to fetch block: {}", err);
            panic!("Failed to fetch block");
        }
    }
}

pub async fn get_gas_price() -> EthGasPrice {
    let params = Vec::new();
    let resp: Result<Value, Error> = req_util::HttpUtil::send_json_rpc(ETH_JSON_RPC_URL, "eth_gasPrice", params).await;


    match resp {
        Ok(response) => {
            let response_string = response.to_string();
            eprintln!("get_gas_price response_string {}", response_string);

            // 提取 result 字段
            let result_str = match response.get("result").and_then(Value::as_str) {
                Some(hex_str) => hex_str,
                None => panic!("Invalid response format, 'result' field not found or not a string"),
            };
            eprintln!("get_gas_price response_string result {}", result_str);

            // 去掉前缀 "0x" 并转换为十进制
            let hex_str = &result_str[2..];
            let decimal_value = u64::from_str_radix(hex_str, 16)
                .expect("Failed to convert hex to decimal"); // 使用 expect 替代 unwrap，提供错误信息
            eprintln!("get_gas_price {}", decimal_value);

            EthGasPrice {
                gas_price_hex: result_str.to_string(),
                gas_price_dec: decimal_value.to_string(),
            }
        }
        Err(err) => {
            eprintln!("Failed to fetch block: {}", err);
            panic!("Failed to fetch block");
        }
    }
}

pub async fn get_max_priority_fee_per_gas() -> EthMaxPriorityFeePerGas {
    let params = Vec::new();
    let resp: Result<Value, Error> = req_util::HttpUtil::send_json_rpc(ETH_JSON_RPC_URL, "eth_maxPriorityFeePerGas", params).await;

    match resp {
        Ok(response) => {
            let response_string: String = response.to_string();
            eprintln!("get_max_priority_fee_per_gas response_string {}", response_string);

            // 提取 result 字段
            let result_str: String = match response.get("result") {
                Some(hex_str) => hex_str.as_str().unwrap_or("").to_string(),
                None => panic!("Invalid response format, 'result' field not found or not a string"),
            };
            eprintln!("get_max_priority_fee_per_gas response_string result {}", result_str);

            // 去掉前缀 "0x" 并转换为十进制
            let decimal_value = hex_to_dec(result_str.to_string()).expect("Failed to convert hex to decimal");
            eprintln!("get_max_priority_fee_per_gas {}", decimal_value);

            EthMaxPriorityFeePerGas {
                max_priority_fee_per_gas_hex: result_str.to_string(),
                max_priority_fee_per_gas_dec: decimal_value.to_string(),
            }
        }
        Err(err) => {
            eprintln!("Failed to fetch block: {}", err);
            panic!("Failed to fetch block");
        }
    }
}

pub async fn get_block_by_block_number(block_status_enum: BlockStatusEnum) -> EthBlock {
    let mut params = Vec::new();
    params.push(Value::String(block_status_enum.english()));
    params.push(Value::Bool(true));
    let resp: Result<Value, Error> = req_util::HttpUtil::send_json_rpc(ETH_JSON_RPC_URL, "eth_getBlockByNumber", params).await;

    if (resp.is_err()) {
        let resp_er = resp.err().unwrap();
        panic!("Failed to fetch block: {}", resp_er);
    }
    let resp_value = resp.unwrap();
    info!("[xuegao-web3][eth_call][][resp_value={:?}]", serde_json::to_string(&resp_value));

    let resp_value_result = resp_value["result"].clone();
    if resp_value_result.is_null() {
        panic!("Block not found for the given block number");
    }
    // 克隆 resp_value_result 以便在解析失败时使用
    let resp_value_result_clone = resp_value_result.clone();
    let mut block: EthBlock = match serde_json::from_value(resp_value_result) {
        Ok(block) => block,
        Err(e) => {
            info!("[xuegao-web3][eth_call][resp_value_result][error={:?}]", serde_json::to_string(&resp_value_result_clone));
            panic!("Failed to parse block response: {}", e)
        },
    };

    block.deal_dec_hex();
    info!("[xuegao-web3][eth_call][][block={:?}]", serde_json::to_string(&block));
    return block;
}


/// 根据区块hash，获取区块信息
pub async fn get_block_by_block_hash(block_hash_option: Option<String>) -> Option<String> {
    if (block_hash_option.is_none()) {
        return None;
    }
    let block_hash = block_hash_option.unwrap();
    if (block_hash.trim().is_empty()) {
        return None;
    }

    let mut params = Vec::new();
    params.push(Value::String(block_hash));
    params.push(Value::Bool(true));
    let resp: Result<Value, Error> = req_util::HttpUtil::send_json_rpc(ETH_JSON_RPC_URL, "eth_getBlockByHash", params).await;

    // 根据返回结果返回合适的值
    match resp {
        Ok(response) => {
            let response_string = response.to_string();
            return Some(response_string);
        }
        Err(err) => {
            // 直接抛出异常
            panic!("Failed to fetch block: {}", err);
        }
    }
}

/// 根据交易哈希返回交易信息
/// 注意：这个接口返回的数据，和 get_block_by_block_hash 设置true返回的数据，一模一样
pub async fn get_tx_by_tx_hash(tx_hash: String) -> Option<String> {
    if (tx_hash.trim().is_empty()) {
        return None;
    }

    let mut params = Vec::new();
    params.push(Value::String(tx_hash));
    let resp: Result<Value, Error> = req_util::HttpUtil::send_json_rpc(ETH_JSON_RPC_URL, "eth_getTransactionByHash", params).await;

    // 根据返回结果返回合适的值
    match resp {
        Ok(response) => {
            let response_string = response.to_string();
            return Some(response_string);
        }
        Err(err) => {
            // 直接抛出异常
            panic!("Failed to fetch block: {}", err);
        }
    }
}

/// 根据交易哈希返回交易信息
/// 注意：这个接口返回的数据，和 get_block_by_block_hash 设置true返回的数据，一模一样
pub async fn get_tx_receipt_by_tx_hash(tx_hash: String) -> Option<String> {
    if (tx_hash.trim().is_empty()) {
        return None;
    }

    let mut params = Vec::new();
    params.push(Value::String(tx_hash));
    let resp: Result<Value, Error> = req_util::HttpUtil::send_json_rpc(ETH_JSON_RPC_URL, "eth_getTransactionReceipt", params).await;

    // 根据返回结果返回合适的值
    match resp {
        Ok(response) => {
            let response_string = response.to_string();
            return Some(response_string);
        }
        Err(err) => {
            // 直接抛出异常
            panic!("Failed to fetch block: {}", err);
        }
    }
}

pub async fn get_tx_count_by_address(tx_hash: String,
                                     block_status_enum: BlockStatusEnum) -> Option<String> {
    let mut params = Vec::new();
    params.push(Value::String(tx_hash));
    params.push(Value::String(block_status_enum.english()));
    let resp: Result<Value, Error> = req_util::HttpUtil::send_json_rpc(ETH_JSON_RPC_URL, "eth_getTransactionCount", params).await;

    // 根据返回结果返回合适的值
    match resp {
        Ok(response) => {
            let response_string = response.to_string();
            return Some(response_string);
        }
        Err(err) => {
            // 直接抛出异常
            panic!("Failed to fetch block: {}", err);
        }
    }
}


pub async fn eth_send_raw_transaction(block_status_enum: BlockStatusEnum) -> Option<String> {
    let mut params = Vec::new();
    params.push(Value::String(block_status_enum.english()));
    params.push(Value::Bool(true));
    let resp: Result<Value, Error> = req_util::HttpUtil::send_json_rpc(ETH_JSON_RPC_URL, "eth_sendRawTransaction", params).await;

    // 根据返回结果返回合适的值
    match resp {
        Ok(response) => {
            let response_string = response.to_string();
            return Some(response_string);
        }
        Err(err) => {
            // 直接抛出异常
            panic!("Failed to fetch block: {}", err);
        }
    }
}


