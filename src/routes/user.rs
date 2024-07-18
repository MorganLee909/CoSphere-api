use actix_web::{web, Responder};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/user")
            .route(web::get().to(create))
    );
}

async fn create() -> impl Responder {
    "Create new user route"
}
