use anyhow::Result;
use serde::{Serialize, Deserialize};
use sqlx::{PgPool, postgres::{PgQueryResult}};

#[derive(Serialize, Deserialize)]
pub struct PostRequest {
    pub body: String,
    pub subject: String,
}

#[derive(Serialize, Deserialize)]
pub struct Post {
    pub id: i32,
    pub user_id: i32,
    pub body: String,
    pub subject: String,
    pub created: chrono::NaiveDateTime,
    pub reply_id: Vec<i32>,
    pub reply: bool,
}

impl Post {
    pub async fn get_top_ten(pool: &PgPool) -> Result<Vec<Post>> {
        let posts: Vec<Post> = sqlx::query_as!(
            Post,
            r#"
                SELECT DISTINCT * FROM posts
                WHERE reply = false
                ORDER BY created DESC
                FETCH FIRST 10 ROWS ONLY 
            "#
        )
            .fetch_all(pool)
            .await?;

        Ok(posts)
    }

    pub async fn get_post(post_id: i32, pool: &PgPool) -> Result<Post> {
        let post = sqlx::query_as!(
            Post,
            r#"
                SELECT * FROM posts
                WHERE id = $1
            "#,
            post_id
        )
            .fetch_one(pool)
            .await?;

        Ok(post)
    }

    pub async fn create_post(user_id: i32, post: PostRequest, pool: &PgPool) -> Result<PgQueryResult> {
        let mut table = pool.begin().await?;
        let replies: Vec::<i32> = vec![];
        let created = sqlx::query("
            INSERT INTO posts (body, subject, user_id, reply_id) values ($1, $2, $3, $4) RETURNING *
        ")
            .bind(post.body)
            .bind(post.subject)
            .bind(user_id)
            .bind(replies)
            .execute(&mut table)
            .await?;

        table.commit().await?;
        Ok(created)
    }

    pub async fn create_reply(user_id: i32, post: PostRequest, pool: &PgPool) -> Result<i32> {
        let mut table = pool.begin().await?;
        let replies: Vec::<i32> = vec![];
        let created = sqlx::query!(
            r#"
            INSERT INTO posts (body, subject, user_id, reply_id, reply) 
            values ($1, $2, $3, $4, $5) RETURNING id 
            "#,
            post.body,
            post.subject,
            user_id,
            &replies,
            true,
        )
            .fetch_one(&mut table)
            .await?;

        table.commit().await?;
        Ok(created.id)
    }

    pub async fn link_reply(parent_id: i32, reply_id: i32, pool: &PgPool) -> Result<PgQueryResult> {
        let mut table = pool.begin().await?;
        let linked = sqlx::query!(
            r#"
            UPDATE posts
            SET reply_id = array_append(reply_id, $1)
            WHERE id = $2
            "#,
            reply_id,
            parent_id,
        )
            .execute(&mut table)
            .await?;

        table.commit().await?;
        Ok(linked)
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

    pub async fn get_replies(post_id: i32, pool: &PgPool) -> Result<Vec<Post>> {
        let record = sqlx::query!(
            r#"
                SELECT reply_id FROM posts
                WHERE id = $1
            "#,
            post_id
        )
            .fetch_one(pool)
            .await?;

        let reply_id = record.reply_id;
        let mut replies: Vec<Post> = vec![];

        for id in reply_id.iter() {
            let reply = sqlx::query_as!(
                Post,
                r#"
                    SELECT * FROM posts
                    WHERE id = $1
                "#,
                id
            )
                .fetch_one(pool)
                .await?;

            replies.push(reply)
        }

        Ok(replies)
    }
}