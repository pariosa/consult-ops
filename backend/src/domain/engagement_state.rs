// src/domain/engagement_state.rs

//! Engagement lifecycle state machine
//!
//! this module defines valid engagement states and transitions.
//! All engagement status changes must pass through this module

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EngagementStatus {
    Draft,
    PendingSignature,
    AwaitingPayment,
    Active,
    MilestoneReview,
    Overdue,
    Suspended,
    Completed,
    Cancelled,
    Disputed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum EngagementEvent {
    ContractSent,
    ContractSigned,
    PaymentReceived,
    MilestoneSubmitted,
    MilestoneApproved,
    InvoiceOverdue,
    Suspend,
    Resume,
    Complete,
    Cancel,
    Dispute,
}

#[derive(Debug, Clone)]
pub struct EngagementTransition {
    pub from: EngagementStatus,
    pub to: EngagementStatus,
    pub event: EngagementEvent,
}

impl EngagementStatus {
    pub fn transition(self, event: EngagementEvent) -> Result<EngagementTransition, String> {
        use EngagementEvent::*;
        use EngagementStatus::*;

        let to = match (self, event) {
            (Draft, ContractSent) => PendingSignature,
            (PendingSignature, ContractSigned) => AwaitingPayment,
            (AwaitingPayment, PaymentReceived) => Active,

            (Active, MilestoneSubmitted) => MilestoneReview,
            (MilestoneReview, MilestoneApproved) => Active,

            (Active, InvoiceOverdue) => Overdue,
            (Overdue, Suspend) => Suspended,
            (Suspended, PaymentReceived) => Active,
            (Suspended, Resume) => Active,

            (Active, Complete) => Completed,

            (Draft, Cancel)
            | (PendingSignature, Cancel)
            | (AwaitingPayment, Cancel)
            | (Active, Cancel)
            | (Overdue, Cancel)
            | (Suspended, Cancel) => Cancelled,

            (Active, Dispute)
            | (MilestoneReview, Dispute)
            | (Overdue, Dispute)
            | (Suspended, Dispute) => Disputed,

            _ => {
                return Err(format!(
                    "Invalid engagement transition: {:?} + {:?}",
                    self, event
                ));
            }
        };

        Ok(EngagementTransition {
            from: self,
            to,
            event,
        })
    }

    pub fn is_terminal(self) -> bool {
        matches!(
            self,
            EngagementStatus::Completed | EngagementStatus::Cancelled | EngagementStatus::Disputed
        )
    }
    pub fn can_transition(self, event: EngagementEvent) -> bool {
        self.transition(event).is_ok()
    }
}
