use crate::services::email_notification_service::EmailNotificationService;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Result as SqlxResult, SqlitePool};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct OrganizationMember {
    pub id: i64,
    pub organization_id: i64,
    pub user_id: i64,
    pub role: String,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
}

impl OrganizationMember {
    pub async fn list_for_organization(
        db: &SqlitePool,
        organization_id: i64,
    ) -> SqlxResult<Vec<Self>> {
        sqlx::query_as::<_, OrganizationMember>(
            r#"
            SELECT *
            FROM organization_members
            WHERE organization_id = ?
            ORDER BY created_at DESC
            "#,
        )
        .bind(organization_id)
        .fetch_all(db)
        .await
    }

    pub async fn upsert_active_member(
        db: &SqlitePool,
        organization_id: i64,
        user_id: i64,
        role: &str,
    ) -> SqlxResult<Self> {
        sqlx::query_as::<_, OrganizationMember>(
            r#"
            INSERT INTO organization_members (
                organization_id,
                user_id,
                role,
                status,
                created_at,
                updated_at
            )
            VALUES (?, ?, ?, 'active', datetime('now'), datetime('now'))
            ON CONFLICT(organization_id, user_id)
            DO UPDATE SET
                role = excluded.role,
                status = 'active',
                updated_at = datetime('now')
            RETURNING *
            "#,
        )
        .bind(organization_id)
        .bind(user_id)
        .bind(role)
        .fetch_one(db)
        .await
    }
}
