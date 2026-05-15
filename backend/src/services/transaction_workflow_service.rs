use sqlx::SqlitePool;

use crate::models::operational_transaction::OperationalTransaction;
use crate::services::event_service::EventService;

pub struct TransactionWorkflowService;

impl TransactionWorkflowService {
    pub async fn generate_transactions_for_trigger(
        pool: &SqlitePool,
        organization_id: i64,
        agreement_id: i64,
        engagement_id: Option<i64>,
        milestone_id: Option<i64>,
        trigger_event: &str,
        base_amount_cents: i64,
    ) -> Result<Vec<OperationalTransaction>, String> {
        let rules = sqlx::query!(
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
            agreement_id,
            trigger_event
        )
        .fetch_all(pool)
        .await
        .map_err(|err| err.to_string())?;

        let mut created = Vec::new();

        for rule in rules {
            let amount_cents = if let Some(percent) = rule.percent {
                base_amount_cents * percent / 100
            } else {
                rule.amount_cents.unwrap_or(0)
            };

            if amount_cents <= 0 {
                continue;
            }
            let agreement_id_opt = Some(agreement_id);
            let trigger_event_string = trigger_event.to_string();
            let trigger_event_opt = Some(trigger_event_string.clone());
            let duplicate_count: i32 = sqlx::query_scalar!(
                r#"
    SELECT COUNT(*) as "count!"
    FROM operational_transactions
    WHERE agreement_id = $1
      AND engagement_id = $2
      AND milestone_id = $3
      AND from_party_id = $4
      AND to_party_id = $5
      AND transaction_type = $6
      AND trigger_event = $7
    "#,
                agreement_id_opt,
                engagement_id,
                milestone_id,
                rule.from_party_id,
                rule.to_party_id,
                rule.rule_type,
                trigger_event_opt
            )
            .fetch_one(pool)
            .await
            .map_err(|err| err.to_string())?;
            if duplicate_count > 0 {
                continue;
            }
            let transaction = OperationalTransaction::create(
                pool,
                organization_id,
                Some(agreement_id),
                engagement_id,
                milestone_id,
                rule.from_party_id,
                rule.to_party_id,
                rule.rule_type.clone(),
                amount_cents,
                "usd".to_string(),
                Some(trigger_event.to_string()),
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

            created.push(transaction);
        }

        Ok(created)
    }
}
