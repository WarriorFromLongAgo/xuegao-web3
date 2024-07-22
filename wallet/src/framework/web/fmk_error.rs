use actix_web::{error, http::{header::ContentType, StatusCode}, HttpResponse};
use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
pub enum FmkErrorEnum {
    #[display(fmt = "internal error")]
    InternalError,
    #[display(fmt = "bad request")]
    BadClientData,
    #[display(fmt = "timeout")]
    Timeout,
    #[display(fmt = "vaildation error on filed:{}", filed)]
    ValidationError { filed: String },
}

impl error::ResponseError for FmkErrorEnum {
    fn status_code(&self) -> StatusCode {
        match *self {
            FmkErrorEnum::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            FmkErrorEnum::BadClientData => StatusCode::BAD_REQUEST,
            FmkErrorEnum::Timeout => StatusCode::GATEWAY_TIMEOUT,
            FmkErrorEnum::ValidationError { .. } => StatusCode::BAD_REQUEST
        }
    }
    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }
}
