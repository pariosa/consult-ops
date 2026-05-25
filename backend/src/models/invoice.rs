use crate::db::Db;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool, Result as SqlxResult};

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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateInvoice {
    pub organization_id: i64,
    pub contract_id: i64,

    pub invoice_number: String,

    pub status: String,

    pub issued_at: Option<String>,
    pub due_date: Option<String>,

    pub subtotal: Option<f64>,
    pub tax: Option<f64>,
    pub total: Option<f64>,

    pub currency: Option<String>,

    pub notes: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateInvoice {
    pub invoice_number: String,

    pub status: String,

    pub issued_at: Option<String>,
    pub due_date: Option<String>,

    pub subtotal: Option<f64>,
    pub tax: Option<f64>,
    pub total: Option<f64>,

    pub currency: Option<String>,

    pub notes: Option<String>,
}

impl Invoice {
    pub async fn all(db: &Db) -> SqlxResult<Vec<Self>> {
        sqlx::query_as::<_, Invoice>(
            r#"
            SELECT *
            FROM invoices
            ORDER BY created_at DESC
            "#,
        )
        .fetch_all(&*db.pool)
        .await
    }

    pub async fn for_organization(db: &PgPool, organization_id: i64) -> SqlxResult<Vec<Self>> {
        sqlx::query_as::<_, Invoice>(
            r#"
            SELECT *
            FROM invoices
            WHERE organization_id = $1
            ORDER BY created_at DESC
            "#,
        )
        .bind(organization_id)
        .fetch_all(db)
        .await
    }

    pub async fn find(db: &PgPool, id: i64) -> SqlxResult<Self> {
        sqlx::query_as::<_, Invoice>(
            r#"
            SELECT *
            FROM invoices
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_one(db)
        .await
    }

    pub async fn find_for_user(db: &PgPool, id: i64, user_id: i64) -> SqlxResult<Self> {
        sqlx::query_as::<_, Invoice>(
            r#"
            SELECT i.*
            FROM invoices i
            JOIN organization_members om
              ON om.organization_id = i.organization_id
            WHERE i.id = $1
              AND om.user_id = $2
              AND om.status = 'active'
            "#,
        )
        .bind(id)
        .bind(user_id)
        .fetch_one(db)
        .await
    }

    pub async fn create(db: &PgPool, invoice: CreateInvoice) -> SqlxResult<Self> {
        let currency = invoice.currency.unwrap_or_else(|| "usd".to_string());

        sqlx::query_as::<_, Invoice>(
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
            VALUES (
                $1, $2, $3, $4, $5, $6,
                $7, $8, $9, $10, $11,
                CURRENT_TIMESTAMP
            )
            RETURNING *
            "#,
        )
        .bind(invoice.organization_id)
        .bind(invoice.contract_id)
        .bind(invoice.invoice_number)
        .bind(invoice.status)
        .bind(invoice.issued_at)
        .bind(invoice.due_date)
        .bind(invoice.subtotal)
        .bind(invoice.tax)
        .bind(invoice.total)
        .bind(currency)
        .bind(invoice.notes)
        .fetch_one(db)
        .await
    }

    pub async fn update(db: &PgPool, id: i64, invoice: UpdateInvoice) -> SqlxResult<Self> {
        let currency = invoice.currency.unwrap_or_else(|| "usd".to_string());

        sqlx::query_as::<_, Invoice>(
            r#"
            UPDATE invoices
            SET
                invoice_number = $1,
                status = $2,
                issued_at = $3,
                due_date = $4,
                subtotal = $5,
                tax = $6,
                total = $7,
                currency = $8,
                notes = $9
            WHERE id = $10
            RETURNING *
            "#,
        )
        .bind(invoice.invoice_number)
        .bind(invoice.status)
        .bind(invoice.issued_at)
        .bind(invoice.due_date)
        .bind(invoice.subtotal)
        .bind(invoice.tax)
        .bind(invoice.total)
        .bind(currency)
        .bind(invoice.notes)
        .bind(id)
        .fetch_one(db)
        .await
    }

    pub async fn update_status(db: &PgPool, id: i64, status: &str) -> SqlxResult<Self> {
        sqlx::query_as::<_, Invoice>(
            r#"
            UPDATE invoices
            SET status = $1
            WHERE id = $2
            RETURNING *
            "#,
        )
        .bind(status)
        .bind(id)
        .fetch_one(db)
        .await
    }

    pub async fn delete(db: &PgPool, id: i64) -> SqlxResult<()> {
        sqlx::query(
            r#"
            DELETE FROM invoices
            WHERE id = $1
            "#,
        )
        .bind(id)
        .execute(db)
        .await?;

        Ok(())
    }
}
