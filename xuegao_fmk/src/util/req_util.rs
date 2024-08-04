use actix_web::Error;
use log::info;
use reqwest::Client;
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
        info!("send_post_request url {}, method {}, params {:?}", url, method, params);

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


        // 检查响应状态
        if response.status().is_success() {
            // 解析响应体为 JSON
            let response_json: Result<Value, _> = response.json().await.map_err(|e| {
                actix_web::error::ErrorInternalServerError(format!("Failed to parse response JSON: {:?}", e))
            });

            // 解包 response_json
            match response_json {
                Ok(value) => {
                    let response_json = serde_json::to_string(&value)
                        .map_err(|e| actix_web::error::ErrorInternalServerError(format!("Failed to format JSON: {:?}", e)))?;

                    info!("response_json {} ", response_json);

                    Ok(value)
                }
                Err(e) => Err(actix_web::error::ErrorInternalServerError(format!("Failed to parse response JSON: {:?}", e))),
            }
        } else {
            // 处理错误情况
            Err(actix_web::error::ErrorInternalServerError(format!("Request failed with status: {}", response.status())))
        }
    }
}