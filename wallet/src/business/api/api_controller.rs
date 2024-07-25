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
    let list = Deposit::list(&pool).unwrap();
    R::success(list);
}

#[post("/api/v1/withdrawals/list")]
async fn withdrawals_list(pool: web::Data<PgPool>) -> impl Responder {}

#[post("/api/v1/withdrawals/submit")]
async fn withdrawals_submit(pool: web::Data<PgPool>) -> impl Responder {}

