// backend/src/models/subscription.rs

use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool, Result as SqlxResult};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct OrganizationSubscription {
    pub id: i64,
    pub organization_id: i64,
    pub subscription_status: String,
    pub subscription_plan: String,
    pub stripe_customer_id: Option<String>,
    pub stripe_subscription_id: Option<String>,
    pub current_period_start: Option<String>,
    pub current_period_end: Option<String>,
    pub cancel_at_period_end: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpsertOrganizationSubscription {
    pub subscription_status: Option<String>,
    pub subscription_plan: Option<String>,
    pub stripe_customer_id: Option<String>,
    pub stripe_subscription_id: Option<String>,
    pub current_period_start: Option<String>,
    pub current_period_end: Option<String>,
    pub cancel_at_period_end: Option<bool>,
}

impl OrganizationSubscription {
    pub async fn find_by_organization(
        db: &PgPool,
        organization_id: i64,
    ) -> SqlxResult<Option<Self>> {
        sqlx::query_as::<_, Self>(
            r#"
            SELECT *
            FROM organization_subscriptions
            WHERE organization_id = $1
            "#,
        )
        .bind(organization_id)
        .fetch_optional(db)
        .await
    }

    pub async fn upsert(
        db: &PgPool,
        organization_id: i64,
        input: UpsertOrganizationSubscription,
    ) -> SqlxResult<Self> {
        sqlx::query_as::<_, Self>(
            r#"
            INSERT INTO organization_subscriptions (
                organization_id,
                subscription_status,
                subscription_plan,
                stripe_customer_id,
                stripe_subscription_id,
                current_period_start,
                current_period_end,
                cancel_at_period_end,
                created_at,
                updated_at
            )
            VALUES (
                $1,
                COALESCE($2, 'inactive'),
                COALESCE($3, 'free'),
                $4,
                $5,
                $6,
                $7,
                COALESCE($8, FALSE),
                CURRENT_TIMESTAMP,
                CURRENT_TIMESTAMP
            )
            ON CONFLICT (organization_id)
            DO UPDATE SET
                subscription_status = COALESCE(excluded.subscription_status, organization_subscriptions.subscription_status),
                subscription_plan = COALESCE(excluded.subscription_plan, organization_subscriptions.subscription_plan),
                stripe_customer_id = COALESCE(excluded.stripe_customer_id, organization_subscriptions.stripe_customer_id),
                stripe_subscription_id = COALESCE(excluded.stripe_subscription_id, organization_subscriptions.stripe_subscription_id),
                current_period_start = COALESCE(excluded.current_period_start, organization_subscriptions.current_period_start),
                current_period_end = COALESCE(excluded.current_period_end, organization_subscriptions.current_period_end),
                cancel_at_period_end = excluded.cancel_at_period_end,
                updated_at = CURRENT_TIMESTAMP
            RETURNING *
            "#,
        )
        .bind(organization_id)
        .bind(input.subscription_status)
        .bind(input.subscription_plan)
        .bind(input.stripe_customer_id)
        .bind(input.stripe_subscription_id)
        .bind(input.current_period_start)
        .bind(input.current_period_end)
        .bind(input.cancel_at_period_end)
        .fetch_one(db)
        .await
    }

    pub async fn mark_active(
        db: &PgPool,
        organization_id: i64,
        subscription_plan: &str,
        stripe_customer_id: Option<String>,
        stripe_subscription_id: Option<String>,
    ) -> SqlxResult<Self> {
        Self::upsert(
            db,
            organization_id,
            UpsertOrganizationSubscription {
                subscription_status: Some("active".to_string()),
                subscription_plan: Some(subscription_plan.to_string()),
                stripe_customer_id,
                stripe_subscription_id,
                current_period_start: None,
                current_period_end: None,
                cancel_at_period_end: Some(false),
            },
        )
        .await
    }

    pub async fn mark_cancelled(db: &PgPool, organization_id: i64) -> SqlxResult<Self> {
        Self::upsert(
            db,
            organization_id,
            UpsertOrganizationSubscription {
                subscription_status: Some("cancelled".to_string()),
                subscription_plan: None,
                stripe_customer_id: None,
                stripe_subscription_id: None,
                current_period_start: None,
                current_period_end: None,
                cancel_at_period_end: Some(false),
            },
        )
        .await
    }

    pub fn is_active(&self) -> bool {
        matches!(self.subscription_status.as_str(), "active" | "trialing")
    }
}
