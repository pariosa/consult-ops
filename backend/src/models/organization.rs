// backend/src/models/organization.rs

use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

use crate::db::Db;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Organization {
    pub id: i64,
    pub name: String,
    pub slug: Option<String>,
    pub created_by_user_id: Option<i64>,
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
    pub slug: Option<String>,
}

fn slugify(name: &str) -> String {
    let slug = name
        .trim()
        .to_lowercase()
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .split('-')
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join("-");

    if slug.is_empty() {
        "organization".to_string()
    } else {
        slug
    }
}

impl Organization {
    pub async fn all(db: &Db) -> sqlx::Result<Vec<Organization>> {
        sqlx::query_as::<_, Organization>(
            r#"
            SELECT id, name, slug, created_by_user_id, created_at, updated_at
            FROM organizations
            ORDER BY name ASC
            "#,
        )
        .fetch_all(&*db.pool)
        .await
    }

    pub async fn find(db: &Db, id: i64) -> sqlx::Result<Option<Organization>> {
        Self::find_by_pool(&db.pool, id).await
    }

    pub async fn find_by_pool(db: &PgPool, id: i64) -> sqlx::Result<Option<Organization>> {
        sqlx::query_as::<_, Organization>(
            r#"
            SELECT id, name, slug, created_by_user_id, created_at, updated_at
            FROM organizations
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(db)
        .await
    }

    pub async fn create(db: &Db, payload: CreateOrganization) -> sqlx::Result<Organization> {
        let slug = slugify(&payload.name);

        sqlx::query_as::<_, Organization>(
            r#"
            INSERT INTO organizations (
                name,
                slug,
                created_at,
                updated_at
            )
            VALUES (
                $1,
                $2,
                CURRENT_TIMESTAMP,
                CURRENT_TIMESTAMP
            )
            RETURNING id, name, slug, created_by_user_id, created_at, updated_at
            "#,
        )
        .bind(payload.name)
        .bind(slug)
        .fetch_one(&*db.pool)
        .await
    }

    pub async fn create_for_user(
        db: &PgPool,
        creator_user_id: i64,
        payload: CreateOrganization,
    ) -> sqlx::Result<Organization> {
        let slug = slugify(&payload.name);

        let mut tx = db.begin().await?;

        let organization = sqlx::query_as::<_, Organization>(
            r#"
            INSERT INTO organizations (
                name,
                slug,
                created_by_user_id,
                created_at,
                updated_at
            )
            VALUES (
                $1,
                $2,
                $3,
                CURRENT_TIMESTAMP,
                CURRENT_TIMESTAMP
            )
            RETURNING id, name, slug, created_by_user_id, created_at, updated_at
            "#,
        )
        .bind(payload.name)
        .bind(slug)
        .bind(creator_user_id)
        .fetch_one(&mut *tx)
        .await?;

        sqlx::query(
            r#"
            INSERT INTO organization_members (
                organization_id,
                user_id,
                role,
                status,
                created_at,
                updated_at
            )
            VALUES (
                $1,
                $2,
                'owner',
                'active',
                CURRENT_TIMESTAMP,
                CURRENT_TIMESTAMP
            )
            ON CONFLICT(organization_id, user_id)
            DO UPDATE SET
                role = 'owner',
                status = 'active',
                updated_at = CURRENT_TIMESTAMP
            "#,
        )
        .bind(organization.id)
        .bind(creator_user_id)
        .execute(&mut *tx)
        .await?;

        sqlx::query(
            r#"
            UPDATE users
            SET
                current_organization_id = $1,
                updated_at = CURRENT_TIMESTAMP
            WHERE id = $2
            "#,
        )
        .bind(organization.id)
        .bind(creator_user_id)
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(organization)
    }

    pub async fn update(
        db: &Db,
        id: i64,
        payload: UpdateOrganization,
    ) -> sqlx::Result<Option<Organization>> {
        if payload.name.is_none() && payload.slug.is_none() {
            return Self::find(db, id).await;
        }

        let existing = Self::find(db, id).await?;

        let Some(existing) = existing else {
            return Ok(None);
        };

        let name = payload.name.unwrap_or(existing.name);
        let slug = payload
            .slug
            .or(existing.slug)
            .unwrap_or_else(|| slugify(&name));

        let organization = sqlx::query_as::<_, Organization>(
            r#"
            UPDATE organizations
            SET
                name = $1,
                slug = $2,
                updated_at = CURRENT_TIMESTAMP
            WHERE id = $3
            RETURNING id, name, slug, created_by_user_id, created_at, updated_at
            "#,
        )
        .bind(name)
        .bind(slug)
        .bind(id)
        .fetch_optional(&*db.pool)
        .await?;

        Ok(organization)
    }

    pub async fn find_for_user(
        db: &PgPool,
        organization_id: i64,
        user_id: i64,
    ) -> sqlx::Result<Option<Organization>> {
        sqlx::query_as::<_, Organization>(
            r#"
            SELECT o.id, o.name, o.slug, o.created_by_user_id, o.created_at, o.updated_at
            FROM organizations o
            JOIN organization_members om
              ON om.organization_id = o.id
            WHERE o.id = $1
              AND om.user_id = $2
              AND om.status = 'active'
            "#,
        )
        .bind(organization_id)
        .bind(user_id)
        .fetch_optional(db)
        .await
    }
}
