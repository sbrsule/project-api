use std::env::var;
<<<<<<< HEAD
use actix_identity::{IdentityService, CookieIdentityPolicy};
=======
use actix_identity::{CookieIdentityPolicy, IdentityService};
>>>>>>> newdb
use actix_web::{HttpServer, App};
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
<<<<<<< HEAD
use crate::routes::user::init as user_init;
=======
use routes::user::init as user_init;
use routes::post::init as post_init;
>>>>>>> newdb

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
<<<<<<< HEAD
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(&[0; 32])
                .name("auth")
                .secure(false)
            ))
            .wrap(cors)
            .data(pool.clone())
            .configure(user_init)
=======
        .wrap(IdentityService::new(
            CookieIdentityPolicy::new(&[0; 32])
            .name("auth")
            .secure(false)
        ))
            .wrap(cors)
            .data(pool.clone())
            .configure(user_init)
            .configure(post_init)
>>>>>>> newdb
    })
        .bind(format!("{}:{}", var("HOST").unwrap(), var("PORT").unwrap()))?
        .run()
        .await
}