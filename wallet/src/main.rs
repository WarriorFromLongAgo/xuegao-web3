#![recursion_limit = "512"]

use std::{env, io};
use std::sync::{Arc, Mutex};
use actix_http::body::MessageBody;
use actix_web::{App, dev::Service as _, HttpServer, middleware};
use actix_web::middleware::DefaultHeaders;
use actix_web::web;
use log::info;
use sqlx::postgres::PgPoolOptions;
use xuegao_fmk::config::log4rs::init_logger;
use xuegao_fmk::util::time_chrono_util::format;
use xuegao_fmk::util::time_util;
use crate::business::eth::service::create_address_service::create_batch_addresses_test;
use crate::business::wallet::api::api_controller::{deposits_list, health, withdrawals_list, withdrawals_submit};
use crate::business::wallet::job::deposit_job::deposit_job;
use crate::config::db::DATABASE_URL;

mod business;
mod config;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    init_logger();

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(DATABASE_URL)
        .await
        .expect("Failed to create pool.");
    info!("链接数据库成功 {:?}", pool);
    // 将 pool 包装在 Arc 中
    let pool_arc = Arc::new(pool);

    create_batch_addresses_test(&pool_arc).await;
    info!("保存数据成功");

    // 启动定时任务
    let pool_arc_clone = Arc::clone(&pool_arc);
    tokio::spawn(async move {
        deposit_job(&pool_arc_clone).await;
    });

    HttpServer::new(move || {
        let mut app = App::new();

        // 使用已有的 pool_arc，不需要再次克隆
        let pool_arc_clone_v2 = Arc::clone(&pool_arc);
        app = app.app_data(web::Data::new(pool_arc_clone_v2));

        app = app.service(health);
        app = app.service(deposits_list);
        app = app.service(withdrawals_list);
        app = app.service(withdrawals_submit);

        return app;
    })
        .bind("127.0.0.1:9000")?
        .run()
        .await
}


