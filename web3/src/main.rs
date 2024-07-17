use std::time::Duration;

use actix_web::{App, HttpServer, web};
use tokio::time;

use util::time_util;

mod util;
mod config;

async fn my_periodic_task() {
    let mut interval = time::interval(Duration::from_secs(5));
    loop {
        interval.tick().await;

        // 这里是你的任务逻辑
        let str = time_util::format_system_time(time_util::now());
        println!("Task executed {}", str);
    }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // 启动定时任务
    actix_rt::spawn(my_periodic_task());

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(|| async { "Hello, world!" }))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
