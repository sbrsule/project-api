use actix_identity::Identity;
use actix_web::{web, HttpResponse, get, post, delete, http::header::HttpDate};
use sqlx::PgPool;

use crate::models::post::{Post, PostRequest};

#[post("/create_post")]
async fn create_post(id: Identity, post: web::Json<PostRequest>, pool: web::Data<PgPool>) -> HttpResponse {
    match id.identity() {
        Some(id) => {
            let user_id= id.parse::<i32>().expect("unable to parse id");
            match Post::create_post(user_id, post.into_inner(), pool.as_ref()).await {
                Ok(_) => HttpResponse::Created().finish(),
                Err(_) => HttpResponse::BadRequest().body("unable to post"),
            }
        }
        None => HttpResponse::Unauthorized().body("unable to get identity"),
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

#[delete("/post")]
async fn delete_post(id: Identity, post_id: web::Json<i32>, pool: web::Data<PgPool>) -> HttpResponse {
    let user_id = Post::get_user_id(post_id.clone(), pool.as_ref()).await;
    match user_id {
        Ok(user_id) => user_id,
        Err(_) => {
            return HttpResponse::NotFound().finish();
        }
    };

    match id.identity() {
        Some(id) => match id.parse::<i32>().unwrap() {
            user_id => match Post::delete_post(post_id.clone(), pool.as_ref()).await {
                Ok(_) => HttpResponse::NoContent().finish(),
                Err(_) => HttpResponse::NotFound().finish(),
            },
            _ => HttpResponse::Unauthorized().finish(),
        }
        None => HttpResponse::BadRequest().finish(),
    }
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(create_post);
    cfg.service(get_top_ten);
    cfg.service(delete_post);
}