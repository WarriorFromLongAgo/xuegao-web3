use actix_web::{get, post, Responder, web};
use sqlx::PgPool;
use xuegao_fmk::web::fmk_result::FmkR;
use crate::business::wallet::model::doo::deposit_do::DepositDo;
use crate::business::wallet::model::doo::withdraw_do::WithdrawDo;

#[get("/health")]
async fn health() -> impl Responder {
    return FmkR::success("");
}


#[post("/api/v1/deposits/list_v2")]
async fn deposits_list(pool: web::Data<PgPool>) -> impl Responder {
    let list = DepositDo::list(&pool).await;
    match list {
        Ok(list) => FmkR::success(list),
        Err(e) => {
            // 处理错误，例如返回一个错误响应
            eprintln!("Error querying deposits: {:?}", e);
            // 假设 R::error 是你用来处理错误响应的方法
            return FmkR::<String>::err_msg("Failed to fetch deposits".to_string());
        }
    }
}

#[post("/api/v1/deposits/list")]
async fn deposits_list_v2(pool: web::Data<PgPool>) -> impl Responder {
    let list = DepositDo::list(&pool).await;
    eprintln!("list {:?}", list);
    match list {
        Ok(list) => FmkR::success(list),
        Err(e) => {
            // 处理错误，例如返回一个包含错误信息的响应
            // HttpResponse::InternalServerError().body(format!("Error: {:?}", e))
            return FmkR::<String>::err_msg("Failed to fetch deposits".to_string());
        }
    }
}



#[post("/api/v1/withdrawals/list")]
async fn withdrawals_list(pool: web::Data<PgPool>) -> impl Responder {
    let list = WithdrawDo::list(&pool).await;
    eprintln!("list {:?}", list);
    match list {
        Ok(list) => FmkR::success(list),
        Err(e) => {
            // 处理错误，例如返回一个包含错误信息的响应
            // HttpResponse::InternalServerError().body(format!("Error: {:?}", e))
            return FmkR::<String>::err_msg("Failed to fetch deposits".to_string());
        }
    }
}

#[post("/api/v1/withdrawals/submit")]
async fn withdrawals_submit(pool: web::Data<PgPool>) -> impl Responder {
    return FmkR::success("withdrawals_submit");
}

