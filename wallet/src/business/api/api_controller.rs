use actix_web::{get, post, Responder, web};
use sqlx::PgPool;

use crate::business::model::r#do::deposit::Deposit;
use crate::framework::web::fmk_result::R;

#[get("/health")]
async fn health() -> impl Responder {
    R::success("")
}

#[post("/api/v1/deposits/list")]
async fn deposits_list(pool: web::Data<PgPool>) -> impl Responder {
    let list = Deposit::list(&pool).await;
    match list {
        Ok(list) => R::success(list),
        Err(e) => {
            // 处理错误，例如返回一个错误响应
            eprintln!("Error querying deposits: {:?}", e);
            // 假设 R::error 是你用来处理错误响应的方法
            return R::<String>::err_msg("Failed to fetch deposits".to_string());
        }
    }
}

#[post("/api/v1/withdrawals/list")]
async fn withdrawals_list(pool: web::Data<PgPool>) -> impl Responder {
    return R::success("withdrawals_list");
}

#[post("/api/v1/withdrawals/submit")]
async fn withdrawals_submit(pool: web::Data<PgPool>) -> impl Responder {
    return R::success("withdrawals_submit");
}

