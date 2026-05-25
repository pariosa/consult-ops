use serde::Serialize;
use sqlx::{PgPool, Row};

#[derive(Debug, Serialize)]
pub struct OrganizationFinanceSummary {
    pub organization_id: i64,
    pub pending_cents: i64,
    pub processing_cents: i64,
    pub paid_cents: i64,
    pub failed_cents: i64,
    pub cancelled_cents: i64,
    pub total_obligations_cents: i64,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct PartyBalanceSummary {
    pub party_id: i64,
    pub party_name: String,
    pub party_type: String,
    pub is_verified: i64,
    pub payable_cents: i64,
    pub receivable_cents: i64,
    pub net_cents: i64,
}

pub struct OperationalFinanceService;

impl OperationalFinanceService {
    pub async fn organization_summary(
        pool: &PgPool,
        organization_id: i64,
    ) -> Result<OrganizationFinanceSummary, String> {
        let rows = sqlx::query(
            r#"
            SELECT
                status,
                COALESCE(SUM(amount_cents), 0)::BIGINT AS amount
            FROM operational_transactions
            WHERE organization_id = $1
            GROUP BY status
            "#,
        )
        .bind(organization_id)
        .fetch_all(pool)
        .await
        .map_err(|err| err.to_string())?;

        let mut summary = OrganizationFinanceSummary {
            organization_id,
            pending_cents: 0,
            processing_cents: 0,
            paid_cents: 0,
            failed_cents: 0,
            cancelled_cents: 0,
            total_obligations_cents: 0,
        };

        for row in rows {
            let status: String = row.try_get("status").map_err(|err| err.to_string())?;
            let amount: i64 = row.try_get("amount").map_err(|err| err.to_string())?;

            summary.total_obligations_cents += amount;

            match status.as_str() {
                "pending" => summary.pending_cents = amount,
                "processing" => summary.processing_cents = amount,
                "paid" => summary.paid_cents = amount,
                "failed" => summary.failed_cents = amount,
                "cancelled" => summary.cancelled_cents = amount,
                _ => {}
            }
        }

        Ok(summary)
    }

    pub async fn party_balances(
        pool: &PgPool,
        organization_id: i64,
    ) -> Result<Vec<PartyBalanceSummary>, String> {
        sqlx::query_as::<_, PartyBalanceSummary>(
            r#"
            SELECT
                p.id AS party_id,
                p.name AS party_name,
                p.party_type,
                p.is_verified,

                COALESCE((
                    SELECT SUM(t.amount_cents)
                    FROM operational_transactions t
                    WHERE t.organization_id = p.organization_id
                      AND t.from_party_id = p.id
                      AND t.status IN ('pending', 'processing')
                ), 0)::BIGINT AS payable_cents,

                COALESCE((
                    SELECT SUM(t.amount_cents)
                    FROM operational_transactions t
                    WHERE t.organization_id = p.organization_id
                      AND t.to_party_id = p.id
                      AND t.status IN ('pending', 'processing')
                ), 0)::BIGINT AS receivable_cents,

                (
                    COALESCE((
                        SELECT SUM(t.amount_cents)
                        FROM operational_transactions t
                        WHERE t.organization_id = p.organization_id
                          AND t.to_party_id = p.id
                          AND t.status IN ('pending', 'processing')
                    ), 0)
                    -
                    COALESCE((
                        SELECT SUM(t.amount_cents)
                        FROM operational_transactions t
                        WHERE t.organization_id = p.organization_id
                          AND t.from_party_id = p.id
                          AND t.status IN ('pending', 'processing')
                    ), 0)
                )::BIGINT AS net_cents

            FROM parties p
            WHERE p.organization_id = $1
            ORDER BY ABS(
                (
                    COALESCE((
                        SELECT SUM(t.amount_cents)
                        FROM operational_transactions t
                        WHERE t.organization_id = p.organization_id
                          AND t.to_party_id = p.id
                          AND t.status IN ('pending', 'processing')
                    ), 0)
                    -
                    COALESCE((
                        SELECT SUM(t.amount_cents)
                        FROM operational_transactions t
                        WHERE t.organization_id = p.organization_id
                          AND t.from_party_id = p.id
                          AND t.status IN ('pending', 'processing')
                    ), 0)
                )
            ) DESC
            "#,
        )
        .bind(organization_id)
        .fetch_all(pool)
        .await
        .map_err(|err| err.to_string())
    }
}
