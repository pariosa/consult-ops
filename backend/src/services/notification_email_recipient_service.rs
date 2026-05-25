use sqlx::{PgPool, Result as SqlxResult};

pub struct NotificationRecipientService;

impl NotificationRecipientService {
    pub async fn engagement_client_email(
        db: &PgPool,
        engagement_id: i64,
    ) -> SqlxResult<Option<String>> {
        sqlx::query_scalar::<_, String>(
            r#"
            SELECT c.email
            FROM engagements e
            JOIN clients c ON c.id = e.client_id
            WHERE e.id = $1
            LIMIT 1
            "#,
        )
        .bind(engagement_id)
        .fetch_optional(db)
        .await
    }

    pub async fn agreement_party_emails(db: &PgPool, agreement_id: i64) -> SqlxResult<Vec<String>> {
        sqlx::query_scalar::<_, String>(
            r#"
            SELECT DISTINCT p.email
            FROM agreement_payout_rules apr
            JOIN parties p ON (
                p.id = apr.from_party_id
                OR p.id = apr.to_party_id
            )
            WHERE apr.agreement_id = $1
              AND p.email IS NOT NULL
              AND p.email != ''
            "#,
        )
        .bind(agreement_id)
        .fetch_all(db)
        .await
    }

    pub async fn transaction_party_emails(
        db: &PgPool,
        transaction_id: i64,
    ) -> SqlxResult<Vec<String>> {
        sqlx::query_scalar::<_, String>(
            r#"
            SELECT DISTINCT p.email
            FROM operational_transactions ot
            JOIN parties p ON (
                p.id = ot.from_party_id
                OR p.id = ot.to_party_id
            )
            WHERE ot.id = $1
              AND p.email IS NOT NULL
              AND p.email != ''
            "#,
        )
        .bind(transaction_id)
        .fetch_all(db)
        .await
    }
}
