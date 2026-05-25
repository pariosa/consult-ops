// backend/src/handlers/subscription.rs

use actix_web::{HttpResponse, Responder, ResponseError, web};

use crate::auth_context::AuthUser;
use crate::db::Db;
use crate::models::subscription::{OrganizationSubscription, UpsertOrganizationSubscription};
use crate::services::authz::{require_org_member, require_org_role};
use crate::services::event_service::EventService;

pub async fn get_organization_subscription(
    db: web::Data<Db>,
    auth: AuthUser,
    path: web::Path<i64>,
) -> impl Responder {
    let organization_id = path.into_inner();

    if let Err(err) = require_org_member(db.pool.as_ref(), auth.id, organization_id).await {
        return err.error_response();
    }

    match OrganizationSubscription::find_by_organization(db.pool.as_ref(), organization_id).await {
        Ok(Some(subscription)) => HttpResponse::Ok().json(subscription),
        Ok(None) => HttpResponse::Ok().json(serde_json::json!({
            "organization_id": organization_id,
            "subscription_status": "inactive",
            "subscription_plan": "free",
            "is_active": false
        })),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn upsert_organization_subscription(
    db: web::Data<Db>,
    auth: AuthUser,
    path: web::Path<i64>,
    payload: web::Json<UpsertOrganizationSubscription>,
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

    match OrganizationSubscription::upsert(db.pool.as_ref(), organization_id, payload.into_inner())
        .await
    {
        Ok(subscription) => {
            let _ = EventService::record_event(
                db.pool.as_ref(),
                organization_id,
                Some(auth.id),
                "organization_subscription",
                subscription.id,
                "OrganizationSubscriptionUpdated",
                None,
                Some(&subscription.subscription_status),
                serde_json::json!({
                    "subscription_plan": subscription.subscription_plan,
                    "subscription_status": subscription.subscription_status,
                    "stripe_customer_id": subscription.stripe_customer_id,
                    "stripe_subscription_id": subscription.stripe_subscription_id,
                    "cancel_at_period_end": subscription.cancel_at_period_end
                }),
            )
            .await;

            HttpResponse::Ok().json(subscription)
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn mark_subscription_active_dev(
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

    match OrganizationSubscription::mark_active(
        db.pool.as_ref(),
        organization_id,
        "pro",
        Some(format!("cus_dev_org_{}", organization_id)),
        Some(format!("sub_dev_org_{}", organization_id)),
    )
    .await
    {
        Ok(subscription) => {
            let _ = EventService::record_event(
                db.pool.as_ref(),
                organization_id,
                Some(auth.id),
                "organization_subscription",
                subscription.id,
                "OrganizationSubscriptionActivatedDev",
                None,
                Some("active"),
                serde_json::json!({
                    "subscription_plan": subscription.subscription_plan,
                    "subscription_status": subscription.subscription_status
                }),
            )
            .await;

            HttpResponse::Ok().json(subscription)
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn cancel_subscription_dev(
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

    match OrganizationSubscription::mark_cancelled(db.pool.as_ref(), organization_id).await {
        Ok(subscription) => {
            let _ = EventService::record_event(
                db.pool.as_ref(),
                organization_id,
                Some(auth.id),
                "organization_subscription",
                subscription.id,
                "OrganizationSubscriptionCancelledDev",
                Some("active"),
                Some("cancelled"),
                serde_json::json!({
                    "subscription_plan": subscription.subscription_plan,
                    "subscription_status": subscription.subscription_status
                }),
            )
            .await;

            HttpResponse::Ok().json(subscription)
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
