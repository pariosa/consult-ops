// src/services/operations_kernel_service.rs
use sqlx::PgPool;

use crate::domain::engagement_state::{EngagementEvent, EngagementStatus};
use crate::services::event_service::EventService;

pub struct OperationsKernelService;

impl OperationsKernelService {
    pub async fn apply_engagement_event(
        pool: &PgPool,
        organization_id: i64,
        engagement_id: i64,
        actor_user_id: Option<i64>,
        current_status: EngagementStatus,
        event: EngagementEvent,
    ) -> Result<EngagementStatus, String> {
        let transition = current_status.transition(event)?;

        EventService::record_engagement_transition(
            pool,
            organization_id,
            engagement_id,
            actor_user_id,
            &transition,
        )
        .await?;

        Ok(transition.to)
    }
}
