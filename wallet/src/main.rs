use std::collections::HashMap;
use std::time::Duration;

use actix_web::{App, HttpResponse, HttpServer, web};
use tokio::time;

use util::time_util;

mod util;
mod config;

async fn my_periodic_task() {
    let mut interval = time::interval(Duration::from_secs(5));
    loop {
        interval.tick().await;

        // 这里是你的任务逻辑
        let str = time_util::format_system_time(time_util::now());
        println!("Task executed {}", str);
    }
}





#[derive(serde::Serialize)]
struct CustomPostData {
    field1: String,
    field2: String,
}

// 处理 GET 请求的处理函数，包含两个参数
async fn send() -> Result<HttpResponse, actix_web::Error> {
    // 发送带参数的 GET 请求
    let mut params = HashMap::new();
    params.insert("param1", "value1");
    params.insert("param2", "value2");

    let client = reqwest::Client::new();
    let response = client
        .get("http://127.0.0.1:8000/get_no_param")
        .query(&params)
        .send()
        .await
        .map_err(|err| {
            println!("req response error1 {}", err.to_string());
            return actix_web::error::ErrorInternalServerError(err.to_string());
        })?;
    let response_text = response
        .text()
        .await
        .map_err(|err| {
            println!("req response_text error1 {}", err.to_string());
            return actix_web::error::ErrorInternalServerError(err.to_string());
        })?;
    ;
    println!("GET Response: {}", response_text);

    let client_v2 = reqwest::Client::new();
    let response_v2 = client_v2
        .get("http://baidu.com/")
        .query(&params)
        .send()
        .await
        .map_err(|err| actix_web::error::ErrorInternalServerError(err.to_string()))?;
    let response_text_v2 = response_v2
        .text()
        .await
        .map_err(|err| actix_web::error::ErrorInternalServerError(err.to_string()))?;
    ;
    println!("GET Response v2: {}", response_text_v2);


    let get_response = req_util::HttpUtil::send_get_request("http://127.0.0.1:8000/get", &params)
        .await
        .map_err(|err| {
            println!("error {}", err.to_string());
            return actix_web::error::ErrorInternalServerError(err.to_string());
        })?;
    println!("GET Response: {}", get_response);

    // 发送 JSON 数据的 POST 请求
    let post_data = CustomPostData {
        field1: String::from("value1"),
        field2: String::from("value2"),
    };

    let post_response = req_util::HttpUtil::send_post_request("http://127.0.0.1:8000/post", &post_data)
        .await
        .map_err(|err| actix_web::error::ErrorInternalServerError(err.to_string()))?;
    println!("POST Response: {}", post_response);

    // Ok::<HttpResponse, Error>(HttpResponse::Ok().json("请求成功"))
    Ok(HttpResponse::Ok().json("请求成功"))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // 启动定时任务
    // actix_rt::spawn(my_periodic_task());

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(|| async { "Hello, world!" }))
            .route("/send", web::get().to(send))
    })
        .bind("127.0.0.1:9000")?
        .run()
        .await
}
