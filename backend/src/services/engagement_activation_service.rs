use crate::db::Db;
use sqlx::Row;

pub async fn maybe_activate_engagement(
    db: &Db,
    engagement_id: i64,
    organization_id: i64,
    actor_user_id: Option<i64>,
) -> Result<bool, String> {
    let mut tx = db.pool.begin().await.map_err(|e| e.to_string())?;

    let row = sqlx::query(
        r#"
        SELECT status
        FROM engagements
        WHERE id = $1 AND organization_id = $2
        FOR UPDATE
        "#,
    )
    .bind(engagement_id)
    .bind(organization_id)
    .fetch_one(&mut *tx)
    .await
    .map_err(|e| e.to_string())?;

    let status: String = row.try_get("status").map_err(|e| e.to_string())?;

    if status == "active" {
        return Ok(false);
    }

    if status != "awaiting_payment" {
        return Ok(false);
    }

    // Confirm an actual paid activation-fee record.
    // The payment-writing endpoint must independently
    // verify Stripe or be strictly development-only.
    let paid: bool = sqlx::query_scalar(
        r#"
        SELECT EXISTS (
            SELECT 1
            FROM engagement_billing
            WHERE engagement_id = $1
              AND organization_id = $2
              AND billing_type = 'activation_fee'
              AND status = 'paid'
              AND amount_cents = 1000
              AND LOWER(currency) = 'usd'
        )
        "#,
    )
    .bind(engagement_id)
    .bind(organization_id)
    .fetch_one(&mut *tx)
    .await
    .map_err(|e| e.to_string())?;

    if !paid {
        return Ok(false);
    }

    sqlx::query(
        r#"
        UPDATE engagements
        SET status = 'active',
            platform_fee_status = 'paid',
            updated_at = CURRENT_TIMESTAMP
        WHERE id = $1 AND organization_id = $2
        "#,
    )
    .bind(engagement_id)
    .bind(organization_id)
    .execute(&mut *tx)
    .await
    .map_err(|e| e.to_string())?;

    sqlx::query(
        r#"
        INSERT INTO operational_events (
            organization_id,
            actor_user_id,
            entity_type,
            entity_id,
            event_type,
            from_status,
            to_status,
            metadata,
            created_at,
            updated_at
        )
        VALUES (
            $1, $2, 'engagement', $3,
            'PaymentReceived',
            'awaiting_payment',
            'active',
            '{}',
            CURRENT_TIMESTAMP,
            CURRENT_TIMESTAMP
        )
        "#,
    )
    .bind(organization_id)
    .bind(actor_user_id)
    .bind(engagement_id)
    .execute(&mut *tx)
    .await
    .map_err(|e| e.to_string())?;

    tx.commit().await.map_err(|e| e.to_string())?;

    Ok(true)
}
