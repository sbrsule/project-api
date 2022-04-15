use actix_identity::Identity;
use actix_web::{post, web, HttpResponse, get};
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
    let user_id = User::get_id(username, pool.get_ref()).await.expect("Could'nt find user id");
    match User::get_password(user.into_inner(), pool.get_ref()).await {
        Ok(password_hash) => {
            match crate::models::password::verify_password(password, password_hash) {
                true => {
                    id.remember(user_id.to_string());
                    HttpResponse::Accepted().finish()
                }
                false => HttpResponse::Unauthorized().finish()
            }
        }
        Err(_) => HttpResponse::NotFound().finish()
    }
}

#[post("/test")]
async fn test(id: Identity) -> HttpResponse {
    match id.identity() {
        Some(_) => HttpResponse::Ok().finish(),
        None => HttpResponse::Forbidden().finish(),
    }
}

#[post("/logout")]
async fn logout_user(id: Identity) -> HttpResponse {
    match id.identity() {
        Some(_) => {
            id.forget();
            HttpResponse::Ok().finish()
        }
        None => HttpResponse::NotFound().finish()
    }
}



pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(register_user);
    cfg.service(login_user);
    cfg.service(test);
    cfg.service(logout_user);
}