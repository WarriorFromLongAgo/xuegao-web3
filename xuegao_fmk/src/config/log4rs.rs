pub fn init_logger()  {
    // 初始化 log4rs
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();
}
