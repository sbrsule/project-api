use std::env::var;
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{HttpServer, App};
use sqlx::PgPool;
use routes::user::init as user_init;
use routes::post::init as post_init;

mod models;
mod routes;

#[allow(deprecated)]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let db_url = var("DATABASE_URL").expect("Unable to read DATABASE_URL variable in .env file!");
    let pool = PgPool::connect_lazy(&db_url)
        .expect("Unable to create database pool");

    HttpServer::new(move || {
        let cors = actix_cors::Cors::permissive();

        App::new()
        .wrap(IdentityService::new(
            CookieIdentityPolicy::new(&[0; 32])
            .name("auth")
            .secure(false)
        ))
            .wrap(cors)
            .data(pool.clone())
            .configure(user_init)
            .configure(post_init)
    })
        .bind(format!("{}:{}", var("HOST").unwrap(), var("PORT").unwrap()))?
        .run()
        .await
}