use actix_http::StatusCode;
use actix_web::http::header::ContentType;
use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};

// 自定义 Response
#[derive(Deserialize, Serialize, Debug)]
pub struct FmkR<T> {
    pub code: i32,
    pub msg: String,
    pub data: Option<T>,
}

// 在 R<T> 结构体附近定义辅助方法
impl<T> FmkR<T>
where
    T: Serialize,
{
    // 创建成功的响应
    pub fn success(data: T) -> HttpResponse {
        let r: FmkR<T> = FmkR {
            code: StatusCode::OK.as_u16() as i32,
            msg: "请求成功".to_string(),
            data: Some(data),
        };
        HttpResponse::build(StatusCode::OK)
            .insert_header(ContentType::json())
            .json(r)
    }

    // 创建失败的响应
    pub fn err() -> HttpResponse {
        let r: FmkR<()> = FmkR {
            code: StatusCode::INTERNAL_SERVER_ERROR.as_u16() as i32,
            msg: "服务器异常".to_string(),
            data: None,
        };
        HttpResponse::build(StatusCode::OK)
            .insert_header(ContentType::json())
            .json(r)
    }

    // 创建失败的响应
    pub fn err_msg(msg: String) -> HttpResponse {
        let r: FmkR<()> = FmkR {
            code: StatusCode::INTERNAL_SERVER_ERROR.as_u16() as i32,
            msg,
            data: None,
        };
        HttpResponse::build(StatusCode::OK)
            .insert_header(ContentType::json())
            .json(r)
    }

    // 创建自定义状态码的响应
    pub fn custom(code: i32, msg: String, data: Option<T>) -> HttpResponse {
        let r: FmkR<T> = FmkR {
            code,
            msg,
            data,
        };
        HttpResponse::build(StatusCode::OK)
            .insert_header(ContentType::json())
            .json(r)
    }
}

// Responder
// impl<T> Responder for R<T>
// where
//     T: Serialize,
// {
//     type Body = BoxBody;
//
//     fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
//         eprintln!("dada {:?}", _req);
// dada
// HttpRequest HTTP/1.1 POST:/return_json
// query: ?"aa=11"
// headers:
// "origin": "chrome-extension://haklpcemfcccpoeaibpbgacinnbfafbl"
// "sec-fetch-mode": "cors"
// "content-type": "text/plain"
// "referrer-policy": "no-referrer, strict-origin-when-cross-origin"
// "connection": "keep-alive"
// "sec-fetch-dest": "empty"
// "accept-language": "zh-CN,zh;q=0.9"
// "user-agent": "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0.0.0 Safari/537.36 Edg/126.0.0.0"
// "accept": "*/*"
// "accept-encoding": "gzip, deflate, br, zstd"
// "host": "127.0.0.1:9000"
// "content-length": "0"
// "sec-fetch-site": "none"

// let body = serde_json::to_string(&self).unwrap();
// return HttpResponse::Ok()
//     .content_type("application/json")
//     .body(body);
// }

// fn customize(self) -> CustomizeResponder<Self>
// where
//     Self: Sized
// {
//     CustomizeResponder::new(self)
// }
//
// fn with_status(self, status: StatusCode) -> CustomizeResponder<Self>
// where
//     Self: Sized
// {
//     CustomizeResponder::with_status(self, status)
// }
//
// fn with_header(self, header: impl TryIntoHeaderPair) -> CustomizeResponder<Self>
// where
//     Self: Sized
// {
//     CustomizeResponder::with_header(self, header)
// }
// }

// use futures::future::{ready, Ready};
// Responder
// impl<T> Responder for R<T>
// where
//     T: Serialize,
// {
//     type Body = BoxBody;
//     type Error = Error;
//     type Future = Ready<Result<HttpResponse, Error>>;
//
//     fn respond_to(self, _req: &HttpRequest) -> Self::Future {
//         let body = serde_json::to_string(&self).unwrap();
//
//         // Create response and set content type
//         ready(Ok(HttpResponse::Ok()
//             .content_type("application/json")
//             .body(body)))
//     }
// }