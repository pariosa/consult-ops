use backend::models::organization_invitation::OrganizationInvitation;
use backend::models::organization_member::OrganizationMember;
use chrono::{Duration, Utc};

use serial_test::serial;
mod common;

use common::setup_test_db;

async fn seed_user(db: &backend::db::Db, email: &str) -> i64 {
    sqlx::query_scalar::<_, i64>(
        r#"
        INSERT INTO users (email, password_hash, name, user_type, email_verified_at, created_at, updated_at)
        VALUES ($1, 'test-hash', $2, 'owner', NOW()::TEXT, NOW()::TEXT, NOW()::TEXT)
        RETURNING id
        "#,
    )
    .bind(email)
    .bind(email)
    .fetch_one(db.pool.as_ref())
    .await
    .unwrap()
}

async fn seed_org(db: &backend::db::Db) -> i64 {
    sqlx::query_scalar::<_, i64>(
        r#"
        INSERT INTO organizations (name, slug, created_at, updated_at)
        VALUES ('Test Org', concat('test-org-', gen_random_uuid()), NOW()::TEXT, NOW()::TEXT)
        RETURNING id
        "#,
    )
    .fetch_one(db.pool.as_ref())
    .await
    .unwrap()
}

#[actix_rt::test]
#[serial]
async fn creates_organization_invitation() {
    let db = setup_test_db().await;

    let invitation = OrganizationInvitation::create(
        db.pool.as_ref(),
        1,
        "new.contractor@example.com".to_string(),
        "contractor".to_string(),
        "token-1".to_string(),
        Some(1),
        (Utc::now() + Duration::days(7)).to_rfc3339(),
    )
    .await
    .unwrap();

    assert_eq!(invitation.organization_id, 1);
    assert_eq!(invitation.email, "new.contractor@example.com");
    assert_eq!(invitation.role, "contractor");
    assert_eq!(invitation.status, "pending");
    assert_eq!(invitation.invited_by_user_id, Some(1));
}

#[actix_rt::test]
#[serial]
async fn duplicate_pending_invite_returns_existing_invitation() {
    let db = setup_test_db().await;

    let first = OrganizationInvitation::create(
        db.pool.as_ref(),
        1,
        "new.contractor@example.com".to_string(),
        "contractor".to_string(),
        "token-1".to_string(),
        Some(1),
        (Utc::now() + Duration::days(7)).to_rfc3339(),
    )
    .await
    .unwrap();

    let second = OrganizationInvitation::create(
        db.pool.as_ref(),
        1,
        "NEW.CONTRACTOR@example.com".to_string(),
        "finance_admin".to_string(),
        "token-2".to_string(),
        Some(1),
        (Utc::now() + Duration::days(7)).to_rfc3339(),
    )
    .await
    .unwrap();

    assert_eq!(first.id, second.id);
    assert_eq!(second.email, "new.contractor@example.com");
    assert_eq!(second.role, "contractor");
}

#[actix_rt::test]
#[serial]
async fn finds_pending_invitation_by_token() {
    let db = setup_test_db().await;

    OrganizationInvitation::create(
        db.pool.as_ref(),
        1,
        "member@example.com".to_string(),
        "member".to_string(),
        "find-me-token".to_string(),
        Some(1),
        (Utc::now() + Duration::days(7)).to_rfc3339(),
    )
    .await
    .unwrap();

    let found = OrganizationInvitation::find_pending_by_token(db.pool.as_ref(), "find-me-token")
        .await
        .unwrap();

    assert_eq!(found.email, "member@example.com");
    assert_eq!(found.status, "pending");
}

#[actix_rt::test]
#[serial]
async fn upserts_active_organization_member() {
    let db = setup_test_db().await;

    let organization_id = seed_org(&db).await;
    let user_id = seed_user(&db, "member@example.com").await;

    let first = OrganizationMember::upsert_active_member(
        db.pool.as_ref(),
        organization_id,
        user_id,
        "contractor",
    )
    .await
    .unwrap();

    assert_eq!(first.organization_id, organization_id);
    assert_eq!(first.user_id, user_id);
    assert_eq!(first.role, "contractor");
    assert_eq!(first.status, "active");

    let updated = OrganizationMember::upsert_active_member(
        db.pool.as_ref(),
        organization_id,
        user_id,
        "finance_admin",
    )
    .await
    .unwrap();

    assert_eq!(first.id, updated.id);
    assert_eq!(updated.role, "finance_admin");

    let count: i64 = sqlx::query_scalar(
        r#"
        SELECT COUNT(*)::BIGINT
        FROM organization_members
        WHERE organization_id = $1
          AND user_id = $2
        "#,
    )
    .bind(organization_id)
    .bind(user_id)
    .fetch_one(db.pool.as_ref())
    .await
    .unwrap();

    assert_eq!(count, 1);
}

#[actix_rt::test]
#[serial]
async fn lists_organization_members() {
    let db = setup_test_db().await;
    let organization_id = seed_org(&db).await;
    let user_id_1 = seed_user(&db, "member-one@example.com").await;
    let user_id_2 = seed_user(&db, "member-two@example.com").await;

    OrganizationMember::upsert_active_member(
        db.pool.as_ref(),
        organization_id,
        user_id_1,
        "contractor",
    )
    .await
    .unwrap();

    OrganizationMember::upsert_active_member(
        db.pool.as_ref(),
        organization_id,
        user_id_2,
        "finance_admin",
    )
    .await
    .unwrap();

    let members = OrganizationMember::list_for_organization(db.pool.as_ref(), organization_id)
        .await
        .unwrap();

    assert_eq!(members.len(), 2);
}
