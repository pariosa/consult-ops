use backend::models::organization_invitation::OrganizationInvitation;
use backend::models::organization_member::OrganizationMember;
use chrono::{Duration, Utc};
use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};

async fn setup_invitation_test_db() -> SqlitePool {
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await
        .expect("failed to create sqlite memory db");

    sqlx::query(
        r#"
        CREATE TABLE organization_invitations (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            organization_id INTEGER NOT NULL,
            email TEXT NOT NULL,
            role TEXT NOT NULL DEFAULT 'member',
            token TEXT NOT NULL UNIQUE,
            status TEXT NOT NULL DEFAULT 'pending',
            invited_by_user_id INTEGER,
            accepted_by_user_id INTEGER,
            expires_at TEXT NOT NULL,
            accepted_at TEXT,
            created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
            UNIQUE(organization_id, email, status)
        );
        "#,
    )
    .execute(&pool)
    .await
    .unwrap();

    sqlx::query(
        r#"
        CREATE TABLE organization_members (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            organization_id INTEGER NOT NULL,
            user_id INTEGER NOT NULL,
            role TEXT NOT NULL DEFAULT 'member',
            status TEXT NOT NULL DEFAULT 'active',
            created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
            UNIQUE(organization_id, user_id)
        );
        "#,
    )
    .execute(&pool)
    .await
    .unwrap();

    pool
}

#[actix_rt::test]
async fn creates_organization_invitation() {
    let pool = setup_invitation_test_db().await;

    let invitation = OrganizationInvitation::create(
        &pool,
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
async fn duplicate_pending_invite_returns_existing_invitation() {
    let pool = setup_invitation_test_db().await;

    let first = OrganizationInvitation::create(
        &pool,
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
        &pool,
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

    let count: i32 = sqlx::query_scalar(
        r#"
        SELECT COUNT(*)
        FROM organization_invitations
        WHERE organization_id = 1
        "#,
    )
    .fetch_one(&pool)
    .await
    .unwrap();

    assert_eq!(count, 1);
}

#[actix_rt::test]
async fn finds_pending_invitation_by_token() {
    let pool = setup_invitation_test_db().await;

    OrganizationInvitation::create(
        &pool,
        1,
        "member@example.com".to_string(),
        "member".to_string(),
        "find-me-token".to_string(),
        Some(1),
        (Utc::now() + Duration::days(7)).to_rfc3339(),
    )
    .await
    .unwrap();

    let found = OrganizationInvitation::find_pending_by_token(&pool, "find-me-token")
        .await
        .unwrap();

    assert_eq!(found.email, "member@example.com");
    assert_eq!(found.status, "pending");
}

#[actix_rt::test]
async fn marks_invitation_accepted() {
    let pool = setup_invitation_test_db().await;

    let invitation = OrganizationInvitation::create(
        &pool,
        1,
        "member@example.com".to_string(),
        "member".to_string(),
        "accept-token".to_string(),
        Some(1),
        (Utc::now() + Duration::days(7)).to_rfc3339(),
    )
    .await
    .unwrap();

    let accepted = OrganizationInvitation::mark_accepted(&pool, invitation.id, 2)
        .await
        .unwrap();

    assert_eq!(accepted.status, "accepted");
    assert_eq!(accepted.accepted_by_user_id, Some(2));
    assert!(accepted.accepted_at.is_some());
}

#[actix_rt::test]
async fn upserts_active_organization_member() {
    let pool = setup_invitation_test_db().await;

    let first = OrganizationMember::upsert_active_member(&pool, 1, 2, "contractor")
        .await
        .unwrap();

    assert_eq!(first.organization_id, 1);
    assert_eq!(first.user_id, 2);
    assert_eq!(first.role, "contractor");
    assert_eq!(first.status, "active");

    let updated = OrganizationMember::upsert_active_member(&pool, 1, 2, "finance_admin")
        .await
        .unwrap();

    assert_eq!(first.id, updated.id);
    assert_eq!(updated.role, "finance_admin");

    let count: i32 = sqlx::query_scalar(
        r#"
        SELECT COUNT(*)
        FROM organization_members
        WHERE organization_id = 1
          AND user_id = 2
        "#,
    )
    .fetch_one(&pool)
    .await
    .unwrap();

    assert_eq!(count, 1);
}

#[actix_rt::test]
async fn lists_organization_members() {
    let pool = setup_invitation_test_db().await;

    OrganizationMember::upsert_active_member(&pool, 1, 2, "contractor")
        .await
        .unwrap();

    OrganizationMember::upsert_active_member(&pool, 1, 3, "finance_admin")
        .await
        .unwrap();

    let members = OrganizationMember::list_for_organization(&pool, 1)
        .await
        .unwrap();

    assert_eq!(members.len(), 2);
}
