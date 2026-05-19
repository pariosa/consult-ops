use backend::models::party::{CreateParty, Party};
use backend::models::party_payment_profile::PartyPaymentProfile;
use sqlx::{Executor, SqlitePool};

async fn setup_db() -> SqlitePool {
    let pool = SqlitePool::connect("sqlite::memory:")
        .await
        .expect("failed to create sqlite memory db");

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
        CREATE TABLE users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT,
            email TEXT NOT NULL,
            user_type TEXT NOT NULL DEFAULT 'member'
        );
        "#,
    )
    .await
    .unwrap();

    pool.execute(
        r#"
        CREATE TABLE clients (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            organization_id INTEGER NOT NULL,
            name TEXT NOT NULL,
            company_name TEXT,
            email TEXT
        );
        "#,
    )
    .await
    .unwrap();

    pool
}

#[actix_rt::test]
async fn creates_unverified_manual_party() {
    let db = setup_db().await;

    let party = Party::create(
        &db,
        1,
        CreateParty {
            name: "Manual Vendor".to_string(),
            email: Some("vendor@example.com".to_string()),
            party_type: "contractor".to_string(),
            linked_user_id: None,
            linked_client_id: None,
            linked_organization_id: None,
            is_verified: None,
            verification_status: None,
            verification_method: None,
        },
    )
    .await
    .unwrap();

    assert_eq!(party.organization_id, 1);
    assert_eq!(party.name, "Manual Vendor");
    assert_eq!(party.is_verified, 0);
    assert_eq!(party.verification_status, "unverified");
    assert!(party.verified_at.is_none());
}

#[actix_rt::test]
async fn creates_verified_manual_party() {
    let db = setup_db().await;

    let party = Party::create(
        &db,
        1,
        CreateParty {
            name: "Verified Vendor".to_string(),
            email: Some("verified@example.com".to_string()),
            party_type: "contractor".to_string(),
            linked_user_id: None,
            linked_client_id: None,
            linked_organization_id: None,
            is_verified: Some(1),
            verification_status: None,
            verification_method: Some("admin".to_string()),
        },
    )
    .await
    .unwrap();

    assert_eq!(party.is_verified, 1);
    assert_eq!(party.verification_status, "verified");
    assert_eq!(party.verification_method.as_deref(), Some("admin"));
    assert!(party.verified_at.is_some());
}

#[actix_rt::test]
async fn creates_verified_user_party() {
    let db = setup_db().await;

    sqlx::query(
        r#"
        INSERT INTO users (name, email, user_type)
        VALUES ('Riley Operations', 'ops@atlas.test', 'contractor');
        "#,
    )
    .execute(&db)
    .await
    .unwrap();

    let party = Party::create_verified_user_party(&db, 1, 1, "contractor")
        .await
        .unwrap();

    assert_eq!(party.name, "Riley Operations");
    assert_eq!(party.email.as_deref(), Some("ops@atlas.test"));
    assert_eq!(party.linked_user_id, Some(1));
    assert_eq!(party.is_verified, 1);
    assert_eq!(party.verification_status, "verified");
    assert_eq!(party.verification_method.as_deref(), Some("linked_user"));
}

#[actix_rt::test]
async fn creates_verified_client_party() {
    let db = setup_db().await;

    sqlx::query(
        r#"
        INSERT INTO clients (organization_id, name, company_name, email)
        VALUES (1, 'Riverbend Water', 'Riverbend Municipal Water Authority', 'ops@riverbend.gov');
        "#,
    )
    .execute(&db)
    .await
    .unwrap();

    let party = Party::create_verified_client_party(&db, 1, 1)
        .await
        .unwrap();

    assert_eq!(party.name, "Riverbend Municipal Water Authority");
    assert_eq!(party.email.as_deref(), Some("ops@riverbend.gov"));
    assert_eq!(party.linked_client_id, Some(1));
    assert_eq!(party.party_type, "client");
    assert_eq!(party.is_verified, 1);
    assert_eq!(party.verification_status, "verified");
    assert_eq!(party.verification_method.as_deref(), Some("linked_client"));
}

#[actix_rt::test]
async fn upserts_party_payment_profile_as_payee() {
    let db = setup_db().await;

    let party = Party::create(
        &db,
        1,
        CreateParty {
            name: "Payout Contractor".to_string(),
            email: Some("payee@example.com".to_string()),
            party_type: "contractor".to_string(),
            linked_user_id: None,
            linked_client_id: None,
            linked_organization_id: None,
            is_verified: Some(1),
            verification_status: None,
            verification_method: Some("admin".to_string()),
        },
    )
    .await
    .unwrap();

    let profile = PartyPaymentProfile::upsert_basic(&db, party.id, 1, "payee", None)
        .await
        .unwrap();

    assert_eq!(profile.party_id, party.id);
    assert_eq!(profile.organization_id, 1);
    assert_eq!(profile.payment_role, "payee");
    assert_eq!(profile.payout_status, "not_ready");
    assert_eq!(profile.stripe_connect_onboarding_status, "not_started");
}

#[actix_rt::test]
async fn upserts_party_payment_profile_role_without_duplicate() {
    let db = setup_db().await;

    let party = Party::create(
        &db,
        1,
        CreateParty {
            name: "Dual Role Party".to_string(),
            email: Some("dual@example.com".to_string()),
            party_type: "contractor".to_string(),
            linked_user_id: None,
            linked_client_id: None,
            linked_organization_id: None,
            is_verified: Some(1),
            verification_status: None,
            verification_method: Some("admin".to_string()),
        },
    )
    .await
    .unwrap();

    PartyPaymentProfile::upsert_basic(&db, party.id, 1, "payee", None)
        .await
        .unwrap();

    let updated =
        PartyPaymentProfile::upsert_basic(&db, party.id, 1, "both", Some("agreement".to_string()))
            .await
            .unwrap();

    let count: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM party_payment_profiles WHERE party_id = ?")
            .bind(party.id)
            .fetch_one(&db)
            .await
            .unwrap();

    assert_eq!(count, 1);
    assert_eq!(updated.payment_role, "both");
    assert_eq!(
        updated.payer_authorization_scope.as_deref(),
        Some("agreement")
    );
}

#[actix_rt::test]
async fn marks_party_payout_ready() {
    let db = setup_db().await;

    let party = Party::create(
        &db,
        1,
        CreateParty {
            name: "Ready Payee".to_string(),
            email: Some("ready@example.com".to_string()),
            party_type: "contractor".to_string(),
            linked_user_id: None,
            linked_client_id: None,
            linked_organization_id: None,
            is_verified: Some(1),
            verification_status: None,
            verification_method: Some("admin".to_string()),
        },
    )
    .await
    .unwrap();

    PartyPaymentProfile::upsert_basic(&db, party.id, 1, "payee", None)
        .await
        .unwrap();

    let profile =
        PartyPaymentProfile::mark_payout_ready(&db, party.id, "acct_test_123".to_string())
            .await
            .unwrap();

    assert_eq!(
        profile.stripe_connect_account_id.as_deref(),
        Some("acct_test_123")
    );
    assert_eq!(profile.stripe_connect_onboarding_status, "complete");
    assert_eq!(profile.payout_status, "ready");
    assert!(profile.payout_verified_at.is_some());
}

#[actix_rt::test]
async fn marks_payer_authorized() {
    let db = setup_db().await;

    let party = Party::create(
        &db,
        1,
        CreateParty {
            name: "Authorized Payer".to_string(),
            email: Some("payer@example.com".to_string()),
            party_type: "client".to_string(),
            linked_user_id: None,
            linked_client_id: None,
            linked_organization_id: None,
            is_verified: Some(1),
            verification_status: None,
            verification_method: Some("admin".to_string()),
        },
    )
    .await
    .unwrap();

    PartyPaymentProfile::upsert_basic(
        &db,
        party.id,
        1,
        "payer",
        Some("single_milestone".to_string()),
    )
    .await
    .unwrap();

    let profile = PartyPaymentProfile::mark_payer_authorized(
        &db,
        party.id,
        "cus_test_123".to_string(),
        "pm_test_123".to_string(),
        "single_milestone".to_string(),
    )
    .await
    .unwrap();

    assert_eq!(profile.stripe_customer_id.as_deref(), Some("cus_test_123"));
    assert_eq!(
        profile.stripe_payment_method_id.as_deref(),
        Some("pm_test_123")
    );
    assert_eq!(profile.payer_authorization_status, "authorized");
    assert_eq!(
        profile.payer_authorization_scope.as_deref(),
        Some("single_milestone"),
    );
    assert!(profile.payer_authorized_at.is_some());
}
