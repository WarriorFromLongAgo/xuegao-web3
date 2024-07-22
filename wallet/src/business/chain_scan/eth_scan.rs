use std::string::ToString;
use actix_web::Error;
use serde_json::Value;

use crate::framework::util::req_util;

const ETH_JSON_RPC_URL: &str = "https://mainnet.optimism.io";

#[tokio::test]
async fn test() {

    let resp = get_block_by_hash();
    println!("get_block_by_hash {:?}", resp)

}

pub async fn get_block_by_hash() -> String {
    // 参数列表
    let params = vec![
        "0xdc0818cf78f21a8e70579cb46a43643f78291264dda342ae31049421c82d21ae",
        "latest",
    ];

    let resp: Result<Value, Error> = req_util::HttpUtil::send_json_rpc_v2(ETH_JSON_RPC_URL, "eth_getBlockByHash", params).await;

    // 根据返回结果返回合适的值
    match resp {
        Ok(response) => {
            let response_string = response.to_string();
            return response_string;
        }
        Err(err) => {
            // 直接抛出异常
            panic!("Failed to fetch block: {}", err);
        }
    }
}