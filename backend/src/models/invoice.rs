use crate::db::Db;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Result as SqlxResult};

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Invoice {
    pub id: i64,
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

    pub async fn create(db: &Db, invoice: Invoice) -> SqlxResult<Self> {
        let rec = sqlx::query(
            r#"
            INSERT INTO invoices (
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
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
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
            ..invoice
        })
    }
}
