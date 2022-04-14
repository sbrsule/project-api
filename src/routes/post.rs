use actix_web::{web, HttpResponse, get, post};
use sqlx::PgPool;

use crate::models::post::{Post, PostRequest};

#[post("/create_post")]
async fn create_post(post: web::Json<PostRequest>, pool: web::Data<PgPool>) -> HttpResponse {
    let post = Post::create_post(post.into_inner(), pool.as_ref()).await;
    match post {
        Ok(_) => HttpResponse::Created().finish(),
        Err(_) => HttpResponse::BadRequest().finish(),
    }
}

#[get("/posts")]
async fn get_top_ten(pool: web::Data<PgPool>) -> HttpResponse {
    let posts = Post::get_top_ten(pool.as_ref()).await;
    match posts {
        Ok(posts) => HttpResponse::Ok().json(posts),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(create_post);
    cfg.service(get_top_ten);
}