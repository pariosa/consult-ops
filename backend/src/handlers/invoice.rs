use crate::auth_context::AuthUser;
use crate::db::Db;
use crate::models::invoice::{CreateInvoice, Invoice};
use crate::services::authz::{require_org_member, require_org_role};
use crate::services::event_service::EventService;
use actix_web::{HttpResponse, Responder, ResponseError, web};

async fn organization_id_for_contract(db: &Db, contract_id: i64) -> Result<i64, sqlx::Error> {
    sqlx::query_scalar::<_, i64>(
        r#"
        SELECT organization_id
        FROM contracts
        WHERE id = $1
        "#,
    )
    .bind(contract_id)
    .fetch_one(db.pool.as_ref())
    .await
}

/// Get invoices for current org/member context.
/// Prefer this over global invoice listing.
pub async fn get_organization_invoices(
    db: web::Data<Db>,
    auth: AuthUser,
    path: web::Path<i64>,
) -> impl Responder {
    let organization_id = path.into_inner();

    if let Err(err) = require_org_member(db.pool.as_ref(), auth.id, organization_id).await {
        return err.error_response();
    }

    match Invoice::for_organization(db.pool.as_ref(), organization_id).await {
        Ok(invoices) => HttpResponse::Ok().json(invoices),
        Err(e) => {
            eprintln!("DB error fetching organization invoices: {}", e);
            HttpResponse::InternalServerError().body("Failed to fetch invoices")
        }
    }
}

/// Global invoice listing should not be public.
/// Keep only if you intend this as platform-admin route later.
pub async fn get_invoices(db: web::Data<Db>, auth: AuthUser) -> impl Responder {
    // Safer temporary behavior:
    // refuse global listing unless you wire platform admin authz here.
    if auth.user_type != "admin" && auth.user_type != "super_admin" {
        return HttpResponse::Forbidden().body("Platform admin access required");
    }

    match Invoice::all(&db).await {
        Ok(invoices) => HttpResponse::Ok().json(invoices),
        Err(e) => {
            eprintln!("DB error fetching invoices: {}", e);
            HttpResponse::InternalServerError().body("Failed to fetch invoices")
        }
    }
}

/// Create a new invoice.
/// The contract decides the organization_id; do not blindly trust body.organization_id.
pub async fn create_invoice(
    db: web::Data<Db>,
    auth: AuthUser,
    info: web::Json<CreateInvoice>,
) -> impl Responder {
    let mut invoice = info.into_inner();

    let organization_id = match organization_id_for_contract(&db, invoice.contract_id).await {
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().body("Invalid contract_id"),
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
    invoice.organization_id = organization_id;

    if invoice.status.trim().is_empty() {
        invoice.status = "draft".to_string();
    }

    if invoice.subtotal.is_none() {
        invoice.subtotal = Some(0.0);
    }

    if invoice.tax.is_none() {
        invoice.tax = Some(0.0);
    }

    if invoice.total.is_none() {
        invoice.total = Some(invoice.subtotal.unwrap_or(0.0) + invoice.tax.unwrap_or(0.0));
    }

    match Invoice::create(db.pool.as_ref(), invoice).await {
        Ok(invoice) => {
            let _ = EventService::record_event(
                db.pool.as_ref(),
                invoice.organization_id,
                Some(auth.id),
                "invoice",
                invoice.id,
                "InvoiceCreated",
                None,
                Some(&invoice.status),
                serde_json::json!({
                    "contract_id": invoice.contract_id,
                    "invoice_number": invoice.invoice_number,
                    "total": invoice.total,
                    "currency": invoice.currency
                }),
            )
            .await;

            HttpResponse::Created().json(invoice)
        }
        Err(e) => {
            eprintln!("DB error creating invoice: {}", e);
            HttpResponse::InternalServerError().body("Failed to create invoice")
        }
    }
}
