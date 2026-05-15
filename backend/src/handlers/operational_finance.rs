use actix_web::{HttpResponse, Responder, web};

use crate::db::Db;
use crate::services::operational_finance_service::OperationalFinanceService;

pub async fn get_organization_finance_summary(
    db: web::Data<Db>,
    path: web::Path<i64>,
) -> impl Responder {
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
    path: web::Path<i64>,
) -> impl Responder {
    let organization_id = path.into_inner();

    match OperationalFinanceService::party_balances(db.pool.as_ref(), organization_id).await {
        Ok(balances) => HttpResponse::Ok().json(balances),
        Err(err) => {
            eprintln!("get_organization_party_balances error: {:?}", err);
            HttpResponse::InternalServerError().body(err)
        }
    }
}
