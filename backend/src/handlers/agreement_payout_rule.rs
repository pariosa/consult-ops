use actix_web::{HttpResponse, Responder, web};

use crate::db::Db;
use crate::models::agreement_payout_rule::{AgreementPayoutRule, CreateAgreementPayoutRule};
use crate::models::operational_agreement::OperationalAgreement;

pub async fn create_payout_rule(
    db: web::Data<Db>,
    path: web::Path<i64>,
    payload: web::Json<CreateAgreementPayoutRule>,
) -> impl Responder {
    let agreement_id = path.into_inner();

    let agreement = match OperationalAgreement::find(db.pool.as_ref(), agreement_id).await {
        Ok(agreement) => agreement,
        Err(err) => return HttpResponse::NotFound().body(err.to_string()),
    };

    if agreement.status == "locked" || agreement.status == "suspended" {
        return HttpResponse::Conflict().body("Agreement is locked and cannot be modified.");
    }

    let input = CreateAgreementPayoutRule {
        from_party_id: payload.from_party_id,
        to_party_id: payload.to_party_id,
        rule_type: payload.rule_type.clone(),
        percent: payload.percent,
        amount_cents: payload.amount_cents,
        trigger_event: payload.trigger_event.clone(),
    };

    match AgreementPayoutRule::create(db.pool.as_ref(), agreement_id, input).await {
        Ok(rule) => HttpResponse::Ok().json(rule),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn list_payout_rules(db: web::Data<Db>, path: web::Path<i64>) -> impl Responder {
    let agreement_id = path.into_inner();

    match AgreementPayoutRule::for_agreement(db.pool.as_ref(), agreement_id).await {
        Ok(rules) => HttpResponse::Ok().json(rules),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
