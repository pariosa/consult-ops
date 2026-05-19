// tests/party_payment_endpoint_tests.rs

use std::sync::Arc;

use actix_web::{App, test, web};
use backend::auth;
use backend::auth::hash_password;
use backend::db::Db;
use backend::handlers::party::{
    create_organization_party, get_party_payment_readiness, mark_party_payer_authorized_dev,
    mark_party_payout_ready_dev, upsert_party_payment_profile, verify_party,
};
use sqlx::{Executor, SqlitePool};

fn app_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/auth/login", web::post().to(auth::login))
            .route(
                "/organizations/{organization_id}/parties",
                web::post().to(create_organization_party),
            )
            .route(
                "/parties/{id}/payment-readiness",
                web::get().to(get_party_payment_readiness),
            )
            .route(
                "/parties/{id}/payment-profile",
                web::post().to(upsert_party_payment_profile),
            )
            .route("/parties/{id}/verify", web::post().to(verify_party))
            .route(
                "/parties/{id}/payout-ready/dev",
                web::post().to(mark_party_payout_ready_dev),
            )
            .route(
                "/parties/{id}/payer-authorized/dev",
                web::post().to(mark_party_payer_authorized_dev),
            ),
    );
}

async fn setup_db() -> Db {
    let pool = SqlitePool::connect("sqlite::memory:")
        .await
        .expect("failed to create sqlite memory db");

    pool.execute(
        r#"
        CREATE TABLE users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            email TEXT NOT NULL UNIQUE,
            password_hash TEXT NOT NULL,
            name TEXT,
            user_type TEXT NOT NULL DEFAULT 'member',
            created_at TEXT,
            updated_at TEXT
        );
        "#,
    )
    .await
    .unwrap();

    pool.execute(
        r#"
        CREATE TABLE parties (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            organization_id INTEGER NOT NULL,
            name TEXT NOT NULL,
            email TEXT,
            party_type TEXT NOT NULL,

            is_verified INTEGER NOT NULL DEFAULT 0,
            verification_status TEXT NOT NULL DEFAULT 'unverified',
            verified_at TEXT,
            verification_method TEXT,

            linked_user_id INTEGER,
            linked_client_id INTEGER,
            linked_organization_id INTEGER,

            created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
        );
        "#,
    )
    .await
    .unwrap();

    pool.execute(
        r#"
        CREATE TABLE party_payment_profiles (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            party_id INTEGER NOT NULL,
            organization_id INTEGER NOT NULL,

            payment_role TEXT NOT NULL,

            stripe_customer_id TEXT,
            stripe_payment_method_id TEXT,
            payer_authorization_status TEXT NOT NULL DEFAULT 'not_configured',
            payer_authorized_at TEXT,
            payer_authorization_scope TEXT,

            stripe_connect_account_id TEXT,
            stripe_connect_onboarding_status TEXT NOT NULL DEFAULT 'not_started',
            payout_status TEXT NOT NULL DEFAULT 'not_ready',
            payout_verified_at TEXT,

            created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
        );
        "#,
    )
    .await
    .unwrap();

    pool.execute(
        r#"
        CREATE UNIQUE INDEX idx_party_payment_profiles_party_id
        ON party_payment_profiles(party_id);
        "#,
    )
    .await
    .unwrap();

    pool.execute(
        r#"
        CREATE TABLE operational_events (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            organization_id INTEGER NOT NULL,
            actor_user_id INTEGER,
            entity_type TEXT NOT NULL,
            entity_id INTEGER NOT NULL,
            event_type TEXT NOT NULL,
            from_status TEXT,
            to_status TEXT,
            metadata TEXT NOT NULL DEFAULT '{}',
            created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
        );
        "#,
    )
    .await
    .unwrap();

    let password_hash = hash_password("DemoPass123!").unwrap();

    sqlx::query(
        r#"
        INSERT INTO users (
            email,
            password_hash,
            name,
            user_type,
            created_at,
            updated_at
        )
        VALUES (?, ?, ?, ?, datetime('now'), datetime('now'))
        "#,
    )
    .bind("admin@atlas.test")
    .bind(password_hash)
    .bind("Atlas Admin")
    .bind("admin")
    .execute(&pool)
    .await
    .unwrap();

    Db {
        pool: Arc::new(pool),
    }
}

#[actix_rt::test]
async fn admin_can_create_party_and_read_default_payment_readiness() {
    let db = setup_db().await;

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(app_config),
    )
    .await;

    let login_payload = serde_json::json!({
        "email": "admin@atlas.test",
        "password": "DemoPass123!"
    });

    let login_req = test::TestRequest::post()
        .uri("/api/auth/login")
        .set_json(&login_payload)
        .to_request();

    let login_resp: serde_json::Value = test::call_and_read_body_json(&app, login_req).await;

    let token = login_resp["token"].as_str().unwrap().to_string();
    let payload = serde_json::json!({
        "name": "Manual Contractor",
        "email": "contractor@example.com",
        "party_type": "contractor"
    });

    let req = test::TestRequest::post()
        .uri("/api/organizations/1/parties")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(&payload)
        .to_request();

    let created: serde_json::Value = test::call_and_read_body_json(&app, req).await;

    assert_eq!(created["name"], "Manual Contractor");
    assert_eq!(created["is_verified"], 0);
    assert_eq!(created["verification_status"], "unverified");

    let req = test::TestRequest::get()
        .uri("/api/parties/1/payment-readiness")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();

    let readiness: serde_json::Value = test::call_and_read_body_json(&app, req).await;

    assert_eq!(readiness["is_verified"], false);
    assert_eq!(readiness["payer_ready"], false);
    assert_eq!(readiness["payee_ready"], false);
}

#[actix_rt::test]
async fn admin_can_verify_party_and_authorize_payer_profile() {
    let db = setup_db().await;

    sqlx::query(
        r#"
        INSERT INTO parties (
            organization_id,
            name,
            email,
            party_type,
            is_verified,
            verification_status
        )
        VALUES (
            1,
            'Riverbend Client',
            'ops@riverbend.gov',
            'client',
            0,
            'unverified'
        )
        "#,
    )
    .execute(db.pool.as_ref())
    .await
    .unwrap();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(app_config),
    )
    .await;
    let login_payload = serde_json::json!({
        "email": "admin@atlas.test",
        "password": "DemoPass123!"
    });

    let login_req = test::TestRequest::post()
        .uri("/api/auth/login")
        .set_json(&login_payload)
        .to_request();

    let login_resp: serde_json::Value = test::call_and_read_body_json(&app, login_req).await;

    let token = login_resp["token"].as_str().unwrap().to_string();
    let req = test::TestRequest::post()
        .uri("/api/parties/1/verify")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();

    let verified: serde_json::Value = test::call_and_read_body_json(&app, req).await;

    assert_eq!(verified["is_verified"], 1);
    assert_eq!(verified["verification_status"], "verified");
    assert_eq!(verified["verification_method"], "admin");

    let payload = serde_json::json!({
        "payment_role": "payer",
        "payer_authorization_scope": "agreement"
    });

    let req = test::TestRequest::post()
        .uri("/api/parties/1/payment-profile")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(&payload)
        .to_request();

    let profile: serde_json::Value = test::call_and_read_body_json(&app, req).await;

    assert_eq!(profile["payment_role"], "payer");
    assert_eq!(profile["payer_authorization_status"], "not_configured");

    let req = test::TestRequest::post()
        .uri("/api/parties/1/payer-authorized/dev")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();

    let authorized: serde_json::Value = test::call_and_read_body_json(&app, req).await;

    assert_eq!(authorized["payer_authorization_status"], "authorized");

    assert_eq!(authorized["stripe_customer_id"], "cus_dev_party_1");

    assert_eq!(authorized["stripe_payment_method_id"], "pm_dev_party_1");

    let req = test::TestRequest::get()
        .uri("/api/parties/1/payment-readiness")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();

    let readiness: serde_json::Value = test::call_and_read_body_json(&app, req).await;

    assert_eq!(readiness["is_verified"], true);
    assert_eq!(readiness["payer_ready"], true);
    assert_eq!(readiness["payee_ready"], false);
}

#[actix_rt::test]
async fn admin_can_mark_payee_payout_ready() {
    let db = setup_db().await;

    sqlx::query(
        r#"
        INSERT INTO parties (
            organization_id,
            name,
            email,
            party_type,
            is_verified,
            verification_status,
            verified_at,
            verification_method
        )
        VALUES (
            1,
            'Riley Operations',
            'ops@atlas.test',
            'contractor',
            1,
            'verified',
            datetime('now'),
            'admin'
        )
        "#,
    )
    .execute(db.pool.as_ref())
    .await
    .unwrap();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .configure(app_config),
    )
    .await;

    let login_payload = serde_json::json!({
        "email": "admin@atlas.test",
        "password": "DemoPass123!"
    });

    let login_req = test::TestRequest::post()
        .uri("/api/auth/login")
        .set_json(&login_payload)
        .to_request();

    let login_resp: serde_json::Value = test::call_and_read_body_json(&app, login_req).await;

    let token = login_resp["token"].as_str().unwrap().to_string();

    let payload = serde_json::json!({
        "payment_role": "payee"
    });

    let req = test::TestRequest::post()
        .uri("/api/parties/1/payment-profile")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(&payload)
        .to_request();

    let profile: serde_json::Value = test::call_and_read_body_json(&app, req).await;

    assert_eq!(profile["payment_role"], "payee");
    assert_eq!(profile["payout_status"], "not_ready");

    let req = test::TestRequest::post()
        .uri("/api/parties/1/payout-ready/dev")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();

    let ready: serde_json::Value = test::call_and_read_body_json(&app, req).await;

    assert_eq!(ready["stripe_connect_account_id"], "acct_dev_party_1");

    assert_eq!(ready["stripe_connect_onboarding_status"], "complete");

    assert_eq!(ready["payout_status"], "ready");

    let req = test::TestRequest::get()
        .uri("/api/parties/1/payment-readiness")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();

    let readiness: serde_json::Value = test::call_and_read_body_json(&app, req).await;

    assert_eq!(readiness["is_verified"], true);
    assert_eq!(readiness["payer_ready"], false);
    assert_eq!(readiness["payee_ready"], true);
}
