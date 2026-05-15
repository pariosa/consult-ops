use actix_web::{HttpResponse, Responder, web};

use crate::db::Db;
use crate::models::operational_transaction::OperationalTransaction;

pub async fn list_engagement_transactions(
    db: web::Data<Db>,
    path: web::Path<i64>,
) -> impl Responder {
    let engagement_id = path.into_inner();

    match OperationalTransaction::for_engagement(db.pool.as_ref(), engagement_id).await {
        Ok(transactions) => HttpResponse::Ok().json(transactions),
        Err(err) => {
            eprintln!("list_engagement_transactions error: {:?}", err);
            HttpResponse::InternalServerError().body(err.to_string())
        }
    }
}

pub async fn list_organization_transactions(
    db: web::Data<Db>,
    path: web::Path<i64>,
) -> impl Responder {
    let organization_id = path.into_inner();

    let result = sqlx::query_as::<_, OperationalTransaction>(
        r#"
        SELECT *
        FROM operational_transactions
        WHERE organization_id = ?
        ORDER BY created_at DESC
        "#,
    )
    .bind(organization_id)
    .fetch_all(db.pool.as_ref())
    .await;

    match result {
        Ok(transactions) => HttpResponse::Ok().json(transactions),
        Err(err) => {
            eprintln!("list_organization_transactions error: {:?}", err);
            HttpResponse::InternalServerError().body(err.to_string())
        }
    }
}
