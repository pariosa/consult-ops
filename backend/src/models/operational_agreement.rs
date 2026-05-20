use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Result as SqlxResult, SqlitePool};

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
        db: &SqlitePool,
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
            VALUES (?, ?, ?, ?, 'draft', datetime('now'))
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

    pub async fn for_organization(db: &SqlitePool, organization_id: i64) -> SqlxResult<Vec<Self>> {
        sqlx::query_as::<_, OperationalAgreement>(
            r#"
            SELECT *
            FROM operational_agreements
            WHERE organization_id = ?
            ORDER BY created_at DESC
            "#,
        )
        .bind(organization_id)
        .fetch_all(db)
        .await
    }

    pub async fn latest_for_engagement(
        db: &SqlitePool,
        engagement_id: i64,
    ) -> SqlxResult<Option<Self>> {
        sqlx::query_as::<_, OperationalAgreement>(
            r#"
            SELECT *
            FROM operational_agreements
            WHERE engagement_id = ?
            ORDER BY created_at DESC
            LIMIT 1
            "#,
        )
        .bind(engagement_id)
        .fetch_optional(db)
        .await
    }
    pub async fn lock(db: &SqlitePool, id: i64) -> SqlxResult<Self> {
        sqlx::query_as::<_, OperationalAgreement>(
            r#"
        UPDATE operational_agreements
        SET status = 'locked'
        WHERE id = ?
        RETURNING *
        "#,
        )
        .bind(id)
        .fetch_one(db)
        .await
    }

    pub async fn find(db: &SqlitePool, id: i64) -> SqlxResult<Self> {
        sqlx::query_as::<_, OperationalAgreement>(
            "SELECT * FROM operational_agreements WHERE id = ?",
        )
        .bind(id)
        .fetch_one(db)
        .await
    }
}
