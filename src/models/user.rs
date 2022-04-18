use serde::{Serialize, Deserialize};
use anyhow::Result;
use sqlx::{PgPool, postgres::{PgRow, PgQueryResult}, Row};

use super::password;

#[derive(Serialize, Deserialize)]
pub struct UserRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password_hash: String,
    pub created: chrono::NaiveDateTime,
}

#[derive(Serialize, Deserialize)]
pub struct UserName {
    pub username: String,
}

#[derive(Serialize, Deserialize)]
pub struct  UserID {
    pub id: i32,
}

#[allow(dead_code)]
impl User {
    pub async fn create(user: UserRequest, pool: &PgPool) -> Result<User> {
        let mut table = pool.begin().await?;
        let password = password::hash_password(user.password).unwrap();
        let user = sqlx::query("INSERT INTO USERS (username, password_hash) values ($1, $2) RETURNING *")
            .bind(&user.username)
            .bind(&password)
            .map(|row: PgRow| {
                User {
                    id: row.get(0),
                    username: row.get(1),
                    password_hash: row.get(2),
                    created: row.get(3),
                }
            })
            .fetch_one(&mut table)
            .await?;

        table.commit().await?;
        Ok(user)
    }

    pub async fn find_by_id(id: i32, pool: &PgPool) -> Result<Option<User>> {
        let user: Option<User> = sqlx::query_as!(
            User,
            r#"
                SELECT * FROM users
                WHERE id = $1
            "#,
            id,
        )
            .fetch_optional(pool)
            .await?;

        Ok(user)
    }

    pub async fn get_username(id: UserID, pool: &PgPool) -> Result<UserName> {
        let record = sqlx::query_as!(
            UserName,
            r#"
                SELECT username FROM users WHERE id = $1
            "#,
            id.id
        )
        .fetch_one(pool)
        .await?;

        Ok(record)
    }

    pub async fn find_by_username(username: UserName, pool: &PgPool) -> Result<Option<User>> {
        let user: Option<User> = sqlx::query_as!(
            User,
            r#"
                SELECT * FROM users
                WHERE username = $1    
            "#,
            username.username,
        )
            .fetch_optional(pool)
            .await?;

        Ok(user)
    }

    pub async fn get_id(username: String, pool: &PgPool) -> Result<i32> {
        let record = sqlx::query!(
            r#"
                SELECT id FROM users WHERE username = $1
            "#,
            username
        )
        .fetch_one(pool)
        .await?;

        Ok(record.id)
    }

    pub async fn get_password(user: UserRequest, pool: &PgPool) -> Result<String> {
        let record = sqlx::query!(
            r#"
                SELECT password_hash FROM users
                WHERE username = $1
            "#,
            user.username
        )
            .fetch_one(pool)
            .await?;

        Ok(record.password_hash)
    }

    pub async fn delete(id: i32, pool: &PgPool) -> Result<PgQueryResult> {
        let mut table = pool.begin().await?;
        let deleted = sqlx::query("DELETE FROM users WHERE id = $1")
            .bind(id)
            .execute(&mut table)
            .await?;

        table.commit().await?;
        Ok(deleted)
    }
}