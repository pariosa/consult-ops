use crate::db::Db;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Result as SqlxResult};

#[derive(Debug, Deserialize)]
pub struct CreatePayment {
    pub organization_id: i64,
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
    pub organization_id: i64,
    pub invoice_id: i64,
    pub paid_at: Option<String>,
    pub amount: f64,
    pub currency: Option<String>,
    pub method: Option<String>,
    pub reference: Option<String>,
    pub notes: Option<String>,
    pub created_at: String,
}

impl Payment {
    pub fn new(create: CreatePayment) -> Self {
        let now = chrono::Utc::now().to_rfc3339();

        Payment {
            id: 0,
            organization_id: create.organization_id,
            invoice_id: create.invoice_id,
            amount: create.amount,
            paid_at: create.paid_at.or(Some(now.clone())),
            currency: create.currency.or(Some("usd".to_string())),
            method: create.method,
            reference: create.reference,
            notes: create.notes,
            created_at: now,
        }
    }

    pub async fn all(db: &Db) -> SqlxResult<Vec<Self>> {
        sqlx::query_as::<_, Payment>(
            r#"
            SELECT *
            FROM payments
            ORDER BY created_at DESC
            "#,
        )
        .fetch_all(&*db.pool)
        .await
    }

    pub async fn find(db: &Db, id: i64) -> SqlxResult<Option<Self>> {
        sqlx::query_as::<_, Payment>(
            r#"
            SELECT *
            FROM payments
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&*db.pool)
        .await
    }

    pub async fn find_for_user(db: &Db, payment_id: i64, user_id: i64) -> SqlxResult<Option<Self>> {
        sqlx::query_as::<_, Payment>(
            r#"
            SELECT p.*
            FROM payments p
            JOIN organization_members om
              ON om.organization_id = p.organization_id
            WHERE p.id = $1
              AND om.user_id = $2
              AND om.status = 'active'
            "#,
        )
        .bind(payment_id)
        .bind(user_id)
        .fetch_optional(&*db.pool)
        .await
    }

    pub async fn for_organization(db: &Db, organization_id: i64) -> SqlxResult<Vec<Self>> {
        sqlx::query_as::<_, Payment>(
            r#"
            SELECT *
            FROM payments
            WHERE organization_id = $1
            ORDER BY created_at DESC
            "#,
        )
        .bind(organization_id)
        .fetch_all(&*db.pool)
        .await
    }

    pub async fn create(db: &Db, payment: Payment) -> SqlxResult<Self> {
        sqlx::query_as::<_, Payment>(
            r#"
            INSERT INTO payments (
                organization_id,
                invoice_id,
                paid_at,
                amount,
                currency,
                method,
                reference,
                notes,
                created_at
            )
            VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9
            )
            RETURNING *
            "#,
        )
        .bind(payment.organization_id)
        .bind(payment.invoice_id)
        .bind(&payment.paid_at)
        .bind(payment.amount)
        .bind(&payment.currency)
        .bind(&payment.method)
        .bind(&payment.reference)
        .bind(&payment.notes)
        .bind(&payment.created_at)
        .fetch_one(&*db.pool)
        .await
    }

    pub async fn for_invoice(db: &Db, invoice_id: i64) -> SqlxResult<Vec<Self>> {
        sqlx::query_as::<_, Payment>(
            r#"
            SELECT *
            FROM payments
            WHERE invoice_id = $1
            ORDER BY created_at DESC
            "#,
        )
        .bind(invoice_id)
        .fetch_all(&*db.pool)
        .await
    }
}
