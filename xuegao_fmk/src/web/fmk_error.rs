use std::backtrace::Backtrace;

use actix_web::{error, http::StatusCode, HttpResponse};
use derive_more::Display;
use serde::Serialize;
use crate::web::fmk_result::FmkR;

#[derive(Debug, Display, Serialize)]
pub enum FmkErrorEnum {
    #[display(fmt = "validate error on field: {}", field)]
    ValidationError { field: String },
    #[display(fmt = "数据库处理异常")]
    DBError(String),
    #[allow(dead_code)]
    #[display(fmt = "服务器异常")]
    ServerError(String),
}

#[derive(Debug, Serialize)]
pub struct FmkErrorResponse {
    error_message: String,

    backtrace: String,
}

impl FmkErrorEnum {
    fn error_response(&self) -> FmkErrorResponse {
        let backtrace = Backtrace::capture();
        let error_message = match self {
            FmkErrorEnum::ValidationError { field } => {
                println!("Validation Error {:?}", field);
                format!("Validation Error {}", field)
            }
            FmkErrorEnum::DBError(msg) => {
                println!("DBError Error {:?}", msg);
                msg.clone()
            }
            FmkErrorEnum::ServerError(msg) => {
                println!("Server Error {:?}", msg);
                msg.clone()
            }
        };
        eprintln!("error_message backtrace {:#}", backtrace);
        FmkErrorResponse {
            error_message,
            backtrace: "".to_string(),
        }
    }
}


impl error::ResponseError for FmkErrorEnum {
    fn status_code(&self) -> StatusCode {
        match self {
            FmkErrorEnum::ValidationError { field } => {
                println!("ValidationError Error: {:?}", field);
                StatusCode::INTERNAL_SERVER_ERROR
            }
            FmkErrorEnum::ServerError(msg) | FmkErrorEnum::DBError(msg) => {
                println!("Server Error: {:?}", msg);
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }

    fn error_response(&self) -> HttpResponse {
        let error_response = self.error_response();
        eprintln!("error_response: {:?}", error_response.error_message);
        // eprintln!("error_response.backtrace: {:#}", error_response.backtrace);

        return FmkR::<()>::err_msg(self.error_response().error_message);
    }
}
