use actix_identity::Identity;
<<<<<<< HEAD
use actix_web::{post, web, HttpResponse};
use sqlx::PgPool;

use crate::models::user::{User,UserRequest};
use crate::models::password;


#[post("/register")]
async fn create(user: web::Json<UserRequest>, pool: web::Data<PgPool>) -> HttpResponse {
    let user = User::create(user.into_inner(), pool.as_ref()).await;
    match user {
        Ok(_) => HttpResponse::Created().finish(),
=======
use actix_web::{post, web, HttpResponse, get};
use sqlx::PgPool;

use crate::models::user::{User, UserRequest, UserID};

#[post("/register")]
async fn register_user(id: Identity, user: web::Json<UserRequest>, pool: web::Data<PgPool>) -> HttpResponse {
    match User::create(user.into_inner(), pool.as_ref()).await {
        Ok(user) => {
            id.remember(user.username);
            HttpResponse::Created().finish()
        }
>>>>>>> newdb
        Err(_) => HttpResponse::BadRequest().finish(),
    }
}

#[post("/login")]
<<<<<<< HEAD
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
=======
async fn login_user(id: Identity, user: web::Json<UserRequest>, pool: web::Data<PgPool>) -> HttpResponse {
    let username = user.username.clone();
    let password = user.password.clone();
    match User::get_id(username, pool.as_ref()).await {
        Ok(user_id) => {
            let user_id = user_id;
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
        Err(_) => {
            return HttpResponse::NotFound().finish();
        }
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

#[post("/get_username")]
async fn get_username(user_id: web::Json<UserID>, pool: web::Data<PgPool>) -> HttpResponse {
    match User::get_username(user_id.into_inner(), pool.as_ref()).await {
        Ok(username) => HttpResponse::Ok().json(username),
>>>>>>> newdb
        Err(_) => HttpResponse::NotFound().finish()
    }
}

<<<<<<< HEAD
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
=======
pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(register_user);
    cfg.service(login_user);
    cfg.service(test);
    cfg.service(logout_user);
    cfg.service(get_username);
>>>>>>> newdb
}