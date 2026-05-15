use actix_web::{HttpResponse, Responder, web};

use crate::db::Db;
use crate::models::agreement_payout_rule::{AgreementPayoutRule, CreateAgreementPayoutRule};
use crate::models::operational_agreement::{CreateOperationalAgreement, OperationalAgreement};
use crate::services::event_service::EventService;

pub async fn list_organization_agreements(
    db: web::Data<Db>,
    path: web::Path<i64>,
) -> impl Responder {
    let organization_id = path.into_inner();

    match OperationalAgreement::for_organization(db.pool.as_ref(), organization_id).await {
        Ok(agreements) => HttpResponse::Ok().json(agreements),
        Err(err) => {
            eprintln!("list_organization_agreements error: {:?}", err);
            HttpResponse::InternalServerError().body(err.to_string())
        }
    }
}

pub async fn create_organization_agreement(
    db: web::Data<Db>,
    path: web::Path<i64>,
    payload: web::Json<CreateOperationalAgreement>,
) -> impl Responder {
    let organization_id = path.into_inner();

    match OperationalAgreement::create(db.pool.as_ref(), organization_id, payload.into_inner())
        .await
    {
        Ok(agreement) => {
            let _ = EventService::record_event(
                db.pool.as_ref(),
                organization_id,
                None,
                "operational_agreement",
                agreement.id,
                "OperationalAgreementCreated",
                None,
                Some(&agreement.status),
                serde_json::json!({
                    "engagement_id": agreement.engagement_id,
                    "agreement_type": agreement.agreement_type,
                    "title": agreement.title
                }),
            )
            .await;

            if let Some(engagement_id) = agreement.engagement_id {
                let _ = EventService::record_event(
                    db.pool.as_ref(),
                    organization_id,
                    None,
                    "engagement",
                    engagement_id,
                    "OperationalAgreementCreated",
                    None,
                    Some(&agreement.status),
                    serde_json::json!({
                        "agreement_id": agreement.id,
                        "agreement_type": agreement.agreement_type,
                        "title": agreement.title
                    }),
                )
                .await;
            }

            HttpResponse::Created().json(agreement)
        }
        Err(err) => {
            eprintln!("create_organization_agreement error: {:?}", err);
            HttpResponse::InternalServerError().body(err.to_string())
        }
    }
}

pub async fn list_agreement_payout_rules(
    db: web::Data<Db>,
    path: web::Path<i64>,
) -> impl Responder {
    let agreement_id = path.into_inner();

    match AgreementPayoutRule::for_agreement(db.pool.as_ref(), agreement_id).await {
        Ok(rules) => HttpResponse::Ok().json(rules),
        Err(err) => {
            eprintln!("list_agreement_payout_rules error: {:?}", err);
            HttpResponse::InternalServerError().body(err.to_string())
        }
    }
}

pub async fn create_agreement_payout_rule(
    db: web::Data<Db>,
    path: web::Path<i64>,
    payload: web::Json<CreateAgreementPayoutRule>,
) -> impl Responder {
    let agreement_id = path.into_inner();
    let input = payload.into_inner();

    if input.percent.is_none() && input.amount_cents.is_none() {
        return HttpResponse::BadRequest().body("Rule requires percent or amount_cents");
    }

    if let Some(percent) = input.percent {
        if percent <= 0 || percent > 100 {
            return HttpResponse::BadRequest().body("percent must be between 1 and 100");
        }
    }
    let requires_verified_parties = matches!(
        input.rule_type.as_str(),
        "contractor_payout" | "revenue_share" | "dividend"
    );

    if requires_verified_parties {
        let verified_count: i32 = sqlx::query_scalar!(
            r#"
        SELECT COUNT(*) as "count!"
        FROM parties
        WHERE id IN ($1, $2)
          AND is_verified = 1
        "#,
            input.from_party_id,
            input.to_party_id
        )
        .fetch_one(db.pool.as_ref())
        .await
        .unwrap_or(0);

        if verified_count < 2 {
            return HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Core payout rules require verified payer and payee parties."
            }));
        }
    }
    let duplicate_count: i32 = sqlx::query_scalar!(
        r#"
    SELECT COUNT(*) as "count!"
    FROM agreement_payout_rules
    WHERE agreement_id = $1
      AND from_party_id = $2
      AND to_party_id = $3
      AND rule_type = $4
      AND trigger_event = $5
    "#,
        agreement_id,
        input.from_party_id,
        input.to_party_id,
        input.rule_type,
        input.trigger_event
    )
    .fetch_one(db.pool.as_ref())
    .await
    .unwrap_or(0);

    if duplicate_count > 0 {
        return HttpResponse::Conflict().body("Duplicate payout rule already exists");
    }
    match AgreementPayoutRule::create(db.pool.as_ref(), agreement_id, input).await {
        Ok(rule) => {
            let organization_id = sqlx::query_scalar!(
                r#"
                SELECT organization_id as "organization_id!"
                FROM operational_agreements
                WHERE id = $1
                "#,
                agreement_id
            )
            .fetch_one(db.pool.as_ref())
            .await
            .unwrap_or(0);

            if organization_id != 0 {
                let _ = EventService::record_event(
                    db.pool.as_ref(),
                    organization_id,
                    None,
                    "agreement_payout_rule",
                    rule.id,
                    "AgreementPayoutRuleCreated",
                    None,
                    Some("active"),
                    serde_json::json!({
                        "agreement_id": rule.agreement_id,
                        "from_party_id": rule.from_party_id,
                        "to_party_id": rule.to_party_id,
                        "rule_type": rule.rule_type,
                        "percent": rule.percent,
                        "amount_cents": rule.amount_cents,
                        "trigger_event": rule.trigger_event
                    }),
                )
                .await;
            }

            HttpResponse::Created().json(rule)
        }
        Err(err) => {
            eprintln!("create_agreement_payout_rule error: {:?}", err);
            HttpResponse::InternalServerError().body(err.to_string())
        }
    }
}
