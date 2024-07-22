use crate::models::{user::User};
use crate::controllers::user as controller;
use actix_web::{web, HttpResponse};
use chrono::Utc;
use mongodb::{Client, Collection};
use serde::Deserialize;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/user")
            .route(web::post().to(create))
    );
}

#[derive(Deserialize)]
struct CreateBody {
    email: String,
    password: String,
    confirm_password: String,
    first_name: String,
    last_name: String
}

async fn create(client: web::Data<Client>, body: web::Json<CreateBody>) -> HttpResponse {
    let email: String = body.email.to_lowercase();
    if body.password != body.confirm_password {
        return controller::create_error(400, "Passwords do not match");
    }

    if !controller::email_valid(&email) {
        return controller::create_error(400, "Invalid email");
    }

    let new_user = User {
        email: body.email.clone(),
        password: body.password.clone(),
        first_name: body.first_name.clone(),
        last_name: body.last_name.clone(),
        status: String::from("active"),
        expiration: Utc::now(),
        created_date: Utc::now(),
        reset_code: None,
        avatar: None,
        default_location: String::from("Myrtle Beach"),
        session_id: String::from("12345")
    };

    let collection:Collection<User> = client.database("cosphere").collection("users");
    let result = collection.insert_one(&new_user).await;
    match result {
        Ok(_) => HttpResponse::Ok().json(new_user),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string())
    }
}
