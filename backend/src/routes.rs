use crate::auth::{forgot_password, login, register, reset_password};
use crate::handlers::{
    approve_engagement_milestone, create_client, create_contract, create_engagement_milestone,
    create_for_project, create_invoice, create_organization_client, create_organization_member,
    create_organization_project, create_payment, create_project, create_user,
    delete_organization_member, generate_for_engagement, get_admin_summary,
    get_client_portal_summary, get_clients, get_contracts, get_invoices, get_me,
    get_my_organization, get_organization, get_organization_clients, get_organization_contracts,
    get_organization_invoices, get_organization_members, get_organization_payments,
    get_organization_projects, get_payments, get_project_portal_summary, get_projects,
    get_user_by_id, get_users, list_engagement_milestones, list_for_project, mark_contract_sent,
    mark_engagement_milestone_paid, mark_signed, show, submit_engagement_milestone,
    update_organization, update_organization_member, update_user_type,
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
            ),
    );
}
