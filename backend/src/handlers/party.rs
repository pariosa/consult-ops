use actix_web::{HttpResponse, Responder, ResponseError, web};

use crate::auth::permissions::can_manage_agreements;
use crate::auth_context::AuthUser;
use crate::db::Db;
use crate::models::party::{CreateParty, Party};
use crate::models::party_payment_profile::{PartyPaymentProfile, UpsertPartyPaymentProfile};
use crate::services::authz::{require_org_member, require_org_role};
use crate::services::event_service::EventService;

async fn organization_id_for_party(db: &Db, party_id: i64) -> Result<i64, sqlx::Error> {
    sqlx::query_scalar::<_, i64>(
        r#"
        SELECT organization_id
        FROM parties
        WHERE id = $1
        "#,
    )
    .bind(party_id)
    .fetch_one(db.pool.as_ref())
    .await
}

pub async fn list_organization_parties(
    db: web::Data<Db>,
    auth: AuthUser,
    path: web::Path<i64>,
) -> impl Responder {
    let organization_id = path.into_inner();

    if !can_manage_agreements(&auth.user_type) {
        return HttpResponse::Forbidden().json(serde_json::json!({
            "error": "You do not have permission to view parties."
        }));
    }

    if let Err(err) = require_org_member(db.pool.as_ref(), auth.id, organization_id).await {
        return err.error_response();
    }

    match Party::for_organization(db.pool.as_ref(), organization_id).await {
        Ok(parties) => HttpResponse::Ok().json(parties),
        Err(err) => {
            eprintln!("list_organization_parties error: {:?}", err);
            HttpResponse::InternalServerError().body(err.to_string())
        }
    }
}

pub async fn create_organization_party(
    db: web::Data<Db>,
    auth: AuthUser,
    path: web::Path<i64>,
    payload: web::Json<CreateParty>,
) -> impl Responder {
    let organization_id = path.into_inner();

    if !can_manage_agreements(&auth.user_type) {
        return HttpResponse::Forbidden().json(serde_json::json!({
            "error": "You do not have permission to create parties."
        }));
    }

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

    match Party::create(db.pool.as_ref(), organization_id, payload.into_inner()).await {
        Ok(party) => {
            let _ = EventService::record_event(
                db.pool.as_ref(),
                organization_id,
                Some(auth.id),
                "party",
                party.id,
                "PartyCreated",
                None,
                Some("created"),
                serde_json::json!({
                    "party_type": party.party_type,
                    "name": party.name,
                    "email": party.email,
                    "is_verified": party.is_verified
                }),
            )
            .await;

            HttpResponse::Created().json(party)
        }
        Err(err) => {
            eprintln!("create_organization_party error: {:?}", err);
            HttpResponse::InternalServerError().body(err.to_string())
        }
    }
}

pub async fn create_party_from_client(
    db: web::Data<Db>,
    auth: AuthUser,
    path: web::Path<(i64, i64)>,
) -> impl Responder {
    let (organization_id, client_id) = path.into_inner();

    if !can_manage_agreements(&auth.user_type) {
        return HttpResponse::Forbidden().json(serde_json::json!({
            "error": "You do not have permission to create verified client parties."
        }));
    }

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

    match Party::create_verified_client_party(db.pool.as_ref(), organization_id, client_id).await {
        Ok(party) => {
            let _ = EventService::record_event(
                db.pool.as_ref(),
                organization_id,
                Some(auth.id),
                "party",
                party.id,
                "VerifiedClientPartyCreated",
                None,
                Some("verified"),
                serde_json::json!({
                    "party_id": party.id,
                    "linked_client_id": party.linked_client_id,
                    "name": party.name,
                    "email": party.email,
                    "is_verified": party.is_verified
                }),
            )
            .await;

            HttpResponse::Created().json(party)
        }
        Err(err) => {
            eprintln!("create_party_from_client error: {:?}", err);
            HttpResponse::InternalServerError().body(err.to_string())
        }
    }
}

pub async fn create_party_from_user(
    db: web::Data<Db>,
    auth: AuthUser,
    path: web::Path<(i64, i64)>,
) -> impl Responder {
    let (organization_id, user_id) = path.into_inner();

    if !can_manage_agreements(&auth.user_type) {
        return HttpResponse::Forbidden().json(serde_json::json!({
            "error": "You do not have permission to create verified user parties."
        }));
    }

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

    match Party::create_verified_user_party(
        db.pool.as_ref(),
        organization_id,
        user_id,
        "contractor",
    )
    .await
    {
        Ok(party) => {
            let _ = EventService::record_event(
                db.pool.as_ref(),
                organization_id,
                Some(auth.id),
                "party",
                party.id,
                "VerifiedUserPartyCreated",
                None,
                Some("verified"),
                serde_json::json!({
                    "party_id": party.id,
                    "linked_user_id": party.linked_user_id,
                    "linked_organization_id": party.linked_organization_id,
                    "name": party.name,
                    "email": party.email,
                    "party_type": party.party_type,
                    "is_verified": party.is_verified
                }),
            )
            .await;

            HttpResponse::Created().json(party)
        }
        Err(err) => {
            eprintln!("create_party_from_user error: {:?}", err);
            HttpResponse::InternalServerError().body(err.to_string())
        }
    }
}

pub async fn get_party_payment_readiness(
    db: web::Data<Db>,
    auth: AuthUser,
    path: web::Path<i64>,
) -> impl Responder {
    let party_id = path.into_inner();

    if !can_manage_agreements(&auth.user_type) {
        return HttpResponse::Forbidden().json(serde_json::json!({
            "error": "You do not have permission to view party payment readiness."
        }));
    }

    let organization_id = match organization_id_for_party(&db, party_id).await {
        Ok(id) => id,
        Err(_) => return HttpResponse::NotFound().body("Party not found"),
    };

    if let Err(err) = require_org_member(db.pool.as_ref(), auth.id, organization_id).await {
        return err.error_response();
    }

    let party = match Party::find(db.pool.as_ref(), party_id).await {
        Ok(party) => party,
        Err(err) => return HttpResponse::NotFound().body(err.to_string()),
    };

    let profile = match PartyPaymentProfile::find_by_party(db.pool.as_ref(), party_id).await {
        Ok(profile) => profile,
        Err(err) => return HttpResponse::InternalServerError().body(err.to_string()),
    };

    HttpResponse::Ok().json(serde_json::json!({
        "party": party,
        "payment_profile": profile,
        "is_verified": party.is_verified == 1,
        "payer_ready": profile.as_ref().is_some_and(|p| {
            (p.payment_role == "payer" || p.payment_role == "both")
                && p.payer_authorization_status == "authorized"
        }),
        "payee_ready": profile.as_ref().is_some_and(|p| {
            (p.payment_role == "payee" || p.payment_role == "both")
                && p.payout_status == "ready"
        })
    }))
}

pub async fn upsert_party_payment_profile(
    db: web::Data<Db>,
    auth: AuthUser,
    path: web::Path<i64>,
    payload: web::Json<UpsertPartyPaymentProfile>,
) -> impl Responder {
    let party_id = path.into_inner();
    let input = payload.into_inner();

    if !can_manage_agreements(&auth.user_type) {
        return HttpResponse::Forbidden().json(serde_json::json!({
            "error": "You do not have permission to update party payment profiles."
        }));
    }

    let organization_id = match organization_id_for_party(&db, party_id).await {
        Ok(id) => id,
        Err(_) => return HttpResponse::NotFound().body("Party not found"),
    };

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

    match PartyPaymentProfile::upsert_basic(
        db.pool.as_ref(),
        party_id,
        organization_id,
        &input.payment_role,
        input.payer_authorization_scope,
    )
    .await
    {
        Ok(profile) => HttpResponse::Ok().json(profile),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn verify_party(
    db: web::Data<Db>,
    auth: AuthUser,
    path: web::Path<i64>,
) -> impl Responder {
    let party_id = path.into_inner();

    if !can_manage_agreements(&auth.user_type) {
        return HttpResponse::Forbidden().json(serde_json::json!({
            "error": "You do not have permission to verify parties."
        }));
    }

    let organization_id = match organization_id_for_party(&db, party_id).await {
        Ok(id) => id,
        Err(_) => return HttpResponse::NotFound().body("Party not found"),
    };

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

    match Party::verify(db.pool.as_ref(), party_id, "admin").await {
        Ok(party) => {
            let _ = EventService::record_event(
                db.pool.as_ref(),
                party.organization_id,
                Some(auth.id),
                "party",
                party.id,
                "PartyVerified",
                None,
                Some("verified"),
                serde_json::json!({
                    "party_id": party.id,
                    "verification_method": party.verification_method
                }),
            )
            .await;

            HttpResponse::Ok().json(party)
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn mark_party_payout_ready_dev(
    db: web::Data<Db>,
    auth: AuthUser,
    path: web::Path<i64>,
) -> impl Responder {
    let party_id = path.into_inner();

    if !can_manage_agreements(&auth.user_type) {
        return HttpResponse::Forbidden().json(serde_json::json!({
            "error": "You do not have permission to mark payout readiness."
        }));
    }

    let organization_id = match organization_id_for_party(&db, party_id).await {
        Ok(id) => id,
        Err(_) => return HttpResponse::NotFound().body("Party not found"),
    };

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

    match PartyPaymentProfile::mark_payout_ready(
        db.pool.as_ref(),
        party_id,
        format!("acct_dev_party_{}", party_id),
    )
    .await
    {
        Ok(profile) => HttpResponse::Ok().json(profile),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn mark_party_payer_authorized_dev(
    db: web::Data<Db>,
    auth: AuthUser,
    path: web::Path<i64>,
) -> impl Responder {
    let party_id = path.into_inner();

    if !can_manage_agreements(&auth.user_type) {
        return HttpResponse::Forbidden().json(serde_json::json!({
            "error": "You do not have permission to authorize payer profile."
        }));
    }

    let organization_id = match organization_id_for_party(&db, party_id).await {
        Ok(id) => id,
        Err(_) => return HttpResponse::NotFound().body("Party not found"),
    };

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

    match PartyPaymentProfile::mark_payer_authorized(
        db.pool.as_ref(),
        party_id,
        format!("cus_dev_party_{}", party_id),
        format!("pm_dev_party_{}", party_id),
        "agreement".to_string(),
    )
    .await
    {
        Ok(profile) => HttpResponse::Ok().json(profile),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
