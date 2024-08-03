use std::collections::HashMap;

use actix_web::Error;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

pub struct HttpUtil;

impl HttpUtil {
    pub async fn send_json_rpc(url: &str, method: &str, params: Vec<Value>) -> Result<Value, Error> {
        // 构造 JSON-RPC 请求体
        let request_body = json!({
            "jsonrpc": "2.0",
            "method": method,
            "params": params,
            "id": 1 // 请求的唯一标识，可以自定义
        });

        eprintln!("send_post_request url {}, method {:?}, params {:?}", url, method, params);

        let client = Client::new();
        let response = client
            .post(url)
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await
            .map_err(|error| {
                actix_web::error::ErrorInternalServerError(format!("Request failed: {:?}", error))
            })?;

        eprintln!("send_json_rpc_v2 resp: {:?}", response);

        // 检查响应状态
        if response.status().is_success() {
            // 解析响应体为 JSON
            // 解析响应体为 JSON
            let response_json: Value = response.json().await.map_err(|e| {
                actix_web::error::ErrorInternalServerError(format!("Failed to parse response JSON: {:?}", e))
            })?;
            eprintln!("send_json_rpc_v2 response_json: {:?}", response_json);
            Ok(response_json)
        } else {
            // 处理错误情况
            Err(actix_web::error::ErrorInternalServerError(format!("Request failed with status: {}", response.status())))
        }
    }
}