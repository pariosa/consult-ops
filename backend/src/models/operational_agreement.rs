use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool, Result as SqlxResult};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct OperationalAgreement {
    pub id: i64,
    pub organization_id: i64,
    pub engagement_id: Option<i64>,
    pub title: String,
    pub agreement_type: String,
    pub status: String,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateOperationalAgreement {
    pub engagement_id: Option<i64>,
    pub title: String,
    pub agreement_type: String,
}

impl OperationalAgreement {
    pub async fn create(
        db: &PgPool,
        organization_id: i64,
        payload: CreateOperationalAgreement,
    ) -> SqlxResult<Self> {
        sqlx::query_as::<_, OperationalAgreement>(
            r#"
            INSERT INTO operational_agreements (
                organization_id,
                engagement_id,
                title,
                agreement_type,
                status,
                created_at
            )
            VALUES ($1, $2, $3, $4, 'draft', CURRENT_TIMESTAMP)
            RETURNING *
            "#,
        )
        .bind(organization_id)
        .bind(payload.engagement_id)
        .bind(payload.title)
        .bind(payload.agreement_type)
        .fetch_one(db)
        .await
    }

    pub async fn for_organization(db: &PgPool, organization_id: i64) -> SqlxResult<Vec<Self>> {
        sqlx::query_as::<_, OperationalAgreement>(
            r#"
            SELECT *
            FROM operational_agreements
            WHERE organization_id = $1
            ORDER BY created_at DESC
            "#,
        )
        .bind(organization_id)
        .fetch_all(db)
        .await
    }

    pub async fn latest_for_engagement(
        db: &PgPool,
        engagement_id: i64,
    ) -> SqlxResult<Option<Self>> {
        sqlx::query_as::<_, OperationalAgreement>(
            r#"
            SELECT *
            FROM operational_agreements
            WHERE engagement_id = $1
            ORDER BY created_at DESC
            LIMIT 1
            "#,
        )
        .bind(engagement_id)
        .fetch_optional(db)
        .await
    }

    pub async fn lock(db: &PgPool, id: i64) -> SqlxResult<Self> {
        sqlx::query_as::<_, OperationalAgreement>(
            r#"
            UPDATE operational_agreements
            SET status = 'locked'
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(id)
        .fetch_one(db)
        .await
    }

    pub async fn find(db: &PgPool, id: i64) -> SqlxResult<Self> {
        sqlx::query_as::<_, OperationalAgreement>(
            r#"
            SELECT *
            FROM operational_agreements
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_one(db)
        .await
    }
    pub async fn find_for_user(db: &PgPool, id: i64, user_id: i64) -> SqlxResult<Self> {
        sqlx::query_as::<_, OperationalAgreement>(
            r#"
        SELECT oa.*
        FROM operational_agreements oa
        JOIN organization_members om
          ON om.organization_id = oa.organization_id
        WHERE oa.id = $1
          AND om.user_id = $2
          AND om.status = 'active'
        "#,
        )
        .bind(id)
        .bind(user_id)
        .fetch_one(db)
        .await
    }

    pub async fn for_organization_for_user(
        db: &PgPool,
        organization_id: i64,
        user_id: i64,
    ) -> SqlxResult<Vec<Self>> {
        sqlx::query_as::<_, OperationalAgreement>(
            r#"
        SELECT oa.*
        FROM operational_agreements oa
        JOIN organization_members om
          ON om.organization_id = oa.organization_id
        WHERE oa.organization_id = $1
          AND om.user_id = $2
          AND om.status = 'active'
        ORDER BY oa.created_at DESC
        "#,
        )
        .bind(organization_id)
        .bind(user_id)
        .fetch_all(db)
        .await
    }

    pub async fn latest_for_engagement_for_user(
        db: &PgPool,
        engagement_id: i64,
        user_id: i64,
    ) -> SqlxResult<Option<Self>> {
        sqlx::query_as::<_, OperationalAgreement>(
            r#"
        SELECT oa.*
        FROM operational_agreements oa
        JOIN organization_members om
          ON om.organization_id = oa.organization_id
        WHERE oa.engagement_id = $1
          AND om.user_id = $2
          AND om.status = 'active'
        ORDER BY oa.created_at DESC
        LIMIT 1
        "#,
        )
        .bind(engagement_id)
        .bind(user_id)
        .fetch_optional(db)
        .await
    }

    pub async fn lock_for_user(db: &PgPool, id: i64, user_id: i64) -> SqlxResult<Self> {
        sqlx::query_as::<_, OperationalAgreement>(
            r#"
        UPDATE operational_agreements oa
        SET status = 'locked'
        FROM organization_members om
        WHERE oa.id = $1
          AND om.organization_id = oa.organization_id
          AND om.user_id = $2
          AND om.status = 'active'
          AND om.role IN ('owner', 'admin')
        RETURNING oa.*
        "#,
        )
        .bind(id)
        .bind(user_id)
        .fetch_one(db)
        .await
    }
}
