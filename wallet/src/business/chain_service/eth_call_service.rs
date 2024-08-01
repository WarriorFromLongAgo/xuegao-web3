use std::string::ToString;

use actix_web::Error;
use serde_json::Value;

use crate::business::model::chain::eth::eth_block_number::EthBlockNumber;
use crate::framework::util::req_util;

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

pub async fn get_block_by_block_number(block_number_hex: Option<String>) -> Option<_> {
    if (block_number_hex.is_none()) {
        return None;
    }


    // 构建 JSON-RPC 请求参数
    let mut params = Vec::new();
    params.push(Value::String(input));
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


/// 根据区块hash，获取区块信息
pub async fn get_block_by_block_hash(block_hash: Option<String>) -> Option<String> {
    let input = block_hash?;

    // 构建 JSON-RPC 请求参数
    let mut params = Vec::new();
    params.push(Value::String(input));
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

