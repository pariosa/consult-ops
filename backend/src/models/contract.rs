use crate::db::Db;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Result as SqlxResult};

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Contract {
    pub id: i64,
    pub organization_id: i64,
    pub project_id: i64,

    pub title: String,

    pub status: String, // draft | active | completed | cancelled

    pub signed_at: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,

    pub value: Option<f64>,
    pub currency: Option<String>,

    pub terms: Option<String>,
    pub notes: Option<String>,

    pub external_id: Option<String>,

    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct CreateContract {
    pub organization_id: i64,
    pub project_id: i64,

    pub title: String,

    pub status: String, // draft | active | completed | cancelled

    pub signed_at: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,

    pub value: Option<f64>,
    pub currency: Option<String>,

    pub terms: Option<String>,
    pub notes: Option<String>,

    pub external_id: Option<String>,

    pub created_at: String,
}

impl Contract {
    pub async fn all(db: &Db) -> SqlxResult<Vec<Self>> {
        sqlx::query_as::<_, Contract>(
            r#"
            SELECT *
            FROM contracts
            ORDER BY created_at DESC
            "#,
        )
        .fetch_all(&*db.pool)
        .await
    }

    pub async fn create(db: &Db, contract: CreateContract) -> SqlxResult<Self> {
        sqlx::query_as::<_, Contract>(
            r#"
            INSERT INTO contracts (
                organization_id,
                project_id,
                title,
                status,
                signed_at,
                start_date,
                end_date,
                value,
                currency,
                terms,
                notes,
                external_id,
                created_at
            )
            VALUES (
                $1, $2, $3, $4, $5, $6,
                $7, $8, $9, $10, $11,
                $12, $13
            )
            RETURNING *
            "#,
        )
        .bind(contract.organization_id)
        .bind(contract.project_id)
        .bind(&contract.title)
        .bind(&contract.status)
        .bind(&contract.signed_at)
        .bind(&contract.start_date)
        .bind(&contract.end_date)
        .bind(contract.value)
        .bind(&contract.currency)
        .bind(&contract.terms)
        .bind(&contract.notes)
        .bind(&contract.external_id)
        .bind(&contract.created_at)
        .fetch_one(&*db.pool)
        .await
    }
    pub async fn for_organization(db: &Db, organization_id: i64) -> SqlxResult<Vec<Self>> {
        sqlx::query_as::<_, Contract>(
            r#"
        SELECT *
        FROM contracts
        WHERE organization_id = $1
        ORDER BY created_at DESC
        "#,
        )
        .bind(organization_id)
        .fetch_all(db.pool.as_ref())
        .await
    }
}
