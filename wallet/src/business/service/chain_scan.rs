use actix_web::{HttpResponse, post, Responder};

use crate::business::chain_scan;
use crate::framework::web::fmk_error::FmkErrorEnum;
use crate::framework::web::fmk_result::R;

#[post("/throw_err")]
pub async fn throw_err() -> impl Responder {
    let a = 1 / 0;

    return HttpResponse::Ok().json("throw_err");
}

#[post("/throw_fmk_error")]
pub async fn throw_fmk_error() -> Result<&'static str, FmkErrorEnum> {
    Err(FmkErrorEnum::BadClientData)
}

#[post("/normal")]
pub async fn responder_impl_responder() -> impl Responder {
    return HttpResponse::Ok().json("normal");
}

#[post("/return_json")]
pub async fn return_json() -> impl Responder {
    return R::success("dasdadada");
}

// 不允许
// #[post("/responder_result_str")]
// pub async fn responder_result_str()-> Result<String> {
//     return  Ok("result");
// }

// curl http://localhost:8088/responder/str
// pub async fn responder_str() -> &'static str {
//     "responder_str"
// }

// curl http://localhost:8088/responder/string
// pub async fn responder_string() -> String {
//     "responder_string".to_owned()
// }


#[post("/chain_scan_service")]
pub async fn chain_scan_service() -> impl Responder {
    let block = chain_scan::eth_scan::get_block_by_hash().await;

    // 将 Person 实例序列化为 JSON 字符串
    let json_string = serde_json::to_string(&block).unwrap();
    println!("chain_scan_service JSON: {}", json_string);

    return HttpResponse::Ok().json("fsfssfsfds");
}

