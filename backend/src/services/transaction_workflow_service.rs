use sqlx::{PgPool, Row};

use crate::models::operational_transaction::{
    CreateOperationalTransaction, OperationalTransaction,
};
use crate::services::event_service::EventService;

pub struct TransactionWorkflowService;

impl TransactionWorkflowService {
    #[allow(clippy::too_many_arguments)]
    pub async fn generate_transactions_for_trigger(
        pool: &PgPool,
        organization_id: i64,
        agreement_id: i64,
        engagement_id: Option<i64>,
        milestone_id: Option<i64>,
        trigger_event: &str,
        base_amount_cents: i64,
    ) -> Result<Vec<OperationalTransaction>, String> {
        let rules = sqlx::query(
            r#"
            SELECT
                id,
                from_party_id,
                to_party_id,
                rule_type,
                percent,
                amount_cents,
                trigger_event
            FROM agreement_payout_rules
            WHERE agreement_id = $1
              AND trigger_event = $2
            "#,
        )
        .bind(agreement_id)
        .bind(trigger_event)
        .fetch_all(pool)
        .await
        .map_err(|err| err.to_string())?;

        let mut created = Vec::new();

        for rule in rules {
            let rule_id: i64 = rule.try_get("id").map_err(|err| err.to_string())?;
            let from_party_id: i64 = rule
                .try_get("from_party_id")
                .map_err(|err| err.to_string())?;
            let to_party_id: i64 = rule.try_get("to_party_id").map_err(|err| err.to_string())?;
            let rule_type: String = rule.try_get("rule_type").map_err(|err| err.to_string())?;
            let percent: Option<i64> = rule.try_get("percent").map_err(|err| err.to_string())?;
            let fixed_amount_cents: Option<i64> = rule
                .try_get("amount_cents")
                .map_err(|err| err.to_string())?;

            let amount_cents = if let Some(percent) = percent {
                base_amount_cents * percent / 100
            } else {
                fixed_amount_cents.unwrap_or(0)
            };

            if amount_cents <= 0 {
                continue;
            }

            let duplicate_count: i64 = sqlx::query_scalar(
                r#"
                SELECT COUNT(*)::BIGINT
                FROM operational_transactions
                WHERE agreement_id = $1
                  AND engagement_id IS NOT DISTINCT FROM $2
                  AND milestone_id IS NOT DISTINCT FROM $3
                  AND from_party_id = $4
                  AND to_party_id = $5
                  AND transaction_type = $6
                  AND trigger_event = $7
                "#,
            )
            .bind(agreement_id)
            .bind(engagement_id)
            .bind(milestone_id)
            .bind(from_party_id)
            .bind(to_party_id)
            .bind(&rule_type)
            .bind(trigger_event)
            .fetch_one(pool)
            .await
            .map_err(|err| err.to_string())?;

            if duplicate_count > 0 {
                continue;
            }

            let transaction = OperationalTransaction::create(
                pool,
                CreateOperationalTransaction {
                    organization_id,
                    agreement_id: Some(agreement_id),
                    engagement_id,
                    milestone_id,
                    from_party_id,
                    to_party_id,
                    transaction_type: rule_type.clone(),
                    amount_cents,
                    currency: Some("usd".to_string()),
                    trigger_event: Some(trigger_event.to_string()),
                },
            )
            .await
            .map_err(|err| err.to_string())?;

            let _ = EventService::record_event(
                pool,
                organization_id,
                None,
                "operational_transaction",
                transaction.id,
                "OperationalTransactionCreated",
                None,
                Some(&transaction.status),
                serde_json::json!({
                    "agreement_id": agreement_id,
                    "engagement_id": engagement_id,
                    "milestone_id": milestone_id,
                    "from_party_id": transaction.from_party_id,
                    "to_party_id": transaction.to_party_id,
                    "transaction_type": transaction.transaction_type,
                    "amount_cents": transaction.amount_cents,
                    "trigger_event": trigger_event
                }),
            )
            .await;

            if let Some(engagement_id) = engagement_id {
                let _ = EventService::record_event(
                    pool,
                    organization_id,
                    None,
                    "engagement",
                    engagement_id,
                    "OperationalTransactionCreated",
                    None,
                    Some(&transaction.status),
                    serde_json::json!({
                        "transaction_id": transaction.id,
                        "agreement_id": agreement_id,
                        "milestone_id": milestone_id,
                        "from_party_id": transaction.from_party_id,
                        "to_party_id": transaction.to_party_id,
                        "transaction_type": transaction.transaction_type,
                        "amount_cents": transaction.amount_cents,
                        "trigger_event": trigger_event
                    }),
                )
                .await;
            }

            tracing::debug!(
                "Generated transaction from rule {} type={} from={} to={} percent={:?} amount={:?}",
                rule_id,
                rule_type,
                from_party_id,
                to_party_id,
                percent,
                fixed_amount_cents
            );

            created.push(transaction);
        }

        Ok(created)
    }
}
