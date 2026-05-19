use actix_web::{HttpResponse, Responder, web};

use crate::auth::permissions::can_manage_agreements;
use crate::auth_context::AuthUser;
use crate::db::Db;
use crate::models::party::{CreateParty, Party};
use crate::models::party_payment_profile::{PartyPaymentProfile, UpsertPartyPaymentProfile};
use crate::services::event_service::EventService;

pub async fn list_organization_parties(
    db: web::Data<Db>,
    auth: AuthUser,
    path: web::Path<i64>,
) -> impl Responder {
    if !can_manage_agreements(&auth.user_type) {
        return HttpResponse::Forbidden().json(serde_json::json!({
            "error": "You do not have permission to view parties."
        }));
    }

    let organization_id = path.into_inner();

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
    if !can_manage_agreements(&auth.user_type) {
        return HttpResponse::Forbidden().json(serde_json::json!({
            "error": "You do not have permission to create parties."
        }));
    }

    let organization_id = path.into_inner();

    match Party::create(db.pool.as_ref(), organization_id, payload.into_inner()).await {
        Ok(party) => {
            let _ = EventService::record_event(
                db.pool.as_ref(),
                organization_id,
                None,
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
    if !can_manage_agreements(&auth.user_type) {
        return HttpResponse::Forbidden().json(serde_json::json!({
            "error": "You do not have permission to create verified client parties."
        }));
    }

    let (organization_id, client_id) = path.into_inner();

    match Party::create_verified_client_party(db.pool.as_ref(), organization_id, client_id).await {
        Ok(party) => {
            let _ = EventService::record_event(
                db.pool.as_ref(),
                organization_id,
                None,
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
    if !can_manage_agreements(&auth.user_type) {
        return HttpResponse::Forbidden().json(serde_json::json!({
            "error": "You do not have permission to create verified user parties."
        }));
    }

    let (organization_id, user_id) = path.into_inner();

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
                None,
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
    if !can_manage_agreements(&auth.user_type) {
        return HttpResponse::Forbidden().json(serde_json::json!({
            "error": "You do not have permission to view party payment readiness."
        }));
    }

    let party_id = path.into_inner();

    let party = match sqlx::query_as::<_, Party>("SELECT * FROM parties WHERE id = ?")
        .bind(party_id)
        .fetch_one(db.pool.as_ref())
        .await
    {
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
            p.payment_role == "payer" || p.payment_role == "both"
        }) && profile.as_ref().is_some_and(|p| {
            p.payer_authorization_status == "authorized"
        }),
        "payee_ready": profile.as_ref().is_some_and(|p| {
            p.payment_role == "payee" || p.payment_role == "both"
        }) && profile.as_ref().is_some_and(|p| {
            p.payout_status == "ready"
        })
    }))
}

pub async fn upsert_party_payment_profile(
    db: web::Data<Db>,
    auth: AuthUser,
    path: web::Path<i64>,
    payload: web::Json<UpsertPartyPaymentProfile>,
) -> impl Responder {
    if !can_manage_agreements(&auth.user_type) {
        return HttpResponse::Forbidden().json(serde_json::json!({
            "error": "You do not have permission to update party payment profiles."
        }));
    }

    let party_id = path.into_inner();
    let input = payload.into_inner();

    let party = match sqlx::query_as::<_, Party>("SELECT * FROM parties WHERE id = ?")
        .bind(party_id)
        .fetch_one(db.pool.as_ref())
        .await
    {
        Ok(party) => party,
        Err(err) => return HttpResponse::NotFound().body(err.to_string()),
    };

    match PartyPaymentProfile::upsert_basic(
        db.pool.as_ref(),
        party.id,
        party.organization_id,
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
    if !can_manage_agreements(&auth.user_type) {
        return HttpResponse::Forbidden().json(serde_json::json!({
            "error": "You do not have permission to verify parties."
        }));
    }

    let party_id = path.into_inner();

    let result = sqlx::query_as::<_, Party>(
        r#"
        UPDATE parties
        SET is_verified = 1,
            verification_status = 'verified',
            verified_at = datetime('now'),
            verification_method = 'admin'
        WHERE id = ?
        RETURNING *
        "#,
    )
    .bind(party_id)
    .fetch_one(db.pool.as_ref())
    .await;

    match result {
        Ok(party) => HttpResponse::Ok().json(party),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn mark_party_payout_ready_dev(
    db: web::Data<Db>,
    auth: AuthUser,
    path: web::Path<i64>,
) -> impl Responder {
    if !can_manage_agreements(&auth.user_type) {
        return HttpResponse::Forbidden().json(serde_json::json!({
            "error": "You do not have permission to mark payout readiness."
        }));
    }

    let party_id = path.into_inner();

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
    if !can_manage_agreements(&auth.user_type) {
        return HttpResponse::Forbidden().json(serde_json::json!({
            "error": "You do not have permission to authorize payer profile."
        }));
    }

    let party_id = path.into_inner();

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
