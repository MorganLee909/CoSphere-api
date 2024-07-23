use crate::models::{user::User};
use crate::errors::http_error;
use actix_web::{web, HttpResponse};
use mongodb::{bson::doc, Client, Collection};
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
    let user_collection: Collection<User> = client.database("cosphere").collection("users");

    let user = match User::new(
        body.email.clone(),
        body.password.clone(),
        body.confirm_password.clone(),
        body.first_name.clone(),
        body.last_name.clone(),
    ){
        Ok(user) => user,
        Err(err) => return http_error(err.0, err.1)
    };

    
    match user_collection.find_one(doc! { "email": &user.email }).await {
        Ok(Some(_)) => return http_error(400, String::from("User with this email already exists")),
        Ok(None) => (),
        Err(_) => return http_error(500, String::from("Internal server error"))
    }

    let collection: Collection<User> = client.database("cosphere").collection("users");
    match collection.insert_one(&user).await {
        Ok(_) => HttpResponse::Ok().json(user),
        Err(_) => http_error(500, String::from("Internal server error"))
    }
}
