use actix_web::{HttpResponse, Responder, web};
use chrono::{Duration, Utc};
use uuid::Uuid;

use crate::auth::permissions::can_manage_agreements;
use crate::auth_context::AuthUser;
use crate::db::Db;
use crate::models::organization_invitation::{
    CreateOrganizationInvitation, OrganizationInvitation,
};
use crate::models::organization_member::OrganizationMember;
use crate::services::event_service::EventService;

pub async fn list_organization_members(
    db: web::Data<Db>,
    auth: AuthUser,
    path: web::Path<i64>,
) -> impl Responder {
    if !can_manage_agreements(&auth.user_type) {
        return HttpResponse::Forbidden().json(serde_json::json!({
            "error": "You do not have permission to view organization members."
        }));
    }

    let organization_id = path.into_inner();

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
    if !can_manage_agreements(&auth.user_type) {
        return HttpResponse::Forbidden().json(serde_json::json!({
            "error": "You do not have permission to view organization invitations."
        }));
    }

    let organization_id = path.into_inner();

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
    if !can_manage_agreements(&auth.user_type) {
        return HttpResponse::Forbidden().json(serde_json::json!({
            "error": "You do not have permission to invite organization members."
        }));
    }

    let organization_id = path.into_inner();
    let input = payload.into_inner();

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

    let token = Uuid::new_v4().to_string();
    let expires_at = (Utc::now() + Duration::days(7)).to_rfc3339();

    match OrganizationInvitation::create(
        db.pool.as_ref(),
        organization_id,
        input.email.clone(),
        input.role.clone(),
        token.clone(),
        Some(auth.id),
        expires_at,
    )
    .await
    {
        Ok(invitation) => {
            let invite_url = format!("http://localhost:3000/invitations/accept?token={}", token);

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

            HttpResponse::Created().json(serde_json::json!({
                "invitation": invitation,
                "invite_url": invite_url,
                "email_preview": {
                    "to": input.email,
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
    query: web::Query<std::collections::HashMap<String, String>>,
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

            HttpResponse::Ok().json(member)
        }
        Err(err) => {
            eprintln!("accept invitation error: {:?}", err);
            HttpResponse::InternalServerError().body(err.to_string())
        }
    }
}
