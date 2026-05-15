use actix_web::{HttpResponse, Responder, web};

use crate::auth::permissions::can_manage_finance;
use crate::auth_context::AuthUser;
use crate::db::Db;
use crate::services::operational_finance_service::OperationalFinanceService;

pub async fn get_organization_finance_summary(
    db: web::Data<Db>,
    auth: AuthUser,
    path: web::Path<i64>,
) -> impl Responder {
    if !can_manage_finance(&auth.user_type) {
        return HttpResponse::Forbidden().json(serde_json::json!({
            "error": "You do not have permission to view finance summaries."
        }));
    }

    let organization_id = path.into_inner();

    match OperationalFinanceService::organization_summary(db.pool.as_ref(), organization_id).await {
        Ok(summary) => HttpResponse::Ok().json(summary),
        Err(err) => {
            eprintln!("get_organization_finance_summary error: {:?}", err);
            HttpResponse::InternalServerError().body(err)
        }
    }
}

pub async fn get_organization_party_balances(
    db: web::Data<Db>,
    auth: AuthUser,
    path: web::Path<i64>,
) -> impl Responder {
    if !can_manage_finance(&auth.user_type) {
        return HttpResponse::Forbidden().json(serde_json::json!({
            "error": "You do not have permission to view party balances."
        }));
    }

    let organization_id = path.into_inner();

    match OperationalFinanceService::party_balances(db.pool.as_ref(), organization_id).await {
        Ok(balances) => HttpResponse::Ok().json(balances),
        Err(err) => {
            eprintln!("get_organization_party_balances error: {:?}", err);
            HttpResponse::InternalServerError().body(err)
        }
    }
}
