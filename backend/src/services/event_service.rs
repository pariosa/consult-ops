// src/services/event_service.rs

use sqlx::PgPool;

use crate::domain::engagement_state::EngagementTransition;

pub struct EventService;

impl EventService {
    pub async fn record_engagement_transition(
        pool: &PgPool,
        organization_id: i64,
        engagement_id: i64,
        actor_user_id: Option<i64>,
        transition: &EngagementTransition,
    ) -> Result<(), String> {
        let event_type = format!("{:?}", transition.event);

        let from_status = serde_json::to_value(transition.from)
            .ok()
            .and_then(|v| v.as_str().map(|s| s.to_string()))
            .unwrap_or_else(|| format!("{:?}", transition.from).to_lowercase());

        let to_status = serde_json::to_value(transition.to)
            .ok()
            .and_then(|v| v.as_str().map(|s| s.to_string()))
            .unwrap_or_else(|| format!("{:?}", transition.to).to_lowercase());

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
            VALUES ($1, $2, 'engagement', $3, $4, $5, $6, '{}', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
            "#,
        )
        .bind(organization_id)
        .bind(actor_user_id)
        .bind(engagement_id)
        .bind(event_type)
        .bind(from_status)
        .bind(to_status)
        .execute(pool)
        .await
        .map_err(|err| err.to_string())?;

        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn record_event(
        pool: &PgPool,
        organization_id: i64,
        actor_user_id: Option<i64>,
        entity_type: &str,
        entity_id: i64,
        event_type: &str,
        from_status: Option<&str>,
        to_status: Option<&str>,
        metadata: serde_json::Value,
    ) -> Result<(), String> {
        let metadata_json = metadata.to_string();

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
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
            "#,
        )
        .bind(organization_id)
        .bind(actor_user_id)
        .bind(entity_type)
        .bind(entity_id)
        .bind(event_type)
        .bind(from_status)
        .bind(to_status)
        .bind(metadata_json)
        .execute(pool)
        .await
        .map_err(|err| err.to_string())?;

        Ok(())
    }
}
