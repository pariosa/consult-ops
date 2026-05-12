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

#[derive(Debug, Deserialize)]
pub struct CreateEngagementBillingRequest {
    pub billing_type: Option<String>,
    pub amount_cents: Option<i64>,
    pub currency: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateCheckoutSessionRequest {
    pub stripe_checkout_session_id: String,
}

impl EngagementBilling {
    pub async fn find_activation_fee(
        db: &SqlitePool,
        engagement_id: i64,
    ) -> SqlxResult<Option<Self>> {
        sqlx::query_as::<_, EngagementBilling>(
            r#"
        SELECT *
        FROM engagement_billing
        WHERE engagement_id = ?
          AND billing_type = 'activation_fee'
        ORDER BY created_at DESC
        LIMIT 1
        "#,
        )
        .bind(engagement_id)
        .fetch_optional(db)
        .await
    }
    pub async fn for_engagement(db: &SqlitePool, engagement_id: i64) -> SqlxResult<Vec<Self>> {
        sqlx::query_as::<_, EngagementBilling>(
            r#"
            SELECT *
            FROM engagement_billing
            WHERE engagement_id = ?
            ORDER BY created_at DESC
            "#,
        )
        .bind(engagement_id)
        .fetch_all(db)
        .await
    }

    pub async fn create(
        db: &SqlitePool,
        engagement_id: i64,
        organization_id: i64,
        billing_type: String,
        amount_cents: i64,
        currency: String,
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
            VALUES (?, ?, ?, ?, ?, 'pending', datetime('now'))
            RETURNING *
            "#,
        )
        .bind(engagement_id)
        .bind(organization_id)
        .bind(billing_type)
        .bind(amount_cents)
        .bind(currency)
        .fetch_one(db)
        .await
    }

    pub async fn create_activation_fee(
        db: &SqlitePool,
        engagement_id: i64,
        organization_id: i64,
    ) -> SqlxResult<Self> {
        Self::create(
            db,
            engagement_id,
            organization_id,
            "activation_fee".to_string(),
            1000,
            "usd".to_string(),
        )
        .await
    }

    pub async fn attach_checkout_session(
        db: &SqlitePool,
        billing_id: i64,
        stripe_checkout_session_id: &str,
    ) -> SqlxResult<Self> {
        sqlx::query_as::<_, EngagementBilling>(
            r#"
            UPDATE engagement_billing
            SET stripe_checkout_session_id = ?
            WHERE id = ?
            RETURNING *
            "#,
        )
        .bind(stripe_checkout_session_id)
        .bind(billing_id)
        .fetch_one(db)
        .await
    }

    pub async fn mark_paid(db: &SqlitePool, billing_id: i64) -> SqlxResult<Self> {
        sqlx::query_as::<_, EngagementBilling>(
            r#"
            UPDATE engagement_billing
            SET status = 'paid',
                paid_at = datetime('now')
            WHERE id = ?
            RETURNING *
            "#,
        )
        .bind(billing_id)
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
