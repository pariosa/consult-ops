use crate::db::Db;
use crate::services::event_service::EventService;
use chrono::Utc;

pub async fn maybe_activate_engagement(
    db: &Db,
    engagement_id: i64,
    organization_id: i64,
    actor_user_id: Option<i64>,
) -> Result<bool, String> {
    let row = sqlx::query_as::<_, (String, String)>(
        r#"
        SELECT status, platform_fee_status
        FROM engagements
        WHERE id = $1
          AND organization_id = $2
        "#,
    )
    .bind(engagement_id)
    .bind(organization_id)
    .fetch_one(db.pool.as_ref())
    .await
    .map_err(|err| err.to_string())?;

    let current_status = row.0;
    let platform_fee_status = row.1;

    if current_status == "activated" {
        return Ok(false);
    }

    let contract_ready = current_status == "signed"
        || current_status == "contract_signed"
        || current_status == "pending_signature";

    let payment_ready = platform_fee_status == "paid";

    if !contract_ready || !payment_ready {
        return Ok(false);
    }

    sqlx::query(
        r#"
        UPDATE engagements
        SET status = 'activated',
            updated_at = $1
        WHERE id = $2
          AND organization_id = $3
        "#,
    )
    .bind(Utc::now().to_rfc3339())
    .bind(engagement_id)
    .bind(organization_id)
    .execute(db.pool.as_ref())
    .await
    .map_err(|err| err.to_string())?;

    EventService::record_event(
        db.pool.as_ref(),
        organization_id,
        actor_user_id,
        "engagement",
        engagement_id,
        "EngagementActivated",
        Some(&current_status),
        Some("activated"),
        serde_json::json!({
            "trigger": "contract_ready_and_activation_fee_paid"
        }),
    )
    .await
    .map_err(|err| err.to_string())?;

    Ok(true)
}
