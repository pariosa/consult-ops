use actix_web::{HttpResponse, Responder, ResponseError, web};
use chrono::{Duration, Utc};
use std::collections::HashMap;
use uuid::Uuid;

use crate::auth_context::AuthUser;
use crate::db::Db;
use crate::models::organization_invitation::{
    CreateOrganizationInvitation, OrganizationInvitation,
};
use crate::models::organization_member::OrganizationMember;
use crate::services::authz::{require_org_member, require_org_role};
use crate::services::event_service::EventService;
use crate::services::notification_service::NotificationService;

fn frontend_url() -> String {
    std::env::var("FRONTEND_URL").unwrap_or_else(|_| "http://localhost:3000".to_string())
}

pub async fn list_organization_members(
    db: web::Data<Db>,
    auth: AuthUser,
    path: web::Path<i64>,
) -> impl Responder {
    let organization_id = path.into_inner();

    if let Err(err) = require_org_member(db.pool.as_ref(), auth.id, organization_id).await {
        return err.error_response();
    }

    match OrganizationMember::list_for_organization(db.pool.as_ref(), organization_id).await {
        Ok(members) => HttpResponse::Ok().json(members),
        Err(err) => {
            eprintln!("list_organization_members error: {:?}", err);
            HttpResponse::InternalServerError().body(err.to_string())
        }
    }
}

pub async fn list_organization_invitations(
    db: web::Data<Db>,
    auth: AuthUser,
    path: web::Path<i64>,
) -> impl Responder {
    let organization_id = path.into_inner();

    if let Err(err) = require_org_role(
        db.pool.as_ref(),
        auth.id,
        organization_id,
        &["owner", "admin"],
    )
    .await
    {
        return err.error_response();
    }

    match OrganizationInvitation::list_for_organization(db.pool.as_ref(), organization_id).await {
        Ok(invitations) => HttpResponse::Ok().json(invitations),
        Err(err) => {
            eprintln!("list_organization_invitations error: {:?}", err);
            HttpResponse::InternalServerError().body(err.to_string())
        }
    }
}

pub async fn invite_organization_member(
    db: web::Data<Db>,
    auth: AuthUser,
    path: web::Path<i64>,
    payload: web::Json<CreateOrganizationInvitation>,
) -> impl Responder {
    let organization_id = path.into_inner();
    let input = payload.into_inner();

    if let Err(err) = require_org_role(
        db.pool.as_ref(),
        auth.id,
        organization_id,
        &["owner", "admin"],
    )
    .await
    {
        return err.error_response();
    }

    let allowed_roles = [
        "admin",
        "finance_admin",
        "operations_manager",
        "contractor",
        "client_viewer",
        "member",
    ];

    if !allowed_roles.contains(&input.role.as_str()) {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Invalid organization role."
        }));
    }

    let email = input.email.trim().to_lowercase();

    if !email.contains('@') {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "A valid email is required."
        }));
    }

    let token = Uuid::new_v4().to_string();
    let expires_at = (Utc::now() + Duration::days(7)).to_rfc3339();

    match OrganizationInvitation::create(
        db.pool.as_ref(),
        organization_id,
        email.clone(),
        input.role.clone(),
        token.clone(),
        Some(auth.id),
        expires_at,
    )
    .await
    {
        Ok(invitation) => {
            let invite_url = format!("{}/invitations/accept?token={}", frontend_url(), token);

            let _ = EventService::record_event(
                db.pool.as_ref(),
                organization_id,
                Some(auth.id),
                "organization_invitation",
                invitation.id,
                "OrganizationInvitationCreated",
                None,
                Some("pending"),
                serde_json::json!({
                    "email": invitation.email,
                    "role": invitation.role,
                    "invite_url": invite_url
                }),
            )
            .await;

            if let Err(err) = NotificationService::notify_email(
                db.pool.as_ref(),
                invitation.organization_id,
                invitation.email.clone(),
                "organization_invitation_created",
                "You're invited to join Consult Ops",
                &format!(
                    "You've been invited to join Consult Ops as {}.\n\nAccept your invitation here:\n{}",
                    invitation.role,
                    invite_url
                ),
                Some("organization_invitation"),
                Some(invitation.id),
            )
            .await
            {
                eprintln!("invitation email error: {:?}", err);
            }

            HttpResponse::Created().json(serde_json::json!({
                "invitation": invitation,
                "invite_url": invite_url,
                "email_preview": {
                    "to": email,
                    "subject": "You're invited to join Consult Ops",
                    "body": format!(
                        "You've been invited to join an organization in Consult Ops as {}. Accept here: {}",
                        input.role,
                        invite_url
                    )
                }
            }))
        }
        Err(err) => {
            eprintln!("invite_organization_member error: {:?}", err);
            HttpResponse::InternalServerError().body(err.to_string())
        }
    }
}

pub async fn accept_organization_invitation(
    db: web::Data<Db>,
    auth: AuthUser,
    query: web::Query<HashMap<String, String>>,
) -> impl Responder {
    let Some(token) = query.get("token") else {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Missing invitation token."
        }));
    };

    let invitation =
        match OrganizationInvitation::find_pending_by_token(db.pool.as_ref(), token).await {
            Ok(invitation) => invitation,
            Err(err) => {
                eprintln!("find invitation error: {:?}", err);
                return HttpResponse::NotFound().json(serde_json::json!({
                    "error": "Invitation not found or already accepted."
                }));
            }
        };

    if invitation.email.to_lowercase() != auth.email.to_lowercase() {
        return HttpResponse::Forbidden().json(serde_json::json!({
            "error": "Invitation email does not match authenticated user."
        }));
    }

    let expires_at = match chrono::DateTime::parse_from_rfc3339(&invitation.expires_at) {
        Ok(date) => date.with_timezone(&Utc),
        Err(_) => {
            return HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Invalid invitation expiration."
            }));
        }
    };

    if expires_at < Utc::now() {
        let _ = OrganizationInvitation::mark_expired(db.pool.as_ref(), invitation.id).await;

        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Invitation has expired."
        }));
    }

    match OrganizationMember::upsert_active_member(
        db.pool.as_ref(),
        invitation.organization_id,
        auth.id,
        &invitation.role,
    )
    .await
    {
        Ok(member) => {
            let _ = OrganizationInvitation::mark_accepted(db.pool.as_ref(), invitation.id, auth.id)
                .await;

            let _ = sqlx::query(
                r#"
                UPDATE users
                SET current_organization_id = $1,
                    updated_at = CURRENT_TIMESTAMP
                WHERE id = $2
                  AND current_organization_id IS NULL
                "#,
            )
            .bind(invitation.organization_id)
            .bind(auth.id)
            .execute(db.pool.as_ref())
            .await;

            let _ = EventService::record_event(
                db.pool.as_ref(),
                invitation.organization_id,
                Some(auth.id),
                "organization_member",
                member.id,
                "OrganizationInvitationAccepted",
                None,
                Some("active"),
                serde_json::json!({
                    "invitation_id": invitation.id,
                    "user_id": auth.id,
                    "role": invitation.role
                }),
            )
            .await;

            if let Some(invited_by_user_id) = invitation.invited_by_user_id {
                if let Ok(admin_email) = sqlx::query_scalar::<_, String>(
                    r#"
                    SELECT email
                    FROM users
                    WHERE id = $1
                    "#,
                )
                .bind(invited_by_user_id)
                .fetch_one(db.pool.as_ref())
                .await
                {
                    let _ = NotificationService::notify_email(
                        db.pool.as_ref(),
                        invitation.organization_id,
                        admin_email,
                        "organization_invitation_accepted",
                        "Organization invitation accepted",
                        &format!(
                            "{} accepted their invitation as {}.",
                            auth.email, invitation.role
                        ),
                        Some("organization_member"),
                        Some(member.id),
                    )
                    .await;
                }
            }

            HttpResponse::Ok().json(member)
        }
        Err(err) => {
            eprintln!("accept invitation error: {:?}", err);
            HttpResponse::InternalServerError().body(err.to_string())
        }
    }
}
