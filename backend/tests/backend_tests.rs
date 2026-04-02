use actix_web::{test, web, App};
use backend::models::{get_projects};
use backend::db::Db;

#[actix_web::test]
async fn test_get_projects() {
    // Create an in-memory SQLite database for testing
    let db = Db::new(":memory:").await.expect("Failed to create test DB");

    // Initialize the app with the database and route
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db.clone()))
            .route("/api/projects", web::get().to(get_projects))
    )
    .await;

    // Create a GET request
    let req = test::TestRequest::get().uri("/api/projects").to_request();

    // Call the service
    let resp = test::call_service(&app, req).await;

    // Assert the status
    assert!(resp.status().is_success());
}

// Example utils function, make sure your crate has this
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