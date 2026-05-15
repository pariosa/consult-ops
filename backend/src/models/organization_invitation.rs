use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Result as SqlxResult, SqlitePool};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct OrganizationInvitation {
    pub id: i64,
    pub organization_id: i64,
    pub email: String,
    pub role: String,
    pub token: String,
    pub status: String,
    pub invited_by_user_id: Option<i64>,
    pub accepted_by_user_id: Option<i64>,
    pub expires_at: String,
    pub accepted_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateOrganizationInvitation {
    pub email: String,
    pub role: String,
}

impl OrganizationInvitation {
    pub async fn create(
        db: &SqlitePool,
        organization_id: i64,
        email: String,
        role: String,
        token: String,
        invited_by_user_id: Option<i64>,
        expires_at: String,
    ) -> SqlxResult<Self> {
        let normalized_email = email.trim().to_lowercase();

        if let Some(existing) = sqlx::query_as::<_, OrganizationInvitation>(
            r#"
            SELECT *
            FROM organization_invitations
            WHERE organization_id = ?
              AND lower(email) = ?
              AND status = 'pending'
            LIMIT 1
            "#,
        )
        .bind(organization_id)
        .bind(&normalized_email)
        .fetch_optional(db)
        .await?
        {
            return Ok(existing);
        }

        sqlx::query_as::<_, OrganizationInvitation>(
            r#"
            INSERT INTO organization_invitations (
                organization_id,
                email,
                role,
                token,
                status,
                invited_by_user_id,
                expires_at,
                created_at,
                updated_at
            )
            VALUES (?, ?, ?, ?, 'pending', ?, ?, datetime('now'), datetime('now'))
            RETURNING *
            "#,
        )
        .bind(organization_id)
        .bind(normalized_email)
        .bind(role)
        .bind(token)
        .bind(invited_by_user_id)
        .bind(expires_at)
        .fetch_one(db)
        .await
    }

    pub async fn list_for_organization(
        db: &SqlitePool,
        organization_id: i64,
    ) -> SqlxResult<Vec<Self>> {
        sqlx::query_as::<_, OrganizationInvitation>(
            r#"
            SELECT *
            FROM organization_invitations
            WHERE organization_id = ?
            ORDER BY created_at DESC
            "#,
        )
        .bind(organization_id)
        .fetch_all(db)
        .await
    }

    pub async fn find_pending_by_token(db: &SqlitePool, token: &str) -> SqlxResult<Self> {
        sqlx::query_as::<_, OrganizationInvitation>(
            r#"
            SELECT *
            FROM organization_invitations
            WHERE token = ?
              AND status = 'pending'
            "#,
        )
        .bind(token)
        .fetch_one(db)
        .await
    }

    pub async fn mark_accepted(
        db: &SqlitePool,
        invitation_id: i64,
        user_id: i64,
    ) -> SqlxResult<Self> {
        sqlx::query_as::<_, OrganizationInvitation>(
            r#"
            UPDATE organization_invitations
            SET status = 'accepted',
                accepted_by_user_id = ?,
                accepted_at = datetime('now'),
                updated_at = datetime('now')
            WHERE id = ?
            RETURNING *
            "#,
        )
        .bind(user_id)
        .bind(invitation_id)
        .fetch_one(db)
        .await
    }
}
