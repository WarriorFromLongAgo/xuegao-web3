use chrono::{DateTime, Local, NaiveDateTime, Utc};

pub fn now_utc() -> DateTime<Utc> {
    // 获取当前时间并打印
    let now = Utc::now();
    // 打印当前时间
    println!("Current UTC time: {}", now);
    return now;
}

pub fn now() -> DateTime<Local> {
    // 获取当前UTC时间
    let now = Utc::now();
    // 转换为本地时间（北京时区）
    let beijing_time = now.with_timezone(&Local);
    // 打印当前北京时区时间
    println!("Current Beijing time: {}", beijing_time);
    return beijing_time;
}


pub fn now_time_utc() -> NaiveDateTime {
    // 获取当前时间并打印
    let now = Utc::now().naive_utc();
    // 打印当前时间
    println!("Current NaiveDateTime: {}", now);

    return now;
}

pub fn now_time() -> NaiveDateTime {
    // 获取当前UTC时间
    let now = Utc::now();
    // 转换为本地时间（北京时区）
    let beijing_time = now.with_timezone(&Local);
    // 转换为NaiveDateTime
    let naive_beijing_time = beijing_time.naive_local();
    // 打印当前北京时区NaiveDateTime
    println!("Current NaiveDateTime in Beijing: {}", naive_beijing_time);

    return naive_beijing_time;
}

pub fn now_utc_as_string() -> String {
    // 获取当前时间并转换为字符串
    let now = Utc::now();
    let now_string = now.to_string();
    // 打印当前时间字符串
    println!("Current time as string: {}", now_string);
    now_string
}

pub fn now_as_string(now: DateTime<Local>) -> String {
    // 获取当前时间并转换为字符串
    let now_string = now.to_string();
    // 打印当前时间字符串
    println!("Current time in Beijing  as string: {}", now_string);
    now_string
}

pub fn now_time_utc_as_string() -> String {
    // 获取当前 NaiveDateTime 并转换为字符串
    let now = Utc::now().naive_utc();
    let now_string = now.to_string();
    // 打印当前时间字符串
    println!("Current NaiveDateTime as string: {}", now_string);
    now_string
}

pub fn now_time_as_string(now: NaiveDateTime) -> String {
    // 获取当前 NaiveDateTime 并转换为字符串
    // let now = Utc::now().naive_utc();
    let now_string = now.to_string();
    // 打印当前时间字符串
    println!("Current NaiveDateTime in Beijing as string: {}", now_string);
    now_string
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_time() {
        // 获取并打印当前时间
        let current_time = now();
        println!("Current beijing time: {}", current_time);

        // 获取并打印当前 NaiveDateTime
        let current_naive_time = now_time();
        println!("Current NaiveDateTime beijing: {}", current_naive_time);

        // 获取并打印当前时间字符串
        let current_time_str = now_as_string(current_time);
        println!("Current time beijing as string: {}", current_time_str);

        // 获取并打印当前 NaiveDateTime 字符串
        let current_naive_time_str = now_time_as_string(current_naive_time);
        println!("Current NaiveDateTime beijing as string: {}", current_naive_time_str);
    }
}

