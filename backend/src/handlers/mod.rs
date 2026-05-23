// src/handlers/mod.rs

pub mod agreement_payout_rule;
mod client;
mod contract;
pub mod contract_templates;
pub mod engagement;
pub mod engagement_billing;
pub mod engagement_milestone;
mod invoice;
pub mod notification;
pub mod onboarding_handler;
pub mod operational_agreement;
pub mod operational_event;
pub mod operational_finance;
pub mod operational_transaction;
mod organization;
pub mod organization_member;
pub mod party;
mod payment;
pub mod platform_admin;
mod project;
mod stripe_engagement_handler;
pub mod stripe_webhook;
pub mod subscription;
mod user;

pub use platform_admin::{
    assign_platform_user_to_organization, create_platform_organization, create_platform_user,
    list_platform_organization_members, list_platform_organizations, list_platform_users,
};

pub use agreement_payout_rule::{create_payout_rule, list_payout_rules};
pub use onboarding_handler::{
    create_my_organization, list_my_organizations, set_current_organization,
};

pub use client::{create_client, get_clients};
pub use contract::{create_contract, get_contracts};
pub use invoice::{create_invoice, get_invoices};
pub use operational_event::{list_engagement_events, list_organization_events};
pub use organization_member::{
    accept_organization_invitation, invite_organization_member, list_organization_invitations,
    list_organization_members,
};

pub use notification::{
    list_my_notifications, mark_all_notifications_read, mark_notification_read,
};
pub use organization::*;
pub use payment::{create_payment, get_payments};
pub use project::{create_project, get_projects};
pub use user::{create_user, get_user_by_id, get_users, update_user_type};

pub use contract_templates::generate_for_engagement;
pub use engagement::{
    activate_engagement, cancel_engagement, complete_engagement, create_for_project,
    dispute_engagement, list_for_project, mark_contract_sent, mark_signed, show,
};
pub use engagement_billing::{
    attach_checkout_session, create_activation_checkout, create_activation_fee,
    create_engagement_billing, list_engagement_billing, mark_billing_paid,
};
pub use engagement_milestone::{
    approve_engagement_milestone, create_engagement_milestone, list_engagement_milestones,
    mark_engagement_milestone_paid, reopen_engagement_milestone, submit_engagement_milestone,
    update_engagement_milestone,
};
pub use operational_agreement::{
    create_agreement_payout_rule, create_organization_agreement, list_agreement_payout_rules,
    list_organization_agreements, lock_agreement,
};
pub use party::{
    create_organization_party, create_party_from_client, create_party_from_user,
    get_party_payment_readiness, list_organization_parties, mark_party_payer_authorized_dev,
    mark_party_payout_ready_dev, upsert_party_payment_profile, verify_party,
};
pub use stripe_webhook::stripe_webhook;

pub use operational_transaction::{
    cancel_transaction, list_engagement_transactions, list_organization_transactions,
    mark_transaction_failed, mark_transaction_paid, mark_transaction_processing,
};

pub use operational_finance::{get_organization_finance_summary, get_organization_party_balances};
