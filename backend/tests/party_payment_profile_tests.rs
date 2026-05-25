mod common;

use backend::auth::hash_password;
use backend::models::party::{CreateParty, Party};
use backend::models::party_payment_profile::PartyPaymentProfile;
use chrono::Utc;
use common::setup_test_db;
use serial_test::serial;

async fn seed_org(db: &backend::db::Db) -> i64 {
    let now = Utc::now().to_rfc3339();
    let slug = format!(
        "party-profile-org-{}",
        Utc::now().timestamp_nanos_opt().unwrap_or_default()
    );

    sqlx::query_scalar::<_, i64>(
        r#"
        INSERT INTO organizations (name, slug, created_at, updated_at)
        VALUES ($1, $2, $3, $4)
        RETURNING id
        "#,
    )
    .bind("Party Profile Test Org")
    .bind(slug)
    .bind(&now)
    .bind(&now)
    .fetch_one(db.pool.as_ref())
    .await
    .unwrap()
}

async fn seed_user(db: &backend::db::Db, email: &str, name: &str, user_type: &str) -> i64 {
    let now = Utc::now().to_rfc3339();
    let password_hash = hash_password("Password123!").unwrap();

    sqlx::query_scalar::<_, i64>(
        r#"
        INSERT INTO users (
            email,
            password_hash,
            name,
            user_type,
            email_verified_at,
            created_at,
            updated_at
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING id
        "#,
    )
    .bind(email)
    .bind(password_hash)
    .bind(name)
    .bind(user_type)
    .bind(&now)
    .bind(&now)
    .bind(&now)
    .fetch_one(db.pool.as_ref())
    .await
    .unwrap()
}

async fn seed_client(db: &backend::db::Db, organization_id: i64) -> i64 {
    let now = Utc::now().to_rfc3339();

    sqlx::query_scalar::<_, i64>(
        r#"
        INSERT INTO clients (
            organization_id,
            name,
            company_name,
            email,
            created_at,
            updated_at
        )
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING id
        "#,
    )
    .bind(organization_id)
    .bind("Riverbend Water")
    .bind("Riverbend Municipal Water Authority")
    .bind("ops@riverbend.gov")
    .bind(&now)
    .bind(&now)
    .fetch_one(db.pool.as_ref())
    .await
    .unwrap()
}

#[actix_rt::test]
#[serial]
async fn creates_unverified_manual_party() {
    let db = setup_test_db().await;
    let organization_id = seed_org(&db).await;

    let party = Party::create(
        db.pool.as_ref(),
        organization_id,
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

    assert_eq!(party.organization_id, organization_id);
    assert_eq!(party.name, "Manual Vendor");
    assert_eq!(party.is_verified, 0);
    assert_eq!(party.verification_status, "unverified");
    assert!(party.verified_at.is_none());
}

#[actix_rt::test]
#[serial]
async fn creates_verified_manual_party() {
    let db = setup_test_db().await;
    let organization_id = seed_org(&db).await;

    let party = Party::create(
        db.pool.as_ref(),
        organization_id,
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

    assert_eq!(party.organization_id, organization_id);
    assert_eq!(party.is_verified, 1);
    assert_eq!(party.verification_status, "verified");
    assert_eq!(party.verification_method.as_deref(), Some("admin"));
    assert!(party.verified_at.is_some());
}

#[actix_rt::test]
#[serial]
async fn creates_verified_user_party() {
    let db = setup_test_db().await;
    let organization_id = seed_org(&db).await;
    let user_id = seed_user(&db, "ops@atlas.test", "Riley Operations", "contractor").await;

    let party =
        Party::create_verified_user_party(db.pool.as_ref(), organization_id, user_id, "contractor")
            .await
            .unwrap();

    assert_eq!(party.organization_id, organization_id);
    assert_eq!(party.name, "Riley Operations");
    assert_eq!(party.email.as_deref(), Some("ops@atlas.test"));
    assert_eq!(party.linked_user_id, Some(user_id));
    assert_eq!(party.is_verified, 1);
    assert_eq!(party.verification_status, "verified");
}

#[actix_rt::test]
#[serial]
async fn creates_verified_client_party() {
    let db = setup_test_db().await;
    let organization_id = seed_org(&db).await;
    let client_id = seed_client(&db, organization_id).await;

    let party = Party::create_verified_client_party(db.pool.as_ref(), organization_id, client_id)
        .await
        .unwrap();

    assert_eq!(party.organization_id, organization_id);
    assert_eq!(party.party_type, "client");
    assert_eq!(party.linked_client_id, Some(client_id));
    assert_eq!(party.is_verified, 1);
}

#[actix_rt::test]
#[serial]
async fn upserts_party_payment_profile_as_payee() {
    let db = setup_test_db().await;
    let organization_id = seed_org(&db).await;

    let party = Party::create(
        db.pool.as_ref(),
        organization_id,
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

    let profile = PartyPaymentProfile::upsert_basic(
        db.pool.as_ref(),
        party.id,
        organization_id,
        "payee",
        None,
    )
    .await
    .unwrap();

    assert_eq!(profile.party_id, party.id);
    assert_eq!(profile.organization_id, organization_id);
    assert_eq!(profile.payment_role, "payee");
    assert_eq!(profile.payout_status, "not_ready");
}

#[actix_rt::test]
#[serial]
async fn upserts_party_payment_profile_role_without_duplicate() {
    let db = setup_test_db().await;
    let organization_id = seed_org(&db).await;

    let party = Party::create(
        db.pool.as_ref(),
        organization_id,
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

    PartyPaymentProfile::upsert_basic(db.pool.as_ref(), party.id, organization_id, "payee", None)
        .await
        .unwrap();

    let updated = PartyPaymentProfile::upsert_basic(
        db.pool.as_ref(),
        party.id,
        organization_id,
        "both",
        Some("agreement".to_string()),
    )
    .await
    .unwrap();

    let count: i64 = sqlx::query_scalar(
        r#"
        SELECT COUNT(*)::BIGINT
        FROM party_payment_profiles
        WHERE party_id = $1
        "#,
    )
    .bind(party.id)
    .fetch_one(db.pool.as_ref())
    .await
    .unwrap();

    assert_eq!(count, 1);
    assert_eq!(updated.payment_role, "both");
    assert_eq!(updated.organization_id, organization_id);
}
