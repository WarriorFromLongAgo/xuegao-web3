// 充值 job

use std::time::Duration;

use tokio::time::interval;
use xuegao_fmk::util::time_chrono_util;

pub async fn deposit_job() {
    let mut interval = interval(Duration::from_secs(5));
    loop {
        interval.tick().await;
        // 这里可以放定时任务逻辑，例如打印日志或执行某些操作
        let now = time_chrono_util::now_time();
        println!("Periodic task executed {}", now);






    }
}



