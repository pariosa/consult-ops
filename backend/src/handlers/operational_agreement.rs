use actix_web::{HttpResponse, Responder, ResponseError, web};

use crate::auth_context::AuthUser;
use crate::db::Db;
use crate::models::agreement_payout_rule::{AgreementPayoutRule, CreateAgreementPayoutRule};
use crate::models::operational_agreement::{CreateOperationalAgreement, OperationalAgreement};
use crate::services::authz::{require_org_member, require_org_role};
use crate::services::event_service::EventService;

async fn organization_id_for_agreement(db: &Db, agreement_id: i64) -> Result<i64, sqlx::Error> {
    sqlx::query_scalar::<_, i64>(
        r#"
        SELECT organization_id
        FROM operational_agreements
        WHERE id = $1
        "#,
    )
    .bind(agreement_id)
    .fetch_one(db.pool.as_ref())
    .await
}

pub async fn list_organization_agreements(
    db: web::Data<Db>,
    auth: AuthUser,
    path: web::Path<i64>,
) -> impl Responder {
    let organization_id = path.into_inner();

    if let Err(err) = require_org_member(db.pool.as_ref(), auth.id, organization_id).await {
        return err.error_response();
    }

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
    auth: AuthUser,
    path: web::Path<i64>,
    payload: web::Json<CreateOperationalAgreement>,
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

    match OperationalAgreement::create(db.pool.as_ref(), organization_id, payload.into_inner())
        .await
    {
        Ok(agreement) => {
            let _ = EventService::record_event(
                db.pool.as_ref(),
                organization_id,
                Some(auth.id),
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
                    Some(auth.id),
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
    auth: AuthUser,
    path: web::Path<i64>,
) -> impl Responder {
    let agreement_id = path.into_inner();

    let organization_id = match organization_id_for_agreement(&db, agreement_id).await {
        Ok(id) => id,
        Err(_) => return HttpResponse::NotFound().body("Agreement not found"),
    };

    if let Err(err) = require_org_member(db.pool.as_ref(), auth.id, organization_id).await {
        return err.error_response();
    }

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
    auth: AuthUser,
    path: web::Path<i64>,
    payload: web::Json<CreateAgreementPayoutRule>,
) -> impl Responder {
    let agreement_id = path.into_inner();
    let input = payload.into_inner();

    let organization_id = match organization_id_for_agreement(&db, agreement_id).await {
        Ok(id) => id,
        Err(_) => return HttpResponse::NotFound().body("Agreement not found"),
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

    if input.percent.is_none() && input.amount_cents.is_none() {
        return HttpResponse::BadRequest().body("Rule requires percent or amount_cents");
    }

    if let Some(percent) = input.percent {
        if percent <= 0 || percent > 100 {
            return HttpResponse::BadRequest().body("percent must be between 1 and 100");
        }
    }

    let party_count = sqlx::query_scalar::<_, i64>(
        r#"
        SELECT COUNT(*)::BIGINT
        FROM parties
        WHERE organization_id = $1
          AND id IN ($2, $3)
        "#,
    )
    .bind(organization_id)
    .bind(input.from_party_id)
    .bind(input.to_party_id)
    .fetch_one(db.pool.as_ref())
    .await
    .unwrap_or(0);

    if party_count < 2 {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Payer and payee parties must both belong to this organization."
        }));
    }

    let requires_verified_parties = matches!(
        input.rule_type.as_str(),
        "contractor_payout" | "revenue_share" | "dividend"
    );

    if requires_verified_parties {
        let verified_count = sqlx::query_scalar::<_, i64>(
            r#"
            SELECT COUNT(*)::BIGINT
            FROM parties
            WHERE organization_id = $1
              AND id IN ($2, $3)
              AND is_verified = 1
            "#,
        )
        .bind(organization_id)
        .bind(input.from_party_id)
        .bind(input.to_party_id)
        .fetch_one(db.pool.as_ref())
        .await
        .unwrap_or(0);

        if verified_count < 2 {
            return HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Core payout rules require verified payer and payee parties."
            }));
        }
    }

    let duplicate_count = sqlx::query_scalar::<_, i64>(
        r#"
        SELECT COUNT(*)::BIGINT
        FROM agreement_payout_rules
        WHERE agreement_id = $1
          AND from_party_id = $2
          AND to_party_id = $3
          AND rule_type = $4
          AND trigger_event = $5
        "#,
    )
    .bind(agreement_id)
    .bind(input.from_party_id)
    .bind(input.to_party_id)
    .bind(&input.rule_type)
    .bind(&input.trigger_event)
    .fetch_one(db.pool.as_ref())
    .await
    .unwrap_or(0);

    if duplicate_count > 0 {
        return HttpResponse::Conflict().body("Duplicate payout rule already exists");
    }

    match AgreementPayoutRule::create(db.pool.as_ref(), agreement_id, input).await {
        Ok(rule) => {
            let _ = EventService::record_event(
                db.pool.as_ref(),
                organization_id,
                Some(auth.id),
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

            HttpResponse::Created().json(rule)
        }
        Err(err) => {
            eprintln!("create_agreement_payout_rule error: {:?}", err);
            HttpResponse::InternalServerError().body(err.to_string())
        }
    }
}

pub async fn lock_agreement(
    db: web::Data<Db>,
    auth: AuthUser,
    path: web::Path<i64>,
) -> impl Responder {
    let agreement_id = path.into_inner();

    let organization_id = match organization_id_for_agreement(&db, agreement_id).await {
        Ok(id) => id,
        Err(_) => return HttpResponse::NotFound().body("Agreement not found"),
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

    let rule_count = sqlx::query_scalar::<_, i64>(
        r#"
        SELECT COUNT(*)::BIGINT
        FROM agreement_payout_rules
        WHERE agreement_id = $1
        "#,
    )
    .bind(agreement_id)
    .fetch_one(db.pool.as_ref())
    .await;

    let rule_count = match rule_count {
        Ok(count) => count,
        Err(err) => return HttpResponse::InternalServerError().body(err.to_string()),
    };

    if rule_count == 0 {
        return HttpResponse::Conflict()
            .body("Agreement must have at least one payout rule before locking.");
    }

    match OperationalAgreement::lock(db.pool.as_ref(), agreement_id).await {
        Ok(agreement) => {
            let _ = EventService::record_event(
                db.pool.as_ref(),
                organization_id,
                Some(auth.id),
                "operational_agreement",
                agreement.id,
                "OperationalAgreementLocked",
                None,
                Some(&agreement.status),
                serde_json::json!({
                    "agreement_id": agreement.id,
                    "engagement_id": agreement.engagement_id,
                    "agreement_type": agreement.agreement_type
                }),
            )
            .await;

            HttpResponse::Ok().json(agreement)
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
