use sqlx::{Result as SqlxResult, SqlitePool};

pub struct NotificationRecipientService;

impl NotificationRecipientService {
    pub async fn engagement_client_email(
        db: &SqlitePool,
        engagement_id: i64,
    ) -> SqlxResult<Option<String>> {
        let result = sqlx::query_scalar::<_, String>(
            r#"
            SELECT c.email
            FROM engagements e
            JOIN clients c ON c.id = e.client_id
            WHERE e.id = ?
            LIMIT 1
            "#,
        )
        .bind(engagement_id)
        .fetch_optional(db)
        .await?;

        Ok(result)
    }

    pub async fn agreement_party_emails(
        db: &SqlitePool,
        agreement_id: i64,
    ) -> SqlxResult<Vec<String>> {
        let rows = sqlx::query_scalar::<_, String>(
            r#"
            SELECT DISTINCT p.email
            FROM agreement_payout_rules apr
            JOIN parties p ON (
                p.id = apr.from_party_id
                OR p.id = apr.to_party_id
            )
            WHERE apr.agreement_id = ?
              AND p.email IS NOT NULL
              AND p.email != ''
            "#,
        )
        .bind(agreement_id)
        .fetch_all(db)
        .await?;

        Ok(rows)
    }

    pub async fn transaction_party_emails(
        db: &SqlitePool,
        transaction_id: i64,
    ) -> SqlxResult<Vec<String>> {
        let rows = sqlx::query_scalar::<_, String>(
            r#"
            SELECT DISTINCT p.email
            FROM operational_transactions ot
            JOIN parties p ON (
                p.id = ot.from_party_id
                OR p.id = ot.to_party_id
            )
            WHERE ot.id = ?
              AND p.email IS NOT NULL
              AND p.email != ''
            "#,
        )
        .bind(transaction_id)
        .fetch_all(db)
        .await?;

        Ok(rows)
    }
}
