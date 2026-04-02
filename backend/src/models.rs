use crate::db::Db;
use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Result as SqlxResult};

// ====================== Clients ======================
#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Client {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub phone: Option<String>,
}

impl Client {
    pub async fn all(db: &Db) -> SqlxResult<Vec<Self>> {
        sqlx::query_as::<_, Client>("SELECT * FROM clients")
            .fetch_all(&*db.pool)
            .await
    }

    pub async fn create(db: &Db, client: Client) -> SqlxResult<Self> {
        let rec = sqlx::query(
            "INSERT INTO clients (name, email, phone) VALUES (?, ?, ?)"
        )
        .bind(&client.name)
        .bind(&client.email)
        .bind(&client.phone)
        .execute(&*db.pool)
        .await?;

        Ok(Client { id: rec.last_insert_rowid(), ..client })
    }
}

pub async fn get_clients(db: web::Data<Db>) -> impl Responder {
    match Client::all(&db).await {
        Ok(clients) => HttpResponse::Ok().json(clients),
        Err(e) => {
            eprintln!("DB error: {}", e);
            HttpResponse::InternalServerError().body("Failed to fetch clients")
        }
    }
}

pub async fn create_client(db: web::Data<Db>, info: web::Json<Client>) -> impl Responder {
    match Client::create(&db, info.into_inner()).await {
        Ok(client) => HttpResponse::Ok().json(client),
        Err(e) => {
            eprintln!("DB error: {}", e);
            HttpResponse::InternalServerError().body("Failed to create client")
        }
    }
}

// ====================== Projects ======================
#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Project {
    pub id: i64,
    pub client_id: i64,
    pub name: String,
    pub start_date: String,
}

impl Project {
    pub async fn all(db: &Db) -> SqlxResult<Vec<Self>> {
        sqlx::query_as::<_, Project>("SELECT * FROM projects")
            .fetch_all(&*db.pool)
            .await
    }

    pub async fn create(db: &Db, project: Project) -> SqlxResult<Self> {
        let rec = sqlx::query(
            "INSERT INTO projects (client_id, name, start_date) VALUES (?, ?, ?)"
        )
        .bind(project.client_id)
        .bind(&project.name)
        .bind(&project.start_date)
        .execute(&*db.pool)
        .await?;

        Ok(Project { id: rec.last_insert_rowid(), ..project })
    }
}

pub async fn get_projects(db: web::Data<Db>) -> impl Responder {
    match Project::all(&db).await {
        Ok(projects) => HttpResponse::Ok().json(projects),
        Err(e) => {
            eprintln!("DB error: {}", e);
            HttpResponse::InternalServerError().body("Failed to fetch projects")
        }
    }
}

pub async fn create_project(db: web::Data<Db>, info: web::Json<Project>) -> impl Responder {
    match Project::create(&db, info.into_inner()).await {
        Ok(project) => HttpResponse::Ok().json(project),
        Err(e) => {
            eprintln!("DB error: {}", e);
            HttpResponse::InternalServerError().body("Failed to create project")
        }
    }
}

// ====================== Contracts ======================
#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Contract {
    pub id: i64,
    pub project_id: i64,
    pub signed_at: Option<String>,
    pub details: String,
}

impl Contract {
    pub async fn all(db: &Db) -> SqlxResult<Vec<Self>> {
        sqlx::query_as::<_, Contract>("SELECT * FROM contracts")
            .fetch_all(&*db.pool)
            .await
    }

    pub async fn create(db: &Db, contract: Contract) -> SqlxResult<Self> {
        let rec = sqlx::query(
            "INSERT INTO contracts (project_id, signed_at, details) VALUES (?, ?, ?)"
        )
        .bind(contract.project_id)
        .bind(&contract.signed_at)
        .bind(&contract.details)
        .execute(&*db.pool)
        .await?;

        Ok(Contract { id: rec.last_insert_rowid(), ..contract })
    }
}

pub async fn get_contracts(db: web::Data<Db>) -> impl Responder {
    match Contract::all(&db).await {
        Ok(contracts) => HttpResponse::Ok().json(contracts),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn create_contract(db: web::Data<Db>, info: web::Json<Contract>) -> impl Responder {
    match Contract::create(&db, info.into_inner()).await {
        Ok(contract) => HttpResponse::Ok().json(contract),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

// ====================== Invoices ======================
#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Invoice {
    pub id: i64,
    pub contract_id: i64,
    pub due_date: String,
    pub amount: f64,
}

impl Invoice {
    pub async fn all(db: &Db) -> SqlxResult<Vec<Self>> {
        sqlx::query_as::<_, Invoice>("SELECT * FROM invoices")
            .fetch_all(&*db.pool)
            .await
    }

    pub async fn create(db: &Db, invoice: Invoice) -> SqlxResult<Self> {
        let rec = sqlx::query(
            "INSERT INTO invoices (contract_id, due_date, amount) VALUES (?, ?, ?)"
        )
        .bind(invoice.contract_id)
        .bind(&invoice.due_date)
        .bind(invoice.amount)
        .execute(&*db.pool)
        .await?;

        Ok(Invoice { id: rec.last_insert_rowid(), ..invoice })
    }
}

pub async fn get_invoices(db: web::Data<Db>) -> impl Responder {
    match Invoice::all(&db).await {
        Ok(invoices) => HttpResponse::Ok().json(invoices),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn create_invoice(db: web::Data<Db>, info: web::Json<Invoice>) -> impl Responder {
    match Invoice::create(&db, info.into_inner()).await {
        Ok(invoice) => HttpResponse::Ok().json(invoice),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

// ====================== Payments ======================
#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Payment {
    pub id: i64,
    pub invoice_id: i64,
    pub paid_at: String,
    pub amount: f64,
}

impl Payment {
    pub async fn all(db: &Db) -> SqlxResult<Vec<Self>> {
        sqlx::query_as::<_, Payment>("SELECT * FROM payments")
            .fetch_all(&*db.pool)
            .await
    }

    pub async fn create(db: &Db, payment: Payment) -> SqlxResult<Self> {
        let rec = sqlx::query(
            "INSERT INTO payments (invoice_id, paid_at, amount) VALUES (?, ?, ?)"
        )
        .bind(payment.invoice_id)
        .bind(&payment.paid_at)
        .bind(payment.amount)
        .execute(&*db.pool)
        .await?;

        Ok(Payment { id: rec.last_insert_rowid(), ..payment })
    }
}

pub async fn get_payments(db: web::Data<Db>) -> impl Responder {
    match Payment::all(&db).await {
        Ok(payments) => HttpResponse::Ok().json(payments),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn create_payment(db: web::Data<Db>, info: web::Json<Payment>) -> impl Responder {
    match Payment::create(&db, info.into_inner()).await {
        Ok(payment) => HttpResponse::Ok().json(payment),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}