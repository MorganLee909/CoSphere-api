use crate::models::{user::User};
use crate::errors::http_error;
use actix_web::{web, http::header::HeaderValue, HttpResponse, HttpRequest};
use mongodb::{bson::doc, Client, Collection};
use serde::Deserialize;
use bson::oid::ObjectId;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/user/login")
            .route(web::post().to(login))
    );
    cfg.service(
        web::resource("/user/{user_id}")
            .route(web::get().to(retrieve))
            .route(web::put().to(update))
    );
    cfg.service(
        web::resource("/user")
            .route(web::post().to(create))
    );
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

    let token = user.create_token();

    HttpResponse::Ok().json(token)
}

async fn retrieve(
        client: web::Data<Client>,
        user_id: web::Path<String>,
        req: HttpRequest
    ) -> HttpResponse {
        let id = user_id.to_string();
        let auth_header = req.headers().get("Authorization");
        match user_authorized(client, id, auth_header).await {
            Ok(u) => HttpResponse::Ok().json(u.response_user()),
            Err(e) => http_error(500, e.to_string())
        }
}

#[derive(Deserialize)]
struct UpdateBody {
    location: Option<String>,
    first_name: Option<String>,
    last_name: Option<String>
}

async fn update(
        client: web::Data<Client>,
        user_id: web::Path<String>,
        body: web::Json<UpdateBody>,
        req: HttpRequest
    ) -> HttpResponse {
        let user_collection : Collection<User> = client.database("cosphere").collection("users");

        let id = user_id.to_string();
        let auth_header = req.headers().get("Authorization");
        let user = match user_authorized(client, id, auth_header).await {
            Ok(u) => u,
            Err(e) => return http_error(500, e.to_string())
        };

        let data = vec![
            (String::from("location"), &body.location),
            (String::from("first_name"), &body.first_name),
            (String::from("last_name"), &body.last_name),
        ];

        let document = User::create_update_doc(data);
        let res = user_collection.update_one(doc! { "_id": user._id}, document).await;
        match res {
            Ok(u) => HttpResponse::Ok().json(u),
            Err(e) => http_error(500, e.to_string())
        }
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

    let mut user = match User::new(
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
    };

    user.stripe = Some(user.create_stripe_data().await);

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

async fn user_authorized(
        client: actix_web::web::Data<Client>,
        user_id: String,
        auth_header: Option<&HeaderValue>
    ) -> Result<User, Box<dyn std::error::Error>> {
    let user_collection: Collection<User> = client.database("cosphere").collection("users");

    let id = ObjectId::parse_str(user_id).unwrap();
    let user = match user_collection.find_one(doc! { "_id": id }).await {
        Ok(Some(u)) => u,
        Ok(None) => return Err(Box::from("User with this ID deosn't exist")),
        Err(_) => return Err(Box::from("Internal server error"))
    };

    let token = match auth_header {
        Some(h) => String::from(h.to_str().unwrap()),
        None => return Err(Box::from("Unauthorized"))
    };

    match user.authorized(&token) {
        true => Ok(user),
        false => Err(Box::from("Unauthorized"))
    }
}
