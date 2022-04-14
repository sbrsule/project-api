use std::env::var;
use actix_web::{HttpServer, App};
use sqlx::postgres::PgPoolOptions;
use routes::user::init as user_init;

mod models;
mod routes;

#[allow(deprecated)]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let db_url = var("DATABASE_URL").expect("Unable to read DATABASE_URL variable in .env file!");
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&db_url)
        .await
        .expect("Unable to create database pool");

    HttpServer::new(move || {
        let cors = actix_cors::Cors::permissive();

        App::new()
            .wrap(cors)
            .data(pool.clone())
            .configure(user_init)
    })
        .bind(format!("{}:{}", var("HOST").unwrap(), var("PORT").unwrap()))?
        .run()
        .await
}
