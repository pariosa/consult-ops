use actix_web::{App, web};
use backend::{db::Db, routes};
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;

pub async fn setup_test_db() -> Db {
    dotenv::dotenv().ok();

    let database_url = std::env::var("TEST_DATABASE_URL")
        .or_else(|_| std::env::var("DATABASE_URL"))
        .expect("TEST_DATABASE_URL or DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("failed to connect to test postgres db");

    sqlx::query("TRUNCATE TABLE organization_subscriptions, party_payment_profiles, notification_jobs, notifications, operational_transactions, agreement_payout_rules, operational_agreements, parties, operational_events, engagement_billing, engagement_milestones, engagements, payments, invoices, contracts, projects, clients, organization_invitations, organization_members, auth_sessions, auth_attempts, audit_events, oauth_accounts, password_reset_tokens, email_verification_tokens, users, organizations RESTART IDENTITY CASCADE")
        .execute(&pool)
        .await
        .expect("failed to reset test database");

    Db {
        pool: Arc::new(pool),
    }
}

pub fn test_app(
    db: Db,
) -> App<
    impl actix_web::dev::ServiceFactory<
        actix_web::dev::ServiceRequest,
        Config = (),
        Response = actix_web::dev::ServiceResponse,
        Error = actix_web::Error,
        InitError = (),
    >,
> {
    App::new()
        .app_data(web::Data::new(db))
        .configure(routes::config)
}
