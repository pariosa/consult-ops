use crate::auth::{
    auth_me, forgot_password, list_auth_sessions, login, logout, register, resend_verification,
    reset_password, revoke_auth_session, verify_email,
};
use crate::handlers::{
    accept_organization_invitation, activate_engagement, approve_engagement_milestone,
    assign_platform_user_to_organization, attach_checkout_session, cancel_engagement,
    cancel_subscription_dev, cancel_transaction, complete_engagement, create_activation_checkout,
    create_activation_fee, create_agreement_payout_rule, create_client, create_contract,
    create_engagement_billing, create_engagement_milestone, create_for_project, create_invoice,
    create_my_organization, create_organization_agreement, create_organization_client,
    create_organization_member, create_organization_party, create_organization_project,
    create_party_from_client, create_party_from_user, create_payment, create_payout_rule,
    create_platform_organization, create_platform_user, create_project, create_user,
    delete_organization_member, disable_user, dispute_engagement, enable_user,
    force_password_reset, generate_for_engagement, get_admin_summary, get_client_portal_summary,
    get_clients, get_contracts, get_invoices, get_me, get_my_organization, get_organization,
    get_organization_clients, get_organization_contracts, get_organization_finance_summary,
    get_organization_invoices, get_organization_members, get_organization_party_balances,
    get_organization_payments, get_organization_projects, get_organization_subscription,
    get_party_payment_readiness, get_payments, get_project_portal_summary, get_projects,
    get_user_by_id, get_user_memberships, get_users, invite_organization_member,
    list_agreement_payout_rules, list_engagement_billing, list_engagement_events,
    list_engagement_milestones, list_engagement_transactions, list_for_project,
    list_my_notifications, list_my_organizations, list_organization_agreements,
    list_organization_events, list_organization_invitations, list_organization_members,
    list_organization_parties, list_organization_transactions, list_payout_rules,
    list_platform_organization_members, list_platform_organizations, list_platform_users,
    lock_agreement, mark_all_notifications_read, mark_billing_paid, mark_contract_sent,
    mark_engagement_milestone_paid, mark_notification_read, mark_party_payer_authorized_dev,
    mark_party_payout_ready_dev, mark_signed, mark_subscription_active_dev,
    mark_transaction_failed, mark_transaction_paid, mark_transaction_processing,
    reopen_engagement_milestone, revoke_user_sessions, set_current_organization, show,
    stripe_webhook, submit_engagement_milestone, update_engagement_milestone, update_organization,
    update_organization_member, update_user_type, upsert_organization_subscription,
    upsert_party_payment_profile, verify_party,
};
use actix_web::web;
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            // Auth
            .route("/auth/register", web::post().to(register))
            .route("/auth/login", web::post().to(login))
            .route("/auth/forgot-password", web::post().to(forgot_password))
            .route("/auth/reset-password", web::post().to(reset_password))
            .route("/auth/logout", web::post().to(logout))
            .route("/auth/verify-email", web::post().to(verify_email))
            .route("/auth/me", web::get().to(auth_me))
            .route("/admin/users/{id}/disable", web::patch().to(disable_user))
            .route("/admin/users/{id}/enable", web::patch().to(enable_user))
            .route(
                "/admin/users/{id}/memberships",
                web::get().to(get_user_memberships),
            )
            .route(
                "/admin/users/{id}/force-password-reset",
                web::post().to(force_password_reset),
            )
            .route(
                "/admin/users/{id}/sessions",
                web::delete().to(revoke_user_sessions),
            )
            // Logged-in context
            .route("/me", web::get().to(get_me))
            .route("/me/organization", web::get().to(get_my_organization))
            .route("/me/organizations", web::get().to(list_my_organizations))
            .route(
                "/me/current-organization",
                web::post().to(set_current_organization),
            )
            .route("/me/organizations", web::post().to(create_my_organization))
            .route(
                "/organizations/{id}/invitations",
                web::post().to(invite_organization_member),
            )
            .route(
                "/organization-invitations/accept",
                web::post().to(accept_organization_invitation),
            )
            .route(
                "/auth/resend-verification",
                web::post().to(resend_verification),
            )
            .route("/auth/sessions", web::get().to(list_auth_sessions))
            .route("/auth/sessions/{id}", web::delete().to(revoke_auth_session))
            // Logged-in context
            .route("/me", web::get().to(get_me))
            .route("/me/organization", web::get().to(get_my_organization))
            // Admin / users
            .route("/admin/users", web::get().to(get_users))
            .route("/admin/users", web::post().to(create_user))
            .route("/admin/users/{id}", web::get().to(get_user_by_id))
            .route("/admin/users/{id}/type", web::patch().to(update_user_type))
            .route("/admin/summary", web::get().to(get_admin_summary))
            // Organizations
            .route("/organizations/{id}", web::get().to(get_organization))
            .route("/organizations/{id}", web::patch().to(update_organization))
            .route(
                "/organizations/{id}/members",
                web::get().to(get_organization_members),
            )
            .route(
                "/organizations/{id}/members",
                web::post().to(create_organization_member),
            )
            .route(
                "/organization-members/{id}",
                web::patch().to(update_organization_member),
            )
            .route(
                "/organization-members/{id}",
                web::delete().to(delete_organization_member),
            )
            // Organization-scoped resources
            .route(
                "/organizations/{id}/projects",
                web::get().to(get_organization_projects),
            )
            .route(
                "/organizations/{id}/projects",
                web::post().to(create_organization_project),
            )
            .route(
                "/organizations/{id}/clients",
                web::get().to(get_organization_clients),
            )
            .route(
                "/organizations/{id}/clients",
                web::post().to(create_organization_client),
            )
            //subscription routes
            .route(
                "/organizations/{id}/subscription",
                web::get().to(get_organization_subscription),
            )
            .route(
                "/organizations/{id}/subscription",
                web::put().to(upsert_organization_subscription),
            )
            .route(
                "/organizations/{id}/subscription/mark-active/dev",
                web::post().to(mark_subscription_active_dev),
            )
            .route(
                "/organizations/{id}/subscription/cancel/dev",
                web::post().to(cancel_subscription_dev),
            )
            // Portal summaries
            .route(
                "/client-portal/summary",
                web::get().to(get_client_portal_summary),
            )
            .route(
                "/project-portal/summary",
                web::get().to(get_project_portal_summary),
            )
            // backend/src/routes.rs
            .route(
                "/organizations/{id}/contracts",
                web::get().to(get_organization_contracts),
            )
            .route(
                "/organizations/{id}/invoices",
                web::get().to(get_organization_invoices),
            )
            .route(
                "/organizations/{id}/payments",
                web::get().to(get_organization_payments),
            )
            // Existing global routes for now
            .route("/clients", web::get().to(get_clients))
            .route("/clients", web::post().to(create_client))
            .route("/projects", web::get().to(get_projects))
            .route("/projects", web::post().to(create_project))
            .route("/contracts", web::get().to(get_contracts))
            .route("/contracts", web::post().to(create_contract))
            .route("/invoices", web::get().to(get_invoices))
            .route("/invoices", web::post().to(create_invoice))
            .route("/payments", web::get().to(get_payments))
            .route("/payments", web::post().to(create_payment))
            .route(
                "/engagements/{id}/billing",
                web::get().to(list_engagement_billing),
            )
            .route(
                "/engagements/{id}/billing",
                web::post().to(create_engagement_billing),
            )
            .route(
                "/engagements/{id}/activation-fee",
                web::post().to(create_activation_fee),
            )
            .route(
                "/engagement-billing/{id}/checkout-session",
                web::patch().to(attach_checkout_session),
            )
            .route(
                "/engagement-billing/{id}/mark-paid",
                web::post().to(mark_billing_paid),
            )
            .route(
                "/projects/{project_id}/engagements",
                web::post().to(create_for_project),
            )
            .route(
                "/projects/{project_id}/engagements",
                web::get().to(list_for_project),
            )
            .route(
                "/debug/projects/{project_id}/engagements",
                web::get().to(list_for_project),
            )
            .route(
                "/engagements/{id}/events",
                web::get().to(list_engagement_events),
            )
            .route(
                "/organizations/{id}/events",
                web::get().to(list_organization_events),
            )
            .route("/engagements/{id}", web::get().to(show))
            .route(
                "/engagements/{id}/mark-contract-sent",
                web::post().to(mark_contract_sent),
            )
            .route("/engagements/{id}/mark-signed", web::post().to(mark_signed))
            .route(
                "/engagements/{id}/milestones",
                web::post().to(create_engagement_milestone),
            )
            .route(
                "/engagements/{id}/milestones",
                web::get().to(list_engagement_milestones),
            )
            .route(
                "/engagements/milestones/{id}",
                web::patch().to(update_engagement_milestone),
            )
            .route(
                "/engagements/milestones/{id}/reopen",
                web::post().to(reopen_engagement_milestone),
            )
            .route(
                "/milestones/{id}/submit",
                web::post().to(submit_engagement_milestone),
            )
            .route(
                "/milestones/{id}/approve",
                web::post().to(approve_engagement_milestone),
            )
            .route(
                "/milestones/{id}/mark-paid",
                web::post().to(mark_engagement_milestone_paid),
            )
            .route(
                "/engagements/{id}/software-contract",
                web::post().to(generate_for_engagement),
            )
            .route(
                "/engagements/{id}/activate",
                web::post().to(activate_engagement),
            )
            .route(
                "/engagements/{id}/complete",
                web::post().to(complete_engagement),
            )
            .route(
                "/engagements/{id}/cancel",
                web::post().to(cancel_engagement),
            )
            .route(
                "/engagements/{id}/activation-checkout",
                web::post().to(create_activation_checkout),
            )
            .route("/webhooks/stripe", web::post().to(stripe_webhook))
            .route(
                "/engagements/{id}/dispute",
                web::post().to(dispute_engagement),
            )
            .route(
                "/organizations/{id}/parties",
                web::get().to(list_organization_parties),
            )
            .route(
                "/organizations/{id}/parties",
                web::post().to(create_organization_party),
            )
            .route(
                "/organizations/{id}/agreements",
                web::get().to(list_organization_agreements),
            )
            .route(
                "/organizations/{id}/agreements",
                web::post().to(create_organization_agreement),
            )
            .route(
                "/agreements/{id}/payout-rules",
                web::get().to(list_agreement_payout_rules),
            )
            .route(
                "/agreements/{id}/payout-rules",
                web::post().to(create_agreement_payout_rule),
            )
            .route(
                "/engagements/{id}/transactions",
                web::get().to(list_engagement_transactions),
            )
            .route(
                "/organizations/{id}/transactions",
                web::get().to(list_organization_transactions),
            )
            .route(
                "/operational-transactions/{id}/mark-processing",
                web::post().to(mark_transaction_processing),
            )
            .route(
                "/operational-transactions/{id}/mark-paid",
                web::post().to(mark_transaction_paid),
            )
            .route(
                "/operational-transactions/{id}/mark-failed",
                web::post().to(mark_transaction_failed),
            )
            .route(
                "/operational-transactions/{id}/cancel",
                web::post().to(cancel_transaction),
            )
            .route(
                "/organizations/{organization_id}/parties/from-client/{client_id}",
                web::post().to(create_party_from_client),
            )
            .route(
                "/organizations/{organization_id}/parties/from-user/{user_id}",
                web::post().to(create_party_from_user),
            )
            .route(
                "/organizations/{id}/finance-summary",
                web::get().to(get_organization_finance_summary),
            )
            .route(
                "/organizations/{id}/party-balances",
                web::get().to(get_organization_party_balances),
            )
            .route(
                "/organizations/{id}/members",
                web::get().to(list_organization_members),
            )
            .route(
                "/organizations/{id}/invitations",
                web::get().to(list_organization_invitations),
            )
            .route(
                "/organizations/{id}/invitations",
                web::post().to(invite_organization_member),
            )
            .route(
                "/organization-invitations/accept",
                web::post().to(accept_organization_invitation),
            )
            .route(
                "/platform/organizations",
                web::get().to(list_platform_organizations),
            )
            .route(
                "/platform/organizations",
                web::post().to(create_platform_organization),
            )
            .route("/platform/users", web::get().to(list_platform_users))
            .route("/platform/users", web::post().to(create_platform_user))
            .route(
                "/platform/organizations/{id}/members",
                web::get().to(list_platform_organization_members),
            )
            .route(
                "/platform/organizations/{id}/members",
                web::post().to(assign_platform_user_to_organization),
            )
            .route("/notifications", web::get().to(list_my_notifications))
            .route(
                "/notifications/read-all",
                web::post().to(mark_all_notifications_read),
            )
            .route(
                "/notifications/{id}/read",
                web::post().to(mark_notification_read),
            )
            .route(
                "/parties/{id}/payment-readiness",
                web::get().to(get_party_payment_readiness),
            )
            .route(
                "/parties/{id}/payment-profile",
                web::post().to(upsert_party_payment_profile),
            )
            .route("/parties/{id}/verify", web::post().to(verify_party))
            .route(
                "/parties/{id}/payout-ready/dev",
                web::post().to(mark_party_payout_ready_dev),
            )
            .route(
                "/parties/{id}/payer-authorized/dev",
                web::post().to(mark_party_payer_authorized_dev),
            )
            .route("/agreements/{id}/lock", web::post().to(lock_agreement))
            .route(
                "/agreements/{id}/payout-rules",
                web::post().to(create_payout_rule),
            )
            .route(
                "/agreements/{id}/payout-rules",
                web::get().to(list_payout_rules),
            ),
    );
}
