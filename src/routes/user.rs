use actix_identity::Identity;
use actix_web::{post, web, HttpResponse};
use sqlx::PgPool;

use crate::models::user::{User,UserRequest};
use crate::models::password;


#[post("/register")]
async fn create(user: web::Json<UserRequest>, pool: web::Data<PgPool>) -> HttpResponse {
    let user = User::create(user.into_inner(), pool.as_ref()).await;
    match user {
        Ok(_) => HttpResponse::Created().finish(),
        Err(_) => HttpResponse::BadRequest().finish(),
    }
}

#[post("/login")]
async fn login(id: Identity, user: web::Json<UserRequest>, pool: web::Data<PgPool>) -> HttpResponse {
    let username = user.username.clone();
    let password = user.password.clone();
    let password_hash= User::get_password(user.into_inner(), pool.get_ref()).await;
    match password_hash {
        Ok(password_hash) => {
            match password::verify_password(password, password_hash) {
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

#[post("/isloggedin")]
async fn is_logged_in(id: Identity) -> HttpResponse {
    match id.identity() {
        Some(_) => HttpResponse::Accepted().finish(),
        None => HttpResponse::BadRequest().finish(),
    }
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(create);
    cfg.service(login);
    cfg.service(is_logged_in);
}