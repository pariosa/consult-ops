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
