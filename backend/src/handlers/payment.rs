use crate::auth_context::AuthUser;
use crate::db::Db;
use crate::models::payment::{CreatePayment, Payment};
use crate::services::authz::{require_org_member, require_org_role};
use actix_web::{HttpResponse, Responder, ResponseError, web};

async fn organization_id_for_invoice(db: &Db, invoice_id: i64) -> Result<i64, sqlx::Error> {
    sqlx::query_scalar::<_, i64>(
        r#"
        SELECT organization_id
        FROM invoices
        WHERE id = $1
        "#,
    )
    .bind(invoice_id)
    .fetch_one(db.pool.as_ref())
    .await
}

/// Global payment listing should be platform-admin only.
/// Normal app usage should prefer `/api/organizations/{id}/payments`.
pub async fn get_payments(db: web::Data<Db>, auth: AuthUser) -> impl Responder {
    if auth.user_type != "admin" && auth.user_type != "super_admin" {
        return HttpResponse::Forbidden().body("Platform admin access required");
    }

    match Payment::all(&db).await {
        Ok(payments) => HttpResponse::Ok().json(payments),
        Err(e) => {
            eprintln!("DB error fetching payments: {}", e);
            HttpResponse::InternalServerError().body("Failed to fetch payments")
        }
    }
}

pub async fn create_payment(
    db: web::Data<Db>,
    auth: AuthUser,
    info: web::Json<CreatePayment>,
) -> impl Responder {
    let mut input = info.into_inner();

    let organization_id = match organization_id_for_invoice(&db, input.invoice_id).await {
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().body("Invalid invoice_id"),
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

    // Do not trust client-supplied organization_id.
    input.organization_id = organization_id;

    let payment = Payment::new(input);

    match Payment::create(&db, payment).await {
        Ok(saved) => HttpResponse::Created().json(saved),
        Err(e) => {
            eprintln!("DB error creating payment: {}", e);
            HttpResponse::InternalServerError().body("Failed to create payment")
        }
    }
}
