use crate::db::Db;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Result as SqlxResult};

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct User {
    pub id: i64,
    pub email: String,
    pub password_hash: String,
    pub name: Option<String>,
    pub user_type: String,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateUser {
    pub email: String,
    pub password_hash: String,
    pub name: Option<String>,
    pub user_type: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateUserRequest {
    pub email: String,
    pub password: String,
    pub name: Option<String>,
    pub user_type: String,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateUserType {
    pub user_type: String,
}

impl User {
    pub async fn all(db: &Db) -> SqlxResult<Vec<Self>> {
        sqlx::query_as::<_, User>(
            r#"
            SELECT id, email, password_hash, name, user_type, created_at, updated_at
            FROM users
            ORDER BY created_at DESC
            "#,
        )
        .fetch_all(&*db.pool)
        .await
    }

    pub async fn find_by_id(db: &Db, id: i64) -> SqlxResult<Self> {
        sqlx::query_as::<_, User>(
            r#"
            SELECT id, email, password_hash, name, user_type, created_at, updated_at
            FROM users
            WHERE id = ?
            "#,
        )
        .bind(id)
        .fetch_one(&*db.pool)
        .await
    }

    pub async fn find_by_email(db: &Db, email: &str) -> SqlxResult<Self> {
        sqlx::query_as::<_, User>(
            r#"
            SELECT id, email, password_hash, name, user_type, created_at, updated_at
            FROM users
            WHERE email = ?
            "#,
        )
        .bind(email)
        .fetch_one(&*db.pool)
        .await
    }

    pub async fn create(db: &Db, user: CreateUser) -> SqlxResult<Self> {
        let now = chrono::Utc::now().to_rfc3339();

        let rec = sqlx::query(
            r#"
            INSERT INTO users
            (email, password_hash, name, user_type, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&user.email)
        .bind(&user.password_hash)
        .bind(&user.name)
        .bind(&user.user_type)
        .bind(&now)
        .bind(&now)
        .execute(&*db.pool)
        .await?;

        Ok(User {
            id: rec.last_insert_rowid(),
            email: user.email,
            password_hash: user.password_hash,
            name: user.name,
            user_type: user.user_type,
            created_at: Some(now.clone()),
            updated_at: Some(now),
        })
    }

    pub async fn update_user_type(db: &Db, id: i64, user_type: String) -> SqlxResult<Self> {
        let now = chrono::Utc::now().to_rfc3339();

        sqlx::query(
            r#"
            UPDATE users
            SET user_type = ?, updated_at = ?
            WHERE id = ?
            "#,
        )
        .bind(&user_type)
        .bind(&now)
        .bind(id)
        .execute(&*db.pool)
        .await?;

        User::find_by_id(db, id).await
    }
}
