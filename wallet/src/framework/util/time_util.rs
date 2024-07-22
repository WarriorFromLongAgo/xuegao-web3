use std::time::{SystemTime, UNIX_EPOCH};

pub fn now() -> SystemTime {
    // 获取当前时间并打印
    let now = SystemTime::now();
    return now;
}

pub fn format_system_time(system_time: SystemTime) -> String {
    // 获取当前时间与 UNIX 时间原点（1970-01-01 00:00:00 UTC）的时间差
    let datetime = system_time.duration_since(UNIX_EPOCH).expect("Time went backwards");
    // 获取时间差的总秒数
    let total_seconds = datetime.as_secs();
    // println!("total_seconds {}", total_seconds);
    // 获取时间差的总毫秒数
    // let total_millis_seconds = datetime.as_millis();
    // println!("total_millis_seconds {}", total_millis_seconds);

    // 计算总秒数中的秒数部分
    let seconds = total_seconds % 60;
    // 计算总秒数中的分钟部分
    let minutes = (total_seconds / 60) % 60;
    // 计算总秒数中的小时部分
    let hours = (total_seconds / 3600) % 24;
    // 计算自 UNIX 时间原点以来的天数
    let days_since_epoch = total_seconds / 86400;

    // 将天数转换为整数类型
    let days = days_since_epoch as i64;

    // 使用自定义函数计算年、月、日的元组
    let (year, month, day) = days_to_ymd(days);

    // 使用格式化宏将年、月、日、小时、分钟、秒格式化为字符串
    format!("{:04}-{:02}-{:02} {:02}:{:02}:{:02}", year, month, day, hours, minutes, seconds)
}

fn days_to_ymd(days: i64) -> (i32, u32, u32) {
    // 初始化年份为 1970 年
    let mut year = 1970;
    // 将传入的天数赋值给可变变量
    let mut days = days;

    // 循环直到天数小于一年的天数（365 或 366）
    while days >= 365 {
        // 判断当前年份是否为闰年
        if is_leap_year(year) {
            // 如果是闰年且天数小于 366，则跳出循环
            if days < 366 {
                break;
            }
            // 减去闰年的天数
            days -= 366;
        } else {
            // 减去平年的天数
            days -= 365;
        }
        // 年份加一
        year += 1;
    }

    // 初始化月份为 1 月
    let mut month = 1;
    // 每个月的天数数组，闰年二月为 29 天，平年为 28 天
    let days_in_month = [31, if is_leap_year(year) { 29 } else { 28 }, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

    // 循环直到天数小于当前月份的天数
    while days >= days_in_month[month as usize - 1] {
        // 减去当前月份的天数
        days -= days_in_month[month as usize - 1];
        // 月份加一
        month += 1;
    }
    // 返回年、月、日的元组，月份和日从 1 开始计数
    (year, month, days as u32 + 1)
}

// 判断是否为闰年的函数
fn is_leap_year(year: i32) -> bool {
    // 闰年的判断条件：能被 4 整除但不能被 100 整除，或者能被 400 整除
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}