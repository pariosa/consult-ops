//src/models/agreement_payout_rule.rs
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Result as SqlxResult, SqlitePool};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct AgreementPayoutRule {
    pub id: i64,
    pub agreement_id: i64,
    pub from_party_id: i64,
    pub to_party_id: i64,
    pub rule_type: String,
    pub percent: Option<i64>,
    pub amount_cents: Option<i64>,
    pub trigger_event: String,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateAgreementPayoutRule {
    pub from_party_id: i64,
    pub to_party_id: i64,
    pub rule_type: String,
    pub percent: Option<i64>,
    pub amount_cents: Option<i64>,
    pub trigger_event: String,
}

impl AgreementPayoutRule {
    pub async fn create(
        db: &SqlitePool,
        agreement_id: i64,
        payload: CreateAgreementPayoutRule,
    ) -> SqlxResult<Self> {
        sqlx::query_as::<_, AgreementPayoutRule>(
            r#"
            INSERT INTO agreement_payout_rules (
                agreement_id,
                from_party_id,
                to_party_id,
                rule_type,
                percent,
                amount_cents,
                trigger_event,
                created_at
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, datetime('now'))
            RETURNING *
            "#,
        )
        .bind(agreement_id)
        .bind(payload.from_party_id)
        .bind(payload.to_party_id)
        .bind(payload.rule_type)
        .bind(payload.percent)
        .bind(payload.amount_cents)
        .bind(payload.trigger_event)
        .fetch_one(db)
        .await
    }

    pub async fn for_agreement(db: &SqlitePool, agreement_id: i64) -> SqlxResult<Vec<Self>> {
        sqlx::query_as::<_, AgreementPayoutRule>(
            r#"
            SELECT *
            FROM agreement_payout_rules
            WHERE agreement_id = ?
            ORDER BY created_at DESC
            "#,
        )
        .bind(agreement_id)
        .fetch_all(db)
        .await
    }
}
