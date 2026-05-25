use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool, Result as SqlxResult};

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
        db: &PgPool,
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
            WHERE organization_id = $1
              AND lower(email) = $2
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
            VALUES (
                $1,
                $2,
                $3,
                $4,
                'pending',
                $5,
                $6,
                CURRENT_TIMESTAMP,
                CURRENT_TIMESTAMP
            )
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

    pub async fn list_for_organization(db: &PgPool, organization_id: i64) -> SqlxResult<Vec<Self>> {
        sqlx::query_as::<_, OrganizationInvitation>(
            r#"
            SELECT *
            FROM organization_invitations
            WHERE organization_id = $1
            ORDER BY created_at DESC
            "#,
        )
        .bind(organization_id)
        .fetch_all(db)
        .await
    }

    pub async fn list_for_organization_for_user(
        db: &PgPool,
        organization_id: i64,
        user_id: i64,
    ) -> SqlxResult<Vec<Self>> {
        sqlx::query_as::<_, OrganizationInvitation>(
            r#"
            SELECT oi.*
            FROM organization_invitations oi
            JOIN organization_members om
              ON om.organization_id = oi.organization_id
            WHERE oi.organization_id = $1
              AND om.user_id = $2
              AND om.status = 'active'
            ORDER BY oi.created_at DESC
            "#,
        )
        .bind(organization_id)
        .bind(user_id)
        .fetch_all(db)
        .await
    }

    pub async fn find_pending_by_token(db: &PgPool, token: &str) -> SqlxResult<Self> {
        sqlx::query_as::<_, OrganizationInvitation>(
            r#"
            SELECT *
            FROM organization_invitations
            WHERE token = $1
              AND status = 'pending'
            "#,
        )
        .bind(token)
        .fetch_one(db)
        .await
    }

    pub async fn find_pending_by_email(db: &PgPool, email: &str) -> SqlxResult<Vec<Self>> {
        let normalized_email = email.trim().to_lowercase();

        sqlx::query_as::<_, OrganizationInvitation>(
            r#"
            SELECT *
            FROM organization_invitations
            WHERE lower(email) = $1
              AND status = 'pending'
            ORDER BY created_at DESC
            "#,
        )
        .bind(normalized_email)
        .fetch_all(db)
        .await
    }

    pub async fn mark_accepted(db: &PgPool, invitation_id: i64, user_id: i64) -> SqlxResult<Self> {
        sqlx::query_as::<_, OrganizationInvitation>(
            r#"
            UPDATE organization_invitations
            SET
                status = 'accepted',
                accepted_by_user_id = $1,
                accepted_at = CURRENT_TIMESTAMP,
                updated_at = CURRENT_TIMESTAMP
            WHERE id = $2
            RETURNING *
            "#,
        )
        .bind(user_id)
        .bind(invitation_id)
        .fetch_one(db)
        .await
    }

    pub async fn mark_expired(db: &PgPool, invitation_id: i64) -> SqlxResult<Self> {
        sqlx::query_as::<_, OrganizationInvitation>(
            r#"
            UPDATE organization_invitations
            SET
                status = 'expired',
                updated_at = CURRENT_TIMESTAMP
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(invitation_id)
        .fetch_one(db)
        .await
    }

    pub async fn revoke(db: &PgPool, invitation_id: i64) -> SqlxResult<Self> {
        sqlx::query_as::<_, OrganizationInvitation>(
            r#"
            UPDATE organization_invitations
            SET
                status = 'revoked',
                updated_at = CURRENT_TIMESTAMP
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(invitation_id)
        .fetch_one(db)
        .await
    }
}
