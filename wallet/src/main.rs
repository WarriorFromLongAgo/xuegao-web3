use std::collections::HashMap;
use std::time::Duration;

use actix_web::{App, HttpResponse, HttpServer, post, Responder, web};
use actix_web::web::JsonBody;
use tokio::time;

mod business;
mod framework;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // 启动定时任务

    HttpServer::new(|| {
        let mut app = App::new(); // 显式定义 app 的类型为 App<_, Body>
        app = app.route("/health", web::get().to(|| async { "Hello, world!" }));
        app = app.service(business::service::chain_scan::throw_err);
        app = app.service(business::service::chain_scan::throw_fmk_error);
        app = app.service(business::service::chain_scan::responder_impl_responder);
        app = app.service(business::service::chain_scan::return_json);
        // app = app.service(business::service::chain_scan::responder_str);
        // app = app.service(business::service::chain_scan::responder_string);
        return app;
    })
        .bind("127.0.0.1:9000")?
        .run()
        .await
}
