// src/services/event_service.rs
use sqlx::SqlitePool;

use crate::domain::engagement_state::EngagementTransition;

pub struct EventService;

impl EventService {
    pub async fn record_engagement_transition(
        pool: &SqlitePool,
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

        sqlx::query!(
            r#"
            INSERT INTO operational_events (
                organization_id,
                actor_user_id,
                entity_type,
                entity_id,
                event_type,
                from_status,
                to_status,
                metadata
            )
            VALUES ($1, $2, 'engagement', $3, $4, $5, $6, '{}')
            "#,
            organization_id,
            actor_user_id,
            engagement_id,
            event_type,
            from_status,
            to_status
        )
        .execute(pool)
        .await
        .map_err(|err| err.to_string())?;

        Ok(())
    }
}
