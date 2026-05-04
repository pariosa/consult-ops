use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Result as SqlxResult, SqlitePool};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct EngagementBilling {
    pub id: i64,
    pub engagement_id: i64,
    pub organization_id: i64,
    pub billing_type: String,
    pub amount_cents: i64,
    pub currency: String,
    pub status: String,
    pub stripe_checkout_session_id: Option<String>,
    pub stripe_payment_intent_id: Option<String>,
    pub paid_at: Option<String>,
    pub created_at: String,
}

impl EngagementBilling {
    pub async fn create_activation_fee(
        db: &SqlitePool,
        engagement_id: i64,
        organization_id: i64,
    ) -> SqlxResult<Self> {
        sqlx::query_as::<_, EngagementBilling>(
            r#"
            INSERT INTO engagement_billing (
                engagement_id,
                organization_id,
                billing_type,
                amount_cents,
                currency,
                status,
                created_at
            )
            VALUES (?, ?, 'activation_fee', 1000, 'usd', 'pending', datetime('now'))
            RETURNING *
            "#,
        )
        .bind(engagement_id)
        .bind(organization_id)
        .fetch_one(db)
        .await
    }

    pub async fn mark_paid_by_session(
        db: &SqlitePool,
        checkout_session_id: &str,
    ) -> SqlxResult<Self> {
        sqlx::query_as::<_, EngagementBilling>(
            r#"
            UPDATE engagement_billing
            SET status = 'paid',
                paid_at = datetime('now')
            WHERE stripe_checkout_session_id = ?
            RETURNING *
            "#,
        )
        .bind(checkout_session_id)
        .fetch_one(db)
        .await
    }
}
