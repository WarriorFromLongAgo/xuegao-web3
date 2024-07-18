use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::json;

// 处理 GET 请求的处理函数，包含两个参数
async fn get_handler(info: web::Query<GetParams>) -> impl Responder {
    let str = format!("Received: param1 = {}, param2 = {}", info.param1, info.param2);
    println!("str {}", str);
    HttpResponse::Ok().json(str)
}


async fn get_no_param_handler() -> impl Responder {
    let str = format!("Received: get_no_param_handler");
    println!("str {}", str);
    HttpResponse::Ok().json(str)
}


// GET 请求的参数结构体
#[derive(Deserialize, Serialize)]
struct GetParams {
    param1: String,
    param2: String,
}

// 处理 POST 请求的处理函数，使用 JSON 解析
async fn post_handler(item: web::Json<PostParams>) -> impl Responder {
    let str = format!("Received: field1 = {}, field2 = {}", item.field1, item.field2);
    println!("str {}", str);

    // 将 Person 实例序列化为 JSON 字符串
    let json_string = serde_json::to_string(&item).unwrap();
    println!("Serialized JSON: {}", json_string);

    // 如果需要更漂亮的格式，可以使用 to_string_pretty
    let pretty_json_string = serde_json::to_string_pretty(&item).unwrap();
    println!("Pretty Serialized JSON: {}", pretty_json_string);

    // 直接构造 JSON 值并序列化
    let direct_json = json!({
        "name": "Bob",
        "age": 25,
        "address": "456 Another St"
    });
    println!("Direct JSON: {}", direct_json.to_string());

    HttpResponse::Ok().json(str)
}

// POST 请求的参数结构体
#[derive(Deserialize, Serialize)]
struct PostParams {
    field1: String,
    field2: String,
}


#[actix_web::main]
async fn main() -> Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/get", web::get().to(get_handler)) // GET 请求处理
            .route("/get_no_param", web::get().to(get_no_param_handler)) // GET 请求处理
            .route("/post", web::post().to(post_handler)) // POST 请求处理
    })
        .bind("127.0.0.1:8000")?
        .run()
        .await?;
    Ok(())
}
