// src/seed_demo.rs
use crate::db::Db;
use crate::models::project::CreateProject;
use crate::models::{Client, Contract, Invoice, Payment, Project};
use chrono::{Duration, Utc};

pub async fn seed_demo_data(db: &Db) -> sqlx::Result<()> {
    // -------------------------
    // Seed Client
    // -------------------------
    let clients = Client::all(db).await?;
    let client = if clients.is_empty() {
        Client::create(
            db,
            Client {
                id: 0,
                name: "Demo Client".to_string(),
                email: "demo@example.com".to_string(),
                company_name: Some("Demo company".to_string()),
                phone: Some("555-5555".to_string()),
                city: Some("Capital City".to_string()),
                state: Some("California".to_string()),
                country: Some("United States".to_string()),
                zip: Some("12345".to_string()),
                tax_id: None,
                address: None,
                created_at: Some(Utc::now().to_string()),
                updated_at: Some(Utc::now().to_string()),
            },
        )
        .await?
    } else {
        clients.first().unwrap().clone()
    };

    // -------------------------
    // Seed Project
    // -------------------------
    let projects = Project::all(db).await?;
    let project = if projects.is_empty() {
        Project::create(
            db,
            CreateProject {
                client_id: client.id,
                name: "Demo Project".to_string(),
                start_date: Some(Utc::now().to_string()),
                description: Some("Demo Project description 00000101010101".to_string()),
                end_date: None,
            },
        )
        .await?
    } else {
        projects.first().unwrap().clone()
    };

    // -------------------------
    // Seed Contract
    // -------------------------
    let contracts = Contract::all(db).await?;
    let contract = if contracts.is_empty() {
        Contract::create(
            db,
            Contract {
                id: 0,
                project_id: project.id,
                title: "Demo Contract".to_string(),
                status: "Active".to_string(),
                signed_at: Some(Utc::now().to_string()),
                start_date: Some(Utc::now().to_string()),
                end_date: Some((Utc::now() + Duration::days(30)).to_string()),
                value: Some(1000.0),
                currency: Some("USD".to_string()),
                terms: Some("Standard demo terms".to_string()),
                notes: Some("Demo notes".to_string()),
                external_id: None,
                created_at: Utc::now().to_string(),
            },
        )
        .await?
    } else {
        contracts.first().unwrap().clone()
    };

    // -------------------------
    // Seed Invoice
    // -------------------------
    let invoices = Invoice::all(db).await?;
    let invoice = if invoices.is_empty() {
        Invoice::create(
            db,
            Invoice {
                id: 0,
                contract_id: contract.id,
                invoice_number: "INV-001".to_string(),
                status: "Pending".to_string(),
                issued_at: Some(Utc::now().to_string()),
                due_date: Some((Utc::now() + Duration::days(15)).to_string()),
                subtotal: Some(1000.0),
                tax: Some(100.0),
                total: Some(1100.0),
                currency: Some("USD".to_string()),
                notes: Some("Demo invoice notes".to_string()),
                created_at: Utc::now().to_string(),
            },
        )
        .await?
    } else {
        invoices.first().unwrap().clone()
    };

    // -------------------------
    // Seed Payment
    // -------------------------
    let payments = Payment::all(db).await?;
    if payments.is_empty() {
        Payment::create(
            db,
            Payment {
                id: 0,
                invoice_id: invoice.id,
                paid_at: Some(Utc::now().to_string()),
                amount: 1100.0,
                currency: Some("USD".to_string()),
                method: Some("Credit Card".to_string()),
                reference: Some("DEMO-001".to_string()),
                notes: Some("Demo payment".to_string()),
                created_at: Utc::now().to_string(),
            },
        )
        .await?;
    }

    Ok(())
}
