use actix_web::{HttpResponse};
use serde::{Serialize};
use regex::Regex;

#[derive(Serialize)]
struct ErrorMessage<'a> {
    error: bool,
    code: i16,
    message: &'a str
}

pub fn email_valid(email: &String) -> bool {
    let regex = Regex::new(r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})").unwrap();

    regex.is_match(email)
}

pub fn create_error(code: i16, message: &str) -> HttpResponse {
    HttpResponse::BadRequest().json(ErrorMessage {
        error: true,
        code: code,
        message: message
    })
}
