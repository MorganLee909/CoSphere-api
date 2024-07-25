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
    cfg.service(
        web::resource("/user/login")
            .route(web::post().to(login))
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
    ).await{
        Ok(user) => user,
        Err(err) => return http_error(err.0, err.1)
    };

    match user_collection.find_one(doc! { "email": &user.email }).await {
        Ok(Some(_)) => return http_error(400, String::from("User with this email already exists")),
        Ok(None) => (),
        Err(_) => return http_error(500, String::from("Internal server error"))
    };

    let collection: Collection<User> = client.database("cosphere").collection("users");
    match collection.insert_one(&user).await {
        Ok(_) => HttpResponse::Ok().json(user.response_user()),
        Err(_) => http_error(500, String::from("Internal server error"))
    }
}

#[derive(Deserialize)]
struct LoginBody {
    email: String,
    password: String
}

async fn login(client: web::Data<Client>, body: web::Json<LoginBody>) -> HttpResponse {
    let user_collection: Collection<User> = client.database("cosphere").collection("users");

    let user = match user_collection.find_one(doc! { "email": &body.email }).await {
        Ok(Some(u)) => u,
        Ok(None) => return http_error(401, String::from("No user with this email address")),
        Err(e) => return http_error(500, e.to_string())
    };

    match user.valid_password(body.password.clone()) {
        true => (),
        false => return http_error(401, String::from("Invalid password"))
    };

    //let token = user.create_token();

    HttpResponse::Ok().json(user)
}
