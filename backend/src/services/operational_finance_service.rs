use serde::Serialize;
use sqlx::SqlitePool;

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
        pool: &SqlitePool,
        organization_id: i64,
    ) -> Result<OrganizationFinanceSummary, String> {
        let rows = sqlx::query!(
            r#"
            SELECT status as "status!", COALESCE(SUM(amount_cents), 0) as "amount!: i64"
            FROM operational_transactions
            WHERE organization_id = $1
            GROUP BY status
            "#,
            organization_id
        )
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
            summary.total_obligations_cents += row.amount;

            match row.status.as_str() {
                "pending" => summary.pending_cents = row.amount,
                "processing" => summary.processing_cents = row.amount,
                "paid" => summary.paid_cents = row.amount,
                "failed" => summary.failed_cents = row.amount,
                "cancelled" => summary.cancelled_cents = row.amount,
                _ => {}
            }
        }

        Ok(summary)
    }

    pub async fn party_balances(
        pool: &SqlitePool,
        organization_id: i64,
    ) -> Result<Vec<PartyBalanceSummary>, String> {
        sqlx::query_as::<_, PartyBalanceSummary>(
            r#"
            SELECT
                p.id as party_id,
                p.name as party_name,
                p.party_type,
                p.is_verified,

                COALESCE((
                    SELECT SUM(t.amount_cents)
                    FROM operational_transactions t
                    WHERE t.organization_id = p.organization_id
                      AND t.from_party_id = p.id
                      AND t.status IN ('pending', 'processing')
                ), 0) as payable_cents,

                COALESCE((
                    SELECT SUM(t.amount_cents)
                    FROM operational_transactions t
                    WHERE t.organization_id = p.organization_id
                      AND t.to_party_id = p.id
                      AND t.status IN ('pending', 'processing')
                ), 0) as receivable_cents,

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
                ), 0) as net_cents

            FROM parties p
            WHERE p.organization_id = ?
            ORDER BY ABS(net_cents) DESC
            "#,
        )
        .bind(organization_id)
        .fetch_all(pool)
        .await
        .map_err(|err| err.to_string())
    }
}
