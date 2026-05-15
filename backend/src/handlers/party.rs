use actix_web::{HttpResponse, Responder, web};

use crate::db::Db;
use crate::models::party::{CreateParty, Party};
use crate::services::event_service::EventService;

pub async fn list_organization_parties(db: web::Data<Db>, path: web::Path<i64>) -> impl Responder {
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
    path: web::Path<i64>,
    payload: web::Json<CreateParty>,
) -> impl Responder {
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
                    "email": party.email
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
    path: web::Path<(i64, i64)>,
) -> impl Responder {
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
                    "email": party.email
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
    path: web::Path<(i64, i64)>,
) -> impl Responder {
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
                    "party_type": party.party_type
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
