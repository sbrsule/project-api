use serde::{Serialize, Deserialize};
use sqlx::{PgPool, postgres::PgQueryResult};
use anyhow::Result;

#[derive(Serialize, Deserialize)]
pub struct TweetRequest {
    pub body: String,
    pub parent_id: Option<i64>,
    pub user_id: i32,
}

#[derive(Serialize, Deserialize)]
pub struct Tweet {
    pub id: i64,
    pub body: String,
    pub parent_id: Option<i64>,
    pub created: chrono::NaiveDateTime,
    pub user_id: i32,
}

impl Tweet {
    pub async fn find_all(pool: &PgPool) -> Result<Vec<Tweet>> {
        let mut tweets = vec![];
        let records = sqlx::query!(
            r#"
                SELECT * FROM tweets
                ORDER BY created
            "#
        )
            .fetch_all(pool)
            .await?;

        for rec in records {
            tweets.push(Tweet {
                id: rec.id,
                body: rec.body,
                parent_id: rec.parent_id, 
                created: rec.created,
                user_id: rec.user_id,
            });
        }

        Ok(tweets)
    }

    pub async fn find_by_user(user_id: i32, pool: &PgPool) -> Result<Vec<Tweet>> {
        let mut tweets = vec![];
        let records = sqlx::query!(
            r#"
                SELECT * FROM tweets WHERE user_id = $1
                ORDER BY created
            "#,
            user_id,
        )
            .fetch_all(pool)
            .await?;

        for rec in records {
            tweets.push(Tweet {
                id: rec.id,
                body: rec.body,
                parent_id: rec.parent_id,
                created: rec.created,
                user_id: rec.user_id,
            })
        }

        Ok(tweets)
    }
    
    pub async fn find_by_id(id: i64, pool: &PgPool) -> Result<Tweet> {
        let mut tweet = sqlx::query_as!(
            Tweet,
            r#"
                SELECT * FROM tweets WHERE id = $1
            "#,
            id,
        )
            .fetch_one(pool)
            .await?;

        Ok(tweet)
    }

    pub async fn delete(id: i64, pool: &PgPool) -> Result<PgQueryResult> {
        let mut table = pool.begin().await?;
        let deleted = sqlx::query("DELETE FROM tweets WHERE id = $1")
            .bind(id)
            .execute(&mut table)
            .await?;

        table.commit().await?;
        Ok(deleted)
    }

    pub async fn parents(pool: &PgPool) -> Result<Vec<Tweet>> {
        let tweets = sqlx::query_as!(
            Tweet,
            r#"
                SELECT * FROM tweets WHERE parent_id IS NULL
                ORDER BY created
            "#
        )
            .fetch_all(pool)
            .await?;

        Ok(tweets)
    }

    pub async fn get_children(parent_id: i64, pool: &PgPool) -> Result<Vec<Tweet>> {
        let tweets = sqlx::query_as!(
            Tweet,
            r#"
                SELECT * FROM tweets WHERE parent_id = $1
                ORDER BY created
            "#,
            parent_id,
        )
            .fetch_all(pool)
            .await?;

        Ok(tweets)
    }

    pub async fn get_parent(parent_id: i64, pool: &PgPool) -> Result<Tweet> {
        let tweet = sqlx::query_as!(
            Tweet,
            r#"
                SELECT * FROM tweets WHERE id = $1
            "#,
            parent_id,
        )
            .fetch_one(pool)
            .await?;

        Ok(tweet)
    }
}