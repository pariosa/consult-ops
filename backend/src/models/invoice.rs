use crate::db::Db;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Result as SqlxResult};

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Invoice {
    pub id: i64,
    pub organization_id: i64,
    pub contract_id: i64,

    pub invoice_number: String,

    pub status: String, // draft | sent | paid | overdue | cancelled

    pub issued_at: Option<String>,
    pub due_date: Option<String>,

    pub subtotal: Option<f64>,
    pub tax: Option<f64>,
    pub total: Option<f64>,

    pub currency: Option<String>,

    pub notes: Option<String>,

    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct CreateInvoice {
    pub organization_id: i64,
    pub contract_id: i64,

    pub invoice_number: String,

    pub status: String, // draft | sent | paid | overdue | cancelled

    pub issued_at: Option<String>,
    pub due_date: Option<String>,

    pub subtotal: Option<f64>,
    pub tax: Option<f64>,
    pub total: Option<f64>,

    pub currency: Option<String>,

    pub notes: Option<String>,

    pub created_at: String,
}
impl Invoice {
    pub async fn all(db: &Db) -> SqlxResult<Vec<Self>> {
        sqlx::query_as::<_, Invoice>("SELECT * FROM invoices ORDER BY created_at DESC")
            .fetch_all(&*db.pool)
            .await
    }

    pub async fn create(db: &Db, invoice: CreateInvoice) -> SqlxResult<Self> {
        let rec = sqlx::query(
            r#"
            INSERT INTO invoices (
                organization_id,
                contract_id,
                invoice_number,
                status,
                issued_at,
                due_date,
                subtotal,
                tax,
                total,
                currency,
                notes,
                created_at
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(invoice.organization_id)
        .bind(invoice.contract_id)
        .bind(&invoice.invoice_number)
        .bind(&invoice.status)
        .bind(&invoice.issued_at)
        .bind(&invoice.due_date)
        .bind(invoice.subtotal)
        .bind(invoice.tax)
        .bind(invoice.total)
        .bind(&invoice.currency)
        .bind(&invoice.notes)
        .bind(&invoice.created_at)
        .execute(&*db.pool)
        .await?;

        Ok(Invoice {
            id: rec.last_insert_rowid(),
            organization_id: invoice.organization_id,
            contract_id: invoice.contract_id,
            invoice_number: invoice.invoice_number,
            status: invoice.status,
            issued_at: invoice.issued_at,
            due_date: invoice.due_date,
            subtotal: invoice.subtotal,
            tax: invoice.tax,
            total: invoice.total,
            currency: invoice.currency,
            notes: invoice.notes,
            created_at: invoice.created_at,
        })
    }
}
