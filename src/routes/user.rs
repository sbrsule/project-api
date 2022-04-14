use actix_web::{post, web, HttpResponse};
use sqlx::PgPool;

use crate::models::user::{User, UserRequest};

#[post("/register")]
async fn register_user(user: web::Json<UserRequest>, pool: web::Data<PgPool>) -> HttpResponse {
    let user = User::create(user.into_inner(), pool.as_ref()).await;
    match user {
        Ok(_) => HttpResponse::Created().finish(),
        Err(_) => HttpResponse::BadRequest().finish(),
    }
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(register_user);
}