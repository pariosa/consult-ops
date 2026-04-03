use actix_web::{App, test, web};
use backend::db::Db;
use backend::handlers::{
    create_client, create_contract, create_invoice, create_payment, create_project, get_clients,
    get_contracts, get_invoices, get_payments, get_projects,
};
use chrono::Utc;
use serde_json::json;

#[actix_web::test]
async fn test_client_endpoints() {
    let db = Db::new(":memory:").await.expect("Failed to create test DB");

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db.clone()))
            .route("/api/clients", web::get().to(get_clients))
            .route("/api/clients", web::post().to(create_client)),
    )
    .await;

    // Create client
    let payload = json!({
        "name": "Test Client",
        "email": "client@test.com",
        "phone": "555-1234"
    });
    let req = test::TestRequest::post()
        .uri("/api/clients")
        .set_json(&payload)
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    // Get clients
    let req = test::TestRequest::get().uri("/api/clients").to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}

#[actix_web::test]
async fn test_project_endpoints() {
    let db = Db::new(":memory:").await.expect("Failed to create test DB");

    // Seed client
    let client = backend::models::client::Client::create(
        &db,
        backend::models::client::Client {
            id: 0,
            name: "Demo Client".to_string(),
            email: "demo@example.com".to_string(),
            company_name: None,
            phone: Some("555-5555".to_string()),
            city: None,
            state: None,
            country: None,
            zip: None,
            tax_id: None,
            address: None,
            created_at: Some(Utc::now().to_rfc3339()),
            updated_at: Some(Utc::now().to_rfc3339()),
        },
    )
    .await
    .unwrap();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db.clone()))
            .route("/api/projects", web::get().to(get_projects))
            .route("/api/projects", web::post().to(create_project)),
    )
    .await;

    let payload = json!({
        "client_id": client.id,
        "name": "Test Project",
        "start_date": Utc::now().to_rfc3339()
    });

    let req = test::TestRequest::post()
        .uri("/api/projects")
        .set_json(&payload)
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let req = test::TestRequest::get().uri("/api/projects").to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}

#[actix_web::test]
async fn test_contract_endpoints() {
    let db = Db::new(":memory:").await.expect("Failed to create test DB");

    // Seed client & project
    let client = backend::models::client::Client::create(
        &db,
        backend::models::client::Client {
            id: 0,
            name: "Demo Client".to_string(),
            email: "demo@example.com".to_string(),
            company_name: None,
            phone: Some("555-5555".to_string()),
            city: None,
            state: None,
            country: None,
            zip: None,
            tax_id: None,
            address: None,
            created_at: Some(Utc::now().to_rfc3339()),
            updated_at: Some(Utc::now().to_rfc3339()),
        },
    )
    .await
    .unwrap();

    let project = backend::models::project::Project::create(
        &db,
        backend::models::project::CreateProject {
            client_id: client.id,
            name: "Contract Project".to_string(),
            start_date: Some(Utc::now().to_rfc3339()),
            end_date: None,
            description: Some("Test description".to_string()),
        },
    )
    .await
    .unwrap();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db.clone()))
            .route("/api/contracts", web::get().to(get_contracts))
            .route("/api/contracts", web::post().to(create_contract)),
    )
    .await;

    let payload = json!({
        "project_id": project.id,
        "title": "Test Contract",
        "status": "active",
        "signed_at": Utc::now().to_rfc3339(),
        "start_date": Utc::now().to_rfc3339(),
        "end_date": null,
        "value": 1000.0,
        "currency": "USD",
        "terms": "Test terms",
        "notes": "Test notes",
        "external_id": "EXT123",
        "created_at": Utc::now().to_rfc3339()
    });

    let req = test::TestRequest::post()
        .uri("/api/contracts")
        .set_json(&payload)
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let req = test::TestRequest::get().uri("/api/contracts").to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}

#[actix_web::test]
async fn test_invoice_endpoints() {
    let db = Db::new(":memory:").await.expect("Failed to create test DB");

    // Seed client, project, contract
    let client = backend::models::client::Client::create(
        &db,
        backend::models::client::Client {
            id: 0,
            name: "Demo Client".to_string(),
            email: "demo@example.com".to_string(),
            company_name: None,
            phone: None,
            city: None,
            state: None,
            country: None,
            zip: None,
            tax_id: None,
            address: None,
            created_at: Some(Utc::now().to_rfc3339()),
            updated_at: Some(Utc::now().to_rfc3339()),
        },
    )
    .await
    .unwrap();

    let project = backend::models::project::Project::create(
        &db,
        backend::models::project::CreateProject {
            client_id: client.id,
            name: "Invoice Project".to_string(),
            start_date: Some(Utc::now().to_rfc3339()),
            end_date: None,
            description: Some("Test description".to_string()),
        },
    )
    .await
    .unwrap();

    let contract = backend::models::contract::Contract::create(
        &db,
        backend::models::contract::Contract {
            id: 0,
            project_id: project.id,
            title: "Invoice Contract".to_string(),
            status: "active".to_string(),
            signed_at: Some(Utc::now().to_rfc3339()),
            start_date: Some(Utc::now().to_rfc3339()),
            end_date: None,
            value: Some(2000.0),
            currency: Some("USD".to_string()),
            terms: Some("Terms".to_string()),
            notes: Some("Notes".to_string()),
            external_id: Some("INV123".to_string()),
            created_at: Utc::now().to_rfc3339(),
        },
    )
    .await
    .unwrap();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db.clone()))
            .route("/api/invoices", web::get().to(get_invoices))
            .route("/api/invoices", web::post().to(create_invoice)),
    )
    .await;

    let payload = json!({
        "contract_id": contract.id,
        "invoice_number": "INV-001",
        "status": "pending",
        "issued_at": Utc::now().to_rfc3339(),
        "due_date": Utc::now().to_rfc3339(),
        "subtotal": 1000.0,
        "tax": 100.0,
        "total": 1100.0,
        "currency": "USD",
        "notes": "Invoice notes",
        "created_at": Utc::now().to_rfc3339()
    });

    let req = test::TestRequest::post()
        .uri("/api/invoices")
        .set_json(&payload)
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let req = test::TestRequest::get().uri("/api/invoices").to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}

#[actix_web::test]
async fn test_payment_endpoints() {
    let db = Db::new(":memory:").await.expect("Failed to create test DB");

    // Seed client, project, contract, invoice
    let client = backend::models::client::Client::create(
        &db,
        backend::models::client::Client {
            id: 0,
            name: "Demo Client".to_string(),
            email: "demo@example.com".to_string(),
            company_name: None,
            phone: None,
            city: None,
            state: None,
            country: None,
            zip: None,
            tax_id: None,
            address: None,
            created_at: Some(Utc::now().to_rfc3339()),
            updated_at: Some(Utc::now().to_rfc3339()),
        },
    )
    .await
    .unwrap();

    let project = backend::models::project::Project::create(
        &db,
        backend::models::project::CreateProject {
            client_id: client.id,
            name: "Demo Project".to_string(),
            start_date: Some(Utc::now().to_rfc3339()),
            end_date: None,
            description: Some("Demo description".to_string()),
        },
    )
    .await
    .unwrap();

    let contract = backend::models::contract::Contract::create(
        &db,
        backend::models::contract::Contract {
            id: 0,
            project_id: project.id,
            title: "Payment Contract".to_string(),
            status: "active".to_string(),
            signed_at: Some(Utc::now().to_rfc3339()),
            start_date: Some(Utc::now().to_rfc3339()),
            end_date: None,
            value: Some(3000.0),
            currency: Some("USD".to_string()),
            terms: Some("Payment terms".to_string()),
            notes: Some("Payment notes".to_string()),
            external_id: Some("PAY123".to_string()),
            created_at: Utc::now().to_rfc3339(),
        },
    )
    .await
    .unwrap();

    let invoice = backend::models::invoice::Invoice::create(
        &db,
        backend::models::invoice::Invoice {
            id: 0,
            contract_id: contract.id,
            invoice_number: "PAY-001".to_string(),
            status: "pending".to_string(),
            issued_at: Some(Utc::now().to_rfc3339()),
            due_date: Some(Utc::now().to_rfc3339()),
            subtotal: Some(1500.0),
            tax: Some(150.0),
            total: Some(1650.0),
            currency: Some("USD".to_string()),
            notes: Some("Payment invoice".to_string()),
            created_at: Utc::now().to_rfc3339(),
        },
    )
    .await
    .unwrap();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db.clone()))
            .route("/api/payments", web::get().to(get_payments))
            .route("/api/payments", web::post().to(create_payment)),
    )
    .await;

    let payload = json!({
        "invoice_id": invoice.id,
        "amount": 1650.0,
        "paid_at": Utc::now().to_rfc3339(),
        "currency": "USD",
        "method": "card",
        "reference": "TXN123",
        "notes": "Full payment"
    });

    let req = test::TestRequest::post()
        .uri("/api/payments")
        .set_json(&payload)
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let req = test::TestRequest::get().uri("/api/payments").to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}

// Optional: currency formatting helper
mod utils {
    pub fn format_currency(amount: f64) -> String {
        format!("${:.2}", amount)
    }
}

#[actix_web::test]
async fn test_invoice_formatting() {
    let amount = 1200.5;
    let formatted = utils::format_currency(amount);
    assert_eq!(formatted, "$1200.50");
}
