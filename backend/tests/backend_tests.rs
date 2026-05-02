use actix_web::{App, test, web};
use backend::db::Db;
use backend::handlers::{
    create_client, create_contract, create_invoice, create_payment, create_project, create_user,
    get_clients, get_contracts, get_invoices, get_payments, get_projects,
};
use chrono::Utc;
use serde_json::json;

async fn seed_test_org(db: &Db) -> i64 {
    let rec = sqlx::query(
        r#"
        INSERT INTO organizations (name, created_at, updated_at)
        VALUES (?, ?, ?)
        "#,
    )
    .bind("Test Organization")
    .bind(Utc::now().to_rfc3339())
    .bind(Utc::now().to_rfc3339())
    .execute(&*db.pool)
    .await
    .expect("Failed to seed test organization");

    rec.last_insert_rowid()
}

#[actix_web::test]
async fn test_client_endpoints() {
    let db = Db::new(":memory:").await.expect("Failed to create test DB");
    let organization_id = seed_test_org(&db).await;

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db.clone()))
            .route("/api/clients", web::get().to(get_clients))
            .route("/api/clients", web::post().to(create_client)),
    )
    .await;

    // Create client
    let payload = json!({
        "organization_id": organization_id,
        "name": "Test Client",
        "email": "client@test.com",
        "phone": "555-1234"
    });
    let req = test::TestRequest::post()
        .uri("/api/clients")
        .set_json(&payload)
        .to_request();
    let resp = test::call_service(&app, req).await;
    let status = resp.status();
    let body = test::read_body(resp).await;
    let body_text = String::from_utf8_lossy(&body);

    assert!(
        status.is_success(),
        "Expected success but got {}: {}",
        status,
        body_text
    );

    // Get clients
    let req = test::TestRequest::get().uri("/api/clients").to_request();
    let resp = test::call_service(&app, req).await;
    let status = resp.status();
    let body = test::read_body(resp).await;
    let body_text = String::from_utf8_lossy(&body);

    assert!(
        status.is_success(),
        "Expected success but got {}: {}",
        status,
        body_text
    );
}

#[actix_web::test]
async fn test_project_endpoints() {
    let db = Db::new(":memory:").await.expect("Failed to create test DB");
    let organization_id = seed_test_org(&db).await;

    // Seed client
    let client = backend::models::client::Client::create(
        &db,
        backend::models::client::CreateClient {
            organization_id: organization_id,
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
        "organization_id": organization_id,
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
    let organization_id = seed_test_org(&db).await;

    // Seed client & project
    let client = backend::models::client::Client::create(
        &db,
        backend::models::client::CreateClient {
            organization_id: organization_id,
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
        },
    )
    .await
    .unwrap();

    let project = backend::models::project::Project::create(
        &db,
        backend::models::project::CreateProject {
            organization_id: organization_id,
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
        "organization_id": organization_id,
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
    let status = resp.status();
    let body = test::read_body(resp).await;
    let body_text = String::from_utf8_lossy(&body);

    assert!(
        status.is_success(),
        "Expected success but got {}: {}",
        status,
        body_text
    );
    let req = test::TestRequest::get().uri("/api/contracts").to_request();
    let resp = test::call_service(&app, req).await;
    let status = resp.status();
    let body = test::read_body(resp).await;
    let body_text = String::from_utf8_lossy(&body);

    assert!(
        status.is_success(),
        "Expected success but got {}: {}",
        status,
        body_text
    );
}

#[actix_web::test]
async fn test_invoice_endpoints() {
    let db = Db::new(":memory:").await.expect("Failed to create test DB");
    let organization_id = seed_test_org(&db).await;

    // Seed client, project, contract
    let client = backend::models::client::Client::create(
        &db,
        backend::models::client::CreateClient {
            organization_id: organization_id,
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
        },
    )
    .await
    .unwrap();

    let project = backend::models::project::Project::create(
        &db,
        backend::models::project::CreateProject {
            organization_id: organization_id,
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
        backend::models::contract::CreateContract {
            organization_id: organization_id,
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
        "organization_id": organization_id,
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
    let status = resp.status();
    let body = test::read_body(resp).await;
    let body_text = String::from_utf8_lossy(&body);

    assert!(
        status.is_success(),
        "Expected success but got {}: {}",
        status,
        body_text
    );
    let req = test::TestRequest::get().uri("/api/invoices").to_request();
    let resp = test::call_service(&app, req).await;
    let status = resp.status();
    let body = test::read_body(resp).await;
    let body_text = String::from_utf8_lossy(&body);

    assert!(
        status.is_success(),
        "Expected success but got {}: {}",
        status,
        body_text
    );
}

#[actix_web::test]
async fn test_payment_endpoints() {
    let db = Db::new(":memory:").await.expect("Failed to create test DB");
    let organization_id = seed_test_org(&db).await;

    // Seed client, project, contract, invoice
    let client = backend::models::client::Client::create(
        &db,
        backend::models::client::CreateClient {
            organization_id: organization_id,
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
        },
    )
    .await
    .unwrap();

    let project = backend::models::project::Project::create(
        &db,
        backend::models::project::CreateProject {
            client_id: client.id,
            organization_id: organization_id,
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
        backend::models::contract::CreateContract {
            project_id: project.id,
            organization_id: organization_id,
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
        backend::models::invoice::CreateInvoice {
            organization_id: organization_id,
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
        "organization_id": organization_id,
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

#[actix_web::test]
async fn test_create_user_endpoint() {
    let db = Db::new(":memory:").await.expect("Failed to create test DB");

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db.clone()))
            .route("/api/admin/users", web::post().to(create_user)),
    )
    .await;

    let payload = json!({
        "email": "new-user@test.com",
        "password": "StrongPass123!",
        "name": "New User",
        "user_type": "consultant"
    });

    let req = test::TestRequest::post()
        .uri("/api/admin/users")
        .set_json(&payload)
        .to_request();

    let resp = test::call_service(&app, req).await;

    assert!(resp.status().is_success());

    let user_type = sqlx::query_scalar::<_, String>("SELECT user_type FROM users WHERE email = ?")
        .bind("new-user@test.com")
        .fetch_one(&*db.pool)
        .await
        .unwrap();

    assert_eq!(user_type, "consultant");
}
