use crate::db::Db;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Result as SqlxResult};

#[derive(Deserialize)]
pub struct CreatePayment {
    pub invoice_id: i64,
    pub amount: f64,
    pub paid_at: Option<String>,
    pub currency: Option<String>,
    pub method: Option<String>,
    pub reference: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Payment {
    pub id: i64,
    pub invoice_id: i64,

    pub paid_at: Option<String>, // datetime
    pub amount: f64,
    pub currency: Option<String>,

    pub method: Option<String>,    // e.g., cash, card, bank_transfer
    pub reference: Option<String>, // check number, txn id

    pub notes: Option<String>,

    pub created_at: String,
}

impl Payment {
    pub fn new(create: CreatePayment) -> Self {
        let now = chrono::Utc::now().to_rfc3339();
        Payment {
            id: 0,
            invoice_id: create.invoice_id,
            amount: create.amount,
            paid_at: create.paid_at.or(Some(now.clone())),
            currency: create.currency,
            method: create.method,
            reference: create.reference,
            notes: create.notes,
            created_at: now,
        }
    }
    pub async fn all(db: &Db) -> SqlxResult<Vec<Self>> {
        sqlx::query_as::<_, Payment>("SELECT * FROM payments ORDER BY created_at DESC")
            .fetch_all(&*db.pool)
            .await
    }

    pub async fn create(db: &Db, payment: Payment) -> SqlxResult<Self> {
        let rec = sqlx::query(
            r#"
            INSERT INTO payments (
                invoice_id,
                paid_at,
                amount,
                currency,
                method,
                reference,
                notes,
                created_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(payment.invoice_id)
        .bind(&payment.paid_at)
        .bind(payment.amount)
        .bind(&payment.currency)
        .bind(&payment.method)
        .bind(&payment.reference)
        .bind(&payment.notes)
        .bind(&payment.created_at)
        .execute(&*db.pool)
        .await?;

        Ok(Payment {
            id: rec.last_insert_rowid(),
            ..payment
        })
    }
}
