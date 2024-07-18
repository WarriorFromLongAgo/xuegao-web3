use std::collections::HashMap;

use reqwest::Error;
use serde::Serialize;

#[tokio::test]
async fn test() {

    let resp = reqwest::get("http://127.0.0.1:8000/get_no_param").await;
    println!("resp {:?}", resp)

    // let body = reqwest::get("http://127.0.0.1:8000/get_no_param")
    //     .await.expect("Failed to read response text 1 ")
    //     .text()
    //     .await.expect("Failed to read response text 2");
    // println!("body = {body:?}");
}

pub struct HttpUtil;

impl HttpUtil {
    // 发送带参数的 GET 请求
    pub async fn send_get_request(url: &str, params: &HashMap<&str, &str>) -> Result<String, Error> {
        let client = reqwest::Client::new();
        let response = client
            .get(url)
            .query(params)
            .send()
            .await?;

        let response_text = response.text().await?;
        println!("send_get_request: {}", response_text);
        Ok(response_text)
    }

    // 发送 JSON 数据的 POST 请求
    pub async fn send_post_request<T: Serialize>(url: &str, json_data: &T) -> Result<String, Error> {
        let client = reqwest::Client::new();
        let response = client
            .post(url)
            .json(json_data)
            .send()
            .await?;

        let response_text = response.text().await?;
        println!("send_post_request: {}", response_text);
        Ok(response_text)
    }
}