use actix_web::{HttpResponse, post, Responder};
use actix_web::web::block;
use log::info;

use xuegao_fmk::web::fmk_error::FmkErrorEnum;
use xuegao_fmk::web::fmk_result;
use xuegao_fmk::web::fmk_result::FmkR;


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
    return FmkR::success("dasdadada");
}

#[post("/log_info")]
pub async fn log_info() -> impl Responder {
    info!("dadadadadadadadadaasda");
    info!("122222222222222222222222222222");
    info!("33333333333333333333333333");
    return FmkR::success("dasdadada");
}

#[post("/chain_scan_service")]
pub async fn chain_scan_service() -> impl Responder {
    // let block = chain_service::eth_call_service::get_block_by_hash().await;
    return FmkR::success("");
}

