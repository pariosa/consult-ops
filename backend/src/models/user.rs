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
            WHERE id = $1
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
            WHERE lower(email) = lower($1)
            "#,
        )
        .bind(email)
        .fetch_one(&*db.pool)
        .await
    }

    pub async fn find_optional_by_email(db: &Db, email: &str) -> SqlxResult<Option<Self>> {
        sqlx::query_as::<_, User>(
            r#"
            SELECT id, email, password_hash, name, user_type, created_at, updated_at
            FROM users
            WHERE lower(email) = lower($1)
            "#,
        )
        .bind(email)
        .fetch_optional(&*db.pool)
        .await
    }

    pub async fn create(db: &Db, user: CreateUser) -> SqlxResult<Self> {
        let now = chrono::Utc::now().to_rfc3339();
        let normalized_email = user.email.trim().to_lowercase();

        sqlx::query_as::<_, User>(
            r#"
            INSERT INTO users (
                email,
                password_hash,
                name,
                user_type,
                created_at,
                updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, email, password_hash, name, user_type, created_at, updated_at
            "#,
        )
        .bind(normalized_email)
        .bind(&user.password_hash)
        .bind(&user.name)
        .bind(&user.user_type)
        .bind(&now)
        .bind(&now)
        .fetch_one(&*db.pool)
        .await
    }

    pub async fn update_user_type(db: &Db, id: i64, user_type: String) -> SqlxResult<Self> {
        let now = chrono::Utc::now().to_rfc3339();

        sqlx::query_as::<_, User>(
            r#"
            UPDATE users
            SET
                user_type = $1,
                updated_at = $2
            WHERE id = $3
            RETURNING id, email, password_hash, name, user_type, created_at, updated_at
            "#,
        )
        .bind(&user_type)
        .bind(&now)
        .bind(id)
        .fetch_one(&*db.pool)
        .await
    }

    pub async fn disable(db: &Db, id: i64) -> SqlxResult<Self> {
        let now = chrono::Utc::now().to_rfc3339();

        sqlx::query_as::<_, User>(
            r#"
            UPDATE users
            SET
                disabled_at = $1,
                updated_at = $1
            WHERE id = $2
            RETURNING id, email, password_hash, name, user_type, created_at, updated_at
            "#,
        )
        .bind(&now)
        .bind(id)
        .fetch_one(&*db.pool)
        .await
    }

    pub async fn update_current_organization(
        db: &Db,
        id: i64,
        organization_id: i64,
    ) -> SqlxResult<Self> {
        let now = chrono::Utc::now().to_rfc3339();

        sqlx::query_as::<_, User>(
            r#"
            UPDATE users
            SET
                current_organization_id = $1,
                updated_at = $2
            WHERE id = $3
            RETURNING id, email, password_hash, name, user_type, created_at, updated_at
            "#,
        )
        .bind(organization_id)
        .bind(&now)
        .bind(id)
        .fetch_one(&*db.pool)
        .await
    }
}
