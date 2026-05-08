// src/domain/operational_event.rs

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OperationalEventType {
    EngagementContractSent,
    EngagementContractSigned,
    EngagementAwaitingPayment,
    EngagementActivated,
    EngagementMilestoneSubmitted,
    EngagementMilestoneApproved,
    EngagementOverdue,
    EngagementSuspended,
    EngagementResumed,
    EngagementCompleted,
    EngagementCancelled,
    EngagementDisputed,
}
