use anyhow::Result;
use serde::{Serialize, Deserialize};
use sqlx::{PgPool, postgres::{PgRow, PgQueryResult}, Row};

#[derive(Serialize, Deserialize)]
pub struct PostRequest {
    pub body: String,
    pub user_id: i32,
}

#[derive(Serialize, Deserialize)]
pub struct Post {
    pub id: i32,
    pub user_id: i32,
    pub body: String,
    pub created: chrono::NaiveDateTime,
    pub reply_id: Vec<i32>,
}

impl Post {
    pub async fn get_top_ten(pool: &PgPool) -> Result<Vec<Post>> {
        let posts: Vec<Post> = sqlx::query_as!(
            Post,
            r#"
                SELECT DISTINCT * FROM posts
                ORDER BY created
                FETCH FIRST 10 ROWS ONLY 
            "#
        )
            .fetch_all(pool)
            .await?;

        Ok(posts)
    }

    pub async fn create_post(post: PostRequest, pool: &PgPool) -> Result<PgQueryResult> {
        let mut table = pool.begin().await?;
        let created = sqlx::query("
            INSERT INTO posts (body, user_id) values ($1, $2) RETURNING *
        ")
            .bind(post.body)
            .bind(post.user_id)
            .execute(&mut table)
            .await?;

        table.commit().await?;
        Ok(created)
    }

    pub async fn delete_post(post_id: i32, pool: &PgPool) -> Result<PgQueryResult> {
        let mut table = pool.begin().await?;
        let deleted = sqlx::query("
            DELETE FROM posts WHERE id = $1
        ")
            .bind(post_id)
            .execute(&mut table)
            .await?;

        table.commit().await?;
        Ok(deleted)
    }

    pub async fn get_user_id(post_id: i32, pool: &PgPool) -> Result<i32> {
        let id = sqlx::query!(
            r#"
                SELECT user_id FROM posts
                WHERE id = $1
            "#,
            post_id
        )
            .fetch_one(pool)
            .await?;

        Ok(id.user_id)
    }
}