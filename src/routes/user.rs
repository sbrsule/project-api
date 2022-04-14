use actix_identity::Identity;
use actix_web::{post, web, HttpResponse};
use sqlx::PgPool;

use crate::models::user::{User, UserRequest};

#[post("/register")]
async fn register_user(id: Identity, user: web::Json<UserRequest>, pool: web::Data<PgPool>) -> HttpResponse {
    match User::create(user.into_inner(), pool.as_ref()).await {
        Ok(user) => {
            id.remember(user.username);
            HttpResponse::Created().finish()
        }
        Err(_) => HttpResponse::BadRequest().finish(),
    }
}

#[post("/login")]
async fn login_user(id: Identity, user: web::Json<UserRequest>, pool: web::Data<PgPool>) -> HttpResponse {
    let username = user.username.clone();
    let password = user.password.clone();
    match User::get_password(user.into_inner(), pool.get_ref()).await {
        Ok(password_hash) => {
            match crate::models::password::verify_password(password, password_hash) {
                true => {
                    id.remember(username);
                    HttpResponse::Accepted().finish()
                }
                false => HttpResponse::Unauthorized().finish()
            }
        }
        Err(_) => HttpResponse::NotFound().finish()
    }
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(register_user);
    cfg.service(login_user);
}