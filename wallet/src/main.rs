use std::{env, io};

use actix_http::body::MessageBody;
use actix_web::{App, dev::Service as _, HttpServer, middleware};
use actix_web::middleware::DefaultHeaders;
use actix_web::web;
use log::info;
use sqlx::postgres::PgPoolOptions;
use xuegao_fmk::util::time_util;
use crate::business::{chain_service, job};
use crate::business::service::chain_scan_service::chain_scan_service;

mod business;
mod config;

fn init_logger() {
    use std::io::Write;

    let env = env_logger::Env::default()
        .filter_or(env_logger::DEFAULT_FILTER_ENV, "info");

    env::set_var("RUST_LOG", "sqlx=debug");

    // 设置日志打印格式
    env_logger::Builder::from_env(env)
        .format(|buf, record| {
            writeln!(
                buf,
                "{} {} [{}] {}",
                time_util::format_system_time(time_util::now()),
                record.level(),
                record.module_path().unwrap_or("<unnamed>"),
                &record.args()
            )
        })
        .init();
    info!("env_logger initialized.");
}

// pub async fn mysql_conn() -> MySqlPool {
//     let database_url = env::var("DATABASE_URL").expect("Not configured in .env");
//     MySqlPoolOptions::new()
//         .max_connections(50)
//         .connect(&database_url)
//         .await
//         .unwrap()
// }

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    init_logger();

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(business::config::db::DATABASE_URL)
        .await
        .expect("Failed to create pool.");
    eprintln!("链接数据库成功");


    chain_service::create_address_service::create_batch_addresses_test(&pool).await;
    eprintln!("保存数据成功");

    // 启动定时任务
    tokio::spawn(business::job::deposit_job::deposit_job());

    HttpServer::new(move || {
        let mut app = App::new();

        app = app.app_data(web::Data::new(pool.clone()));

        // app = app.wrap_fn(|req, srv| {
        //     println!("Hi from start. You requested: {}", req.path());
        //     srv.call(req).map(|res| {
        //         println!("Hi from response");
        //         res
        //     })
        // });

        // std::env::set_var("RUST_LOG", "actix_web=info");
        // env_logger::init();
        // app = app.wrap(Logger::default());
        // app = app.wrap(Logger::new("%a %{User-Agent}i"));

        // app = app.wrap(middleware::DefaultHeaders::new().header("X-Version", "0.2"));

        // app = app.route("/health", web::get().to(|| async { "Hello, world!" }));
        // app = app.service(business::service::chain_service::throw_err);
        // app = app.service(business::service::chain_service::throw_fmk_error);
        // app = app.service(business::service::chain_service::responder_impl_responder);
        // app = app.service(business::service::chain_service::return_json);
        // app = app.service(business::service::chain_service::log_info);

        app = app.service(business::api::api_controller::health);
        app = app.service(business::api::api_controller::deposits_list);

        app = app.service(business::api::api_controller::withdrawals_list);
        app = app.service(business::api::api_controller::withdrawals_submit);

        return app;
    })
        .bind("127.0.0.1:9000")?
        .run()
        .await
}


