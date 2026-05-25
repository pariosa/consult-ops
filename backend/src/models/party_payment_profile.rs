use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool, Result as SqlxResult};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct PartyPaymentProfile {
    pub id: i64,
    pub party_id: i64,
    pub organization_id: i64,
    pub payment_role: String,

    pub stripe_customer_id: Option<String>,
    pub stripe_payment_method_id: Option<String>,
    pub payer_authorization_status: String,
    pub payer_authorized_at: Option<String>,
    pub payer_authorization_scope: Option<String>,

    pub stripe_connect_account_id: Option<String>,
    pub stripe_connect_onboarding_status: String,
    pub payout_status: String,
    pub payout_verified_at: Option<String>,

    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize)]
pub struct UpsertPartyPaymentProfile {
    pub payment_role: String,
    pub payer_authorization_scope: Option<String>,
}

impl PartyPaymentProfile {
    pub async fn find_by_party(db: &PgPool, party_id: i64) -> SqlxResult<Option<Self>> {
        sqlx::query_as::<_, Self>(
            r#"
            SELECT *
            FROM party_payment_profiles
            WHERE party_id = $1
            LIMIT 1
            "#,
        )
        .bind(party_id)
        .fetch_optional(db)
        .await
    }

    pub async fn find_by_party_for_user(
        db: &PgPool,
        party_id: i64,
        user_id: i64,
    ) -> SqlxResult<Option<Self>> {
        sqlx::query_as::<_, Self>(
            r#"
            SELECT ppp.*
            FROM party_payment_profiles ppp
            JOIN organization_members om
              ON om.organization_id = ppp.organization_id
            WHERE ppp.party_id = $1
              AND om.user_id = $2
              AND om.status = 'active'
            LIMIT 1
            "#,
        )
        .bind(party_id)
        .bind(user_id)
        .fetch_optional(db)
        .await
    }

    pub async fn upsert_basic(
        db: &PgPool,
        party_id: i64,
        organization_id: i64,
        payment_role: &str,
        payer_authorization_scope: Option<String>,
    ) -> SqlxResult<Self> {
        sqlx::query_as::<_, Self>(
            r#"
            INSERT INTO party_payment_profiles (
                party_id,
                organization_id,
                payment_role,
                payer_authorization_scope,
                created_at,
                updated_at
            )
            VALUES (
                $1,
                $2,
                $3,
                $4,
                CURRENT_TIMESTAMP,
                CURRENT_TIMESTAMP
            )
            ON CONFLICT(party_id) DO UPDATE SET
                payment_role = excluded.payment_role,
                payer_authorization_scope = excluded.payer_authorization_scope,
                updated_at = CURRENT_TIMESTAMP
            RETURNING *
            "#,
        )
        .bind(party_id)
        .bind(organization_id)
        .bind(payment_role)
        .bind(payer_authorization_scope)
        .fetch_one(db)
        .await
    }

    pub async fn mark_payout_ready(
        db: &PgPool,
        party_id: i64,
        stripe_connect_account_id: String,
    ) -> SqlxResult<Self> {
        sqlx::query_as::<_, Self>(
            r#"
            UPDATE party_payment_profiles
            SET
                stripe_connect_account_id = $1,
                stripe_connect_onboarding_status = 'complete',
                payout_status = 'ready',
                payout_verified_at = CURRENT_TIMESTAMP,
                updated_at = CURRENT_TIMESTAMP
            WHERE party_id = $2
            RETURNING *
            "#,
        )
        .bind(stripe_connect_account_id)
        .bind(party_id)
        .fetch_one(db)
        .await
    }

    pub async fn mark_payer_authorized(
        db: &PgPool,
        party_id: i64,
        stripe_customer_id: String,
        stripe_payment_method_id: String,
        scope: String,
    ) -> SqlxResult<Self> {
        sqlx::query_as::<_, Self>(
            r#"
            UPDATE party_payment_profiles
            SET
                stripe_customer_id = $1,
                stripe_payment_method_id = $2,
                payer_authorization_status = 'authorized',
                payer_authorized_at = CURRENT_TIMESTAMP,
                payer_authorization_scope = $3,
                updated_at = CURRENT_TIMESTAMP
            WHERE party_id = $4
            RETURNING *
            "#,
        )
        .bind(stripe_customer_id)
        .bind(stripe_payment_method_id)
        .bind(scope)
        .bind(party_id)
        .fetch_one(db)
        .await
    }

    pub async fn mark_connect_onboarding_started(
        db: &PgPool,
        party_id: i64,
        stripe_connect_account_id: String,
    ) -> SqlxResult<Self> {
        sqlx::query_as::<_, Self>(
            r#"
            UPDATE party_payment_profiles
            SET
                stripe_connect_account_id = $1,
                stripe_connect_onboarding_status = 'started',
                updated_at = CURRENT_TIMESTAMP
            WHERE party_id = $2
            RETURNING *
            "#,
        )
        .bind(stripe_connect_account_id)
        .bind(party_id)
        .fetch_one(db)
        .await
    }

    pub async fn mark_payout_not_ready(db: &PgPool, party_id: i64) -> SqlxResult<Self> {
        sqlx::query_as::<_, Self>(
            r#"
            UPDATE party_payment_profiles
            SET
                payout_status = 'not_ready',
                payout_verified_at = NULL,
                updated_at = CURRENT_TIMESTAMP
            WHERE party_id = $1
            RETURNING *
            "#,
        )
        .bind(party_id)
        .fetch_one(db)
        .await
    }

    pub async fn revoke_payer_authorization(db: &PgPool, party_id: i64) -> SqlxResult<Self> {
        sqlx::query_as::<_, Self>(
            r#"
            UPDATE party_payment_profiles
            SET
                payer_authorization_status = 'revoked',
                payer_authorized_at = NULL,
                payer_authorization_scope = NULL,
                updated_at = CURRENT_TIMESTAMP
            WHERE party_id = $1
            RETURNING *
            "#,
        )
        .bind(party_id)
        .fetch_one(db)
        .await
    }
}
