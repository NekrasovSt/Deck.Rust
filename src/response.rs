use actix_web::error;
use actix_web::http::StatusCode;

pub struct Response<T> {
    body: T,
    error: bool,
}

// impl<T> error::ResponseError for Response<T> {
//     fn error_response(&self) -> Response<String> {
//
//     }
//     fn status_code(&self) -> StatusCode {
//         if self.error {
//             StatusCode::BAD_REQUEST
//         } else {
//             StatusCode::OK
//         }
//     }
// }