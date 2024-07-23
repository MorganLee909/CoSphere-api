use actix_web::{HttpResponse};
use serde::{Serialize};
use regex::Regex;

#[derive(Serialize)]
struct ErrorMessage {
    error: bool,
    code: i16,
    message: String
}

pub fn email_valid(email: &String) -> bool {
    let regex = Regex::new(r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})").unwrap();

    regex.is_match(email)
}

pub fn create_error(code: i16, message: String) -> HttpResponse {
    let mut request = match code {
        400 => HttpResponse::BadRequest(),
        500 => HttpResponse::InternalServerError(),
        _ => HttpResponse::InternalServerError()
    };

    request.json(ErrorMessage {
        error: true,
        code: code,
        message: message
    })
}
