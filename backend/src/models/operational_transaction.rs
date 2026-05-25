use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool, Result as SqlxResult};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct OperationalTransaction {
    pub id: i64,
    pub organization_id: i64,
    pub agreement_id: Option<i64>,
    pub engagement_id: Option<i64>,
    pub milestone_id: Option<i64>,
    pub from_party_id: i64,
    pub to_party_id: i64,
    pub transaction_type: String,
    pub amount_cents: i64,
    pub currency: String,
    pub status: String,
    pub trigger_event: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateOperationalTransaction {
    pub organization_id: i64,
    pub agreement_id: Option<i64>,
    pub engagement_id: Option<i64>,
    pub milestone_id: Option<i64>,
    pub from_party_id: i64,
    pub to_party_id: i64,
    pub transaction_type: String,
    pub amount_cents: i64,
    pub currency: Option<String>,
    pub trigger_event: Option<String>,
}

impl OperationalTransaction {
    pub async fn create(db: &PgPool, payload: CreateOperationalTransaction) -> SqlxResult<Self> {
        let currency = payload.currency.unwrap_or_else(|| "usd".to_string());

        sqlx::query_as::<_, OperationalTransaction>(
            r#"
            INSERT INTO operational_transactions (
                organization_id,
                agreement_id,
                engagement_id,
                milestone_id,
                from_party_id,
                to_party_id,
                transaction_type,
                amount_cents,
                currency,
                status,
                trigger_event,
                created_at
            )
            VALUES (
                $1, $2, $3, $4, $5, $6,
                $7, $8, $9,
                'pending',
                $10,
                CURRENT_TIMESTAMP
            )
            RETURNING *
            "#,
        )
        .bind(payload.organization_id)
        .bind(payload.agreement_id)
        .bind(payload.engagement_id)
        .bind(payload.milestone_id)
        .bind(payload.from_party_id)
        .bind(payload.to_party_id)
        .bind(payload.transaction_type)
        .bind(payload.amount_cents)
        .bind(currency)
        .bind(payload.trigger_event)
        .fetch_one(db)
        .await
    }

    pub async fn find(db: &PgPool, id: i64) -> SqlxResult<Self> {
        sqlx::query_as::<_, OperationalTransaction>(
            r#"
            SELECT *
            FROM operational_transactions
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_one(db)
        .await
    }

    pub async fn find_for_user(db: &PgPool, id: i64, user_id: i64) -> SqlxResult<Self> {
        sqlx::query_as::<_, OperationalTransaction>(
            r#"
            SELECT ot.*
            FROM operational_transactions ot
            JOIN organization_members om
              ON om.organization_id = ot.organization_id
            WHERE ot.id = $1
              AND om.user_id = $2
              AND om.status = 'active'
            "#,
        )
        .bind(id)
        .bind(user_id)
        .fetch_one(db)
        .await
    }

    pub async fn for_engagement(db: &PgPool, engagement_id: i64) -> SqlxResult<Vec<Self>> {
        sqlx::query_as::<_, OperationalTransaction>(
            r#"
            SELECT *
            FROM operational_transactions
            WHERE engagement_id = $1
            ORDER BY created_at DESC
            "#,
        )
        .bind(engagement_id)
        .fetch_all(db)
        .await
    }

    pub async fn for_engagement_for_user(
        db: &PgPool,
        engagement_id: i64,
        user_id: i64,
    ) -> SqlxResult<Vec<Self>> {
        sqlx::query_as::<_, OperationalTransaction>(
            r#"
            SELECT ot.*
            FROM operational_transactions ot
            JOIN organization_members om
              ON om.organization_id = ot.organization_id
            WHERE ot.engagement_id = $1
              AND om.user_id = $2
              AND om.status = 'active'
            ORDER BY ot.created_at DESC
            "#,
        )
        .bind(engagement_id)
        .bind(user_id)
        .fetch_all(db)
        .await
    }

    pub async fn for_organization(db: &PgPool, organization_id: i64) -> SqlxResult<Vec<Self>> {
        sqlx::query_as::<_, OperationalTransaction>(
            r#"
            SELECT *
            FROM operational_transactions
            WHERE organization_id = $1
            ORDER BY created_at DESC
            "#,
        )
        .bind(organization_id)
        .fetch_all(db)
        .await
    }

    pub async fn update_status(db: &PgPool, id: i64, status: &str) -> SqlxResult<Self> {
        sqlx::query_as::<_, OperationalTransaction>(
            r#"
            UPDATE operational_transactions
            SET status = $1
            WHERE id = $2
            RETURNING *
            "#,
        )
        .bind(status)
        .bind(id)
        .fetch_one(db)
        .await
    }

    pub async fn mark_processing(db: &PgPool, id: i64) -> SqlxResult<Self> {
        Self::update_status(db, id, "processing").await
    }

    pub async fn mark_paid(db: &PgPool, id: i64) -> SqlxResult<Self> {
        Self::update_status(db, id, "paid").await
    }

    pub async fn mark_failed(db: &PgPool, id: i64) -> SqlxResult<Self> {
        Self::update_status(db, id, "failed").await
    }

    pub async fn cancel(db: &PgPool, id: i64) -> SqlxResult<Self> {
        Self::update_status(db, id, "cancelled").await
    }
}
