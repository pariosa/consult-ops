use crate::db::Db;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Result as SqlxResult};

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Client {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub tax_id: Option<String>,
    pub phone: Option<String>,
    pub company_name: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub zip: Option<String>,
    pub country: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

impl Client {
    pub async fn all(db: &Db) -> SqlxResult<Vec<Self>> {
        sqlx::query_as::<_, Client>("SELECT * FROM clients")
            .fetch_all(&*db.pool)
            .await
    }

    pub async fn create(db: &Db, client: Client) -> SqlxResult<Self> {
        let rec = sqlx::query(
            r#"
            INSERT INTO clients 
            (name, email, phone, company_name, address, city, state, zip, country)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&client.name)
        .bind(&client.email)
        .bind(&client.phone)
        .bind(&client.company_name)
        .bind(&client.address)
        .bind(&client.city)
        .bind(&client.state)
        .bind(&client.zip)
        .bind(&client.country)
        .execute(&*db.pool)
        .await?;

        Ok(Client {
            id: rec.last_insert_rowid(),
            ..client
        })
    }
}
