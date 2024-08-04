// use std::fs::OpenOptions;
//
// use slog::{Drain, Logger, o};
// use slog_async;
// use slog_envlogger;
// use slog_json;
// use slog_term;
//
// pub fn init_logger() -> Logger {
//     // Create terminal drain
//     let decorator = slog_term::TermDecorator::new().build();
//     let drain = slog_term::FullFormat::new(decorator).build().fuse();
//
//
//     // Create file drain
//     let file = OpenOptions::new()
//         .create(true)
//         .write(true)
//         .truncate(false)
//         .open("app.log")
//         .unwrap();
//     let file_drain = slog_json::Json::default(file).fuse();
//
//     // Combine terminal and file drains
//     let drain = slog::Duplicate::new(drain, file_drain).fuse();
//
//     // Use async drain
//     let drain = slog_async::Async::new(drain).build().fuse();
//
//     // Use envlogger
//     let drain = slog_envlogger::new(drain);
//
//     Logger::root(drain, o!())
// }
