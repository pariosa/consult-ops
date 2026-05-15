use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Result as SqlxResult, SqlitePool};

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

impl OperationalTransaction {
    #[allow(clippy::too_many_arguments)]
    pub async fn create(
        db: &SqlitePool,
        organization_id: i64,
        agreement_id: Option<i64>,
        engagement_id: Option<i64>,
        milestone_id: Option<i64>,
        from_party_id: i64,
        to_party_id: i64,
        transaction_type: String,
        amount_cents: i64,
        currency: String,
        trigger_event: Option<String>,
    ) -> SqlxResult<Self> {
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
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, 'pending', ?, datetime('now'))
            RETURNING *
            "#,
        )
        .bind(organization_id)
        .bind(agreement_id)
        .bind(engagement_id)
        .bind(milestone_id)
        .bind(from_party_id)
        .bind(to_party_id)
        .bind(transaction_type)
        .bind(amount_cents)
        .bind(currency)
        .bind(trigger_event)
        .fetch_one(db)
        .await
    }

    pub async fn for_engagement(db: &SqlitePool, engagement_id: i64) -> SqlxResult<Vec<Self>> {
        sqlx::query_as::<_, OperationalTransaction>(
            r#"
            SELECT *
            FROM operational_transactions
            WHERE engagement_id = ?
            ORDER BY created_at DESC
            "#,
        )
        .bind(engagement_id)
        .fetch_all(db)
        .await
    }
}
