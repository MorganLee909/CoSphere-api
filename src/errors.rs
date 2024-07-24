use actix_web::{HttpResponse};
use serde::{Serialize};

#[derive(Serialize)]
struct HttpError {
    error: bool,
    code: i16,
    message: String
}

pub fn http_error(code: i16, message: String) -> HttpResponse {
    let mut request = match code {
        400 => HttpResponse::BadRequest(),
        401 => HttpResponse::Unauthorized(),
        500 => HttpResponse::InternalServerError(),
        _ => HttpResponse::InternalServerError()
    };

    request.json(HttpError {
        error: true,
        code: code,
        message: message
    })
}
