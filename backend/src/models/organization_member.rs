use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool, Result as SqlxResult};

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
    pub async fn list_for_organization(db: &PgPool, organization_id: i64) -> SqlxResult<Vec<Self>> {
        sqlx::query_as::<_, OrganizationMember>(
            r#"
            SELECT *
            FROM organization_members
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
        sqlx::query_as::<_, OrganizationMember>(
            r#"
            SELECT target_members.*
            FROM organization_members target_members
            JOIN organization_members requester
              ON requester.organization_id = target_members.organization_id
            WHERE target_members.organization_id = $1
              AND requester.user_id = $2
              AND requester.status = 'active'
            ORDER BY target_members.created_at DESC
            "#,
        )
        .bind(organization_id)
        .bind(user_id)
        .fetch_all(db)
        .await
    }

    pub async fn find_active(
        db: &PgPool,
        organization_id: i64,
        user_id: i64,
    ) -> SqlxResult<Option<Self>> {
        sqlx::query_as::<_, OrganizationMember>(
            r#"
            SELECT *
            FROM organization_members
            WHERE organization_id = $1
              AND user_id = $2
              AND status = 'active'
            LIMIT 1
            "#,
        )
        .bind(organization_id)
        .bind(user_id)
        .fetch_optional(db)
        .await
    }

    pub async fn upsert_active_member(
        db: &PgPool,
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
            VALUES (
                $1,
                $2,
                $3,
                'active',
                CURRENT_TIMESTAMP,
                CURRENT_TIMESTAMP
            )
            ON CONFLICT(organization_id, user_id)
            DO UPDATE SET
                role = excluded.role,
                status = 'active',
                updated_at = CURRENT_TIMESTAMP
            RETURNING *
            "#,
        )
        .bind(organization_id)
        .bind(user_id)
        .bind(role)
        .fetch_one(db)
        .await
    }

    pub async fn update_role(db: &PgPool, member_id: i64, role: &str) -> SqlxResult<Self> {
        sqlx::query_as::<_, OrganizationMember>(
            r#"
            UPDATE organization_members
            SET
                role = $1,
                updated_at = CURRENT_TIMESTAMP
            WHERE id = $2
            RETURNING *
            "#,
        )
        .bind(role)
        .bind(member_id)
        .fetch_one(db)
        .await
    }

    pub async fn suspend(db: &PgPool, member_id: i64) -> SqlxResult<Self> {
        sqlx::query_as::<_, OrganizationMember>(
            r#"
            UPDATE organization_members
            SET
                status = 'suspended',
                updated_at = CURRENT_TIMESTAMP
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(member_id)
        .fetch_one(db)
        .await
    }

    pub async fn remove(db: &PgPool, member_id: i64) -> SqlxResult<Self> {
        sqlx::query_as::<_, OrganizationMember>(
            r#"
            UPDATE organization_members
            SET
                status = 'removed',
                updated_at = CURRENT_TIMESTAMP
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(member_id)
        .fetch_one(db)
        .await
    }
}
