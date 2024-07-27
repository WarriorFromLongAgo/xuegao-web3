use actix_web::{HttpResponse, post, Responder};
use log::info;
use crate::business::chain_service;
use crate::framework::web::fmk_error::FmkErrorEnum;
use crate::framework::web::fmk_result::R;

#[post("/throw_err")]
pub async fn throw_err() -> impl Responder {
    let a = 1 / 0;

    return HttpResponse::Ok().json("throw_err");
}

#[post("/throw_fmk_error")]
pub async fn throw_fmk_error() -> Result<&'static str, FmkErrorEnum> {
    let str = "dadadad".to_string();
    Err(FmkErrorEnum::ServerError(str))
}

#[post("/normal")]
pub async fn responder_impl_responder() -> impl Responder {
    return HttpResponse::Ok().json("normal");
}

#[post("/return_json")]
pub async fn return_json() -> impl Responder {
    return R::success("dasdadada");
}

#[post("/log_info")]
pub async fn log_info() -> impl Responder {
    info!("dadadadadadadadadaasda");
    info!("122222222222222222222222222222");
    info!("33333333333333333333333333");
    return R::success("dasdadada");
}

#[post("/chain_scan_service")]
pub async fn chain_scan_service() -> impl Responder {
    let block = chain_service::eth_call_service::get_block_by_hash().await;

    // 将 Person 实例序列化为 JSON 字符串
    let json_string = serde_json::to_string(&block).unwrap();
    println!("chain_scan_service JSON: {}", json_string);

    return HttpResponse::Ok().json("fsfssfsfds");
}

