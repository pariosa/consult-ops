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
        sqlx::query_as::<_, Contract>("SELECT * FROM contracts ORDER BY created_at DESC")
            .fetch_all(&*db.pool)
            .await
    }

    pub async fn create(db: &Db, contract: CreateContract) -> SqlxResult<Self> {
        let rec = sqlx::query(
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
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?) 
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
        .execute(&*db.pool)
        .await?;

        Ok(Contract {
            id: rec.last_insert_rowid(),
            organization_id: contract.organization_id,
            project_id: contract.project_id,
            title: contract.title,
            status: contract.status,
            signed_at: contract.signed_at,
            start_date: contract.start_date,
            end_date: contract.end_date,
            value: contract.value,
            currency: contract.currency,
            terms: contract.terms,
            notes: contract.notes,
            external_id: contract.external_id,
            created_at: contract.created_at,
        })
    }
}
