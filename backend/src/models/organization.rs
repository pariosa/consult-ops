// backend/src/models/organization.rs

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::db::Db;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Organization {
    pub id: i64,
    pub name: String,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateOrganization {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateOrganization {
    pub name: Option<String>,
}

impl Organization {
    pub async fn all(db: &Db) -> sqlx::Result<Vec<Organization>> {
        sqlx::query_as::<_, Organization>(
            r#"
            SELECT id, name, created_at, updated_at
            FROM organizations
            ORDER BY name ASC
            "#,
        )
        .fetch_all(&*db.pool)
        .await
    }

    pub async fn find(db: &Db, id: i64) -> sqlx::Result<Option<Organization>> {
        sqlx::query_as::<_, Organization>(
            r#"
            SELECT id, name, created_at, updated_at
            FROM organizations
            WHERE id = ?
            "#,
        )
        .bind(id)
        .fetch_optional(&*db.pool)
        .await
    }

    pub async fn create(db: &Db, payload: CreateOrganization) -> sqlx::Result<Organization> {
        let rec = sqlx::query(
            r#"
            INSERT INTO organizations (name, created_at, updated_at)
            VALUES (?, datetime('now'), datetime('now'))
            "#,
        )
        .bind(payload.name)
        .execute(&*db.pool)
        .await?;

        let id = rec.last_insert_rowid();

        Self::find(db, id).await?.ok_or(sqlx::Error::RowNotFound)
    }

    pub async fn update(
        db: &Db,
        id: i64,
        payload: UpdateOrganization,
    ) -> sqlx::Result<Option<Organization>> {
        if let Some(name) = payload.name {
            sqlx::query(
                r#"
                UPDATE organizations
                SET name = ?, updated_at = datetime('now')
                WHERE id = ?
                "#,
            )
            .bind(name)
            .bind(id)
            .execute(&*db.pool)
            .await?;
        }

        Self::find(db, id).await
    }
}
