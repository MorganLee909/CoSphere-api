use crate::models;
use actix_web::{web, Responder};
use chrono::Utc;
use mongodb::Client;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/user")
            .route(web::get().to(create))
    );
}

async fn create(client: web::Data<Client>) -> impl Responder {
    let new_user = models::user::User {
        email: String::from("lee@leemorgan.dev"),
        password: String::from("password123"),
        first_name: String::from("Lee"),
        last_name: String::from("Morgan"),
        status: String::from("active"),
        expiration: Utc::now(),
        created_date: Utc::now(),
        reset_code: None,
        avatar: None,
        default_location: String::from("Myrtle Beach"),
        session_id: String::from("12345")
    };

    let collection = client.database("cosphere").collection("users");
    let result = collection.insert_one(new_user).await;
    match result {
        Ok(_) => "New user created",
        Err(err) => "Error: user probably not created"
    }
}
