use std::env::var;
use actix_web::{HttpServer, App};
use sqlx::postgres::PgPoolOptions;

mod models;
mod password;

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
    })
        .bind(format!("{}:{}", var("HOST").unwrap(), var("PORT").unwrap()))?
        .run()
        .await
}
