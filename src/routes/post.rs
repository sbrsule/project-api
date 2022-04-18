use actix_identity::Identity;
use actix_web::{web, HttpResponse, get, post, delete, http::header::HttpDate};
use sqlx::PgPool;

use crate::models::{post::{Post, PostRequest}, user::User};

#[post("/create_post")]
async fn create_post(id: Identity, post: web::Json<PostRequest>, pool: web::Data<PgPool>) -> HttpResponse {
    match id.identity() {
        Some(id) => {
                match Post::create_post(id.parse::<i32>().unwrap() , post.into_inner(), pool.as_ref()).await {
                        Ok(_) => HttpResponse::Created().finish(),
                        Err(_) => HttpResponse::BadRequest().finish(),
                    }
                }
                None => HttpResponse::NotFound().finish(),
        }
    }

#[post("/create_reply/{id}")]
async fn create_reply(id: Identity, post: web::Json<PostRequest>, parent_id: web::Path<i32>, pool: web::Data<PgPool>) -> HttpResponse {
    match id.identity() {
        Some(id) => {
                match Post::create_reply(id.parse::<i32>().unwrap(), post.into_inner(), pool.as_ref()).await {
                        Ok(reply_id) => match Post::link_reply(parent_id.into_inner(), reply_id, pool.as_ref()).await {
                            Ok(_) => HttpResponse::Created().finish(),
                            Err(_) => HttpResponse::BadRequest().finish(),
                        }
                        Err(_) => HttpResponse::BadRequest().finish(),
                    }
                }
        None => HttpResponse::BadRequest().finish()
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

#[get("/post/{id}")]
async fn get_post(post_id: web::Path<i32>, pool: web::Data<PgPool>) -> HttpResponse {
    match Post::get_post(post_id.into_inner(), pool.as_ref()).await {
        Ok(post) => HttpResponse::Ok().json(post),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

#[get("/replies/{id}")]
async fn get_replies(post_id: web::Path<i32>, pool: web::Data<PgPool>) -> HttpResponse {
    let replies = Post::get_replies(post_id.into_inner(), pool.as_ref()).await;
    match replies {
        Ok(replies) => HttpResponse::Ok().json(replies),
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
    cfg.service(create_reply);
    cfg.service(get_top_ten);
    cfg.service(delete_post);
    cfg.service(get_replies);
    cfg.service(get_post);
}