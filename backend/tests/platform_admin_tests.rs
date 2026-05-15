use actix_web::{App, test, web};
use backend::auth::{Claims, hash_password};
use backend::db::Db;
use backend::handlers::{
    assign_platform_user_to_organization, create_platform_organization, create_platform_user,
    list_platform_organization_members, list_platform_organizations, list_platform_users,
};
use chrono::{Duration, Utc};
use jsonwebtoken::{EncodingKey, Header, encode};
use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};

async fn setup_db() -> Db {
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await
        .unwrap();

    sqlx::query(
        r#"
        CREATE TABLE users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            email TEXT NOT NULL UNIQUE,
            password_hash TEXT NOT NULL,
            name TEXT,
            user_type TEXT NOT NULL DEFAULT 'client',
            created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
        );
        "#,
    )
    .execute(&pool)
    .await
    .unwrap();

    sqlx::query(
        r#"
        CREATE TABLE organizations (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
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

    seed_user(
        &pool,
        "superadmin@consultops.test",
        "Super Admin",
        "super_admin",
    )
    .await;
    seed_user(&pool, "admin@atlas.test", "Avery Atlas", "admin").await;

    Db { pool: pool.into() }
}

async fn seed_user(pool: &SqlitePool, email: &str, name: &str, user_type: &str) -> i64 {
    let password_hash = hash_password("DemoPass123!").unwrap();

    let id: i64 = sqlx::query_scalar(
        r#"
        INSERT INTO users (email, password_hash, name, user_type, created_at, updated_at)
        VALUES (?, ?, ?, ?, datetime('now'), datetime('now'))
        RETURNING id
        "#,
    )
    .bind(email)
    .bind(password_hash)
    .bind(name)
    .bind(user_type)
    .fetch_one(pool)
    .await
    .unwrap();

    id
}

fn token_for(user_id: i64, email: &str, user_type: &str) -> String {
    let claims = Claims {
        sub: user_id.to_string(),
        email: email.to_string(),
        user_type: user_type.to_string(),
        exp: (Utc::now() + Duration::hours(1)).timestamp() as usize,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret("consult-ops-local-dev-secret".as_bytes()),
    )
    .unwrap()
}

fn auth_header(token: &str) -> (&'static str, String) {
    ("Authorization", format!("Bearer {}", token))
}

#[actix_rt::test]
async fn super_admin_can_create_and_list_platform_organizations() {
    let db = setup_db().await;
    let token = token_for(1, "superadmin@consultops.test", "super_admin");

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .route(
                "/api/platform/organizations",
                web::post().to(create_platform_organization),
            )
            .route(
                "/api/platform/organizations",
                web::get().to(list_platform_organizations),
            ),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/api/platform/organizations")
        .insert_header(auth_header(&token))
        .set_json(serde_json::json!({
            "name": "New Platform Org"
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 201);

    let req = test::TestRequest::get()
        .uri("/api/platform/organizations")
        .insert_header(auth_header(&token))
        .to_request();

    let body: serde_json::Value = test::call_and_read_body_json(&app, req).await;

    assert!(
        body.as_array()
            .unwrap()
            .iter()
            .any(|org| { org.get("name").and_then(|v| v.as_str()) == Some("New Platform Org") })
    );
}

#[actix_rt::test]
async fn non_super_admin_cannot_create_platform_organization() {
    let db = setup_db().await;
    let token = token_for(2, "admin@atlas.test", "admin");

    let app = test::init_service(App::new().app_data(web::Data::new(db)).route(
        "/api/platform/organizations",
        web::post().to(create_platform_organization),
    ))
    .await;

    let req = test::TestRequest::post()
        .uri("/api/platform/organizations")
        .insert_header(auth_header(&token))
        .set_json(serde_json::json!({
            "name": "Blocked Org"
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 403);
}

#[actix_rt::test]
async fn super_admin_can_create_and_list_platform_users() {
    let db = setup_db().await;
    let token = token_for(1, "superadmin@consultops.test", "super_admin");

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .route("/api/platform/users", web::post().to(create_platform_user))
            .route("/api/platform/users", web::get().to(list_platform_users)),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/api/platform/users")
        .insert_header(auth_header(&token))
        .set_json(serde_json::json!({
            "email": "new.user@example.com",
            "name": "New User",
            "user_type": "contractor",
            "password": "DemoPass123!"
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 201);

    let req = test::TestRequest::get()
        .uri("/api/platform/users")
        .insert_header(auth_header(&token))
        .to_request();

    let body: serde_json::Value = test::call_and_read_body_json(&app, req).await;

    assert!(body.as_array().unwrap().iter().any(|user| {
        user.get("email").and_then(|v| v.as_str()) == Some("new.user@example.com")
    }));
}

#[actix_rt::test]
async fn super_admin_can_assign_user_to_organization() {
    let db = setup_db().await;
    let token = token_for(1, "superadmin@consultops.test", "super_admin");

    let org_id: i64 = sqlx::query_scalar(
        r#"
        INSERT INTO organizations (name, created_at, updated_at)
        VALUES ('Atlas Field Consulting', datetime('now'), datetime('now'))
        RETURNING id
        "#,
    )
    .fetch_one(db.pool.as_ref())
    .await
    .unwrap();

    let contractor_id = seed_user(
        db.pool.as_ref(),
        "contractor@example.com",
        "Contractor User",
        "contractor",
    )
    .await;

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .route(
                "/api/platform/organizations/{id}/members",
                web::post().to(assign_platform_user_to_organization),
            )
            .route(
                "/api/platform/organizations/{id}/members",
                web::get().to(list_platform_organization_members),
            ),
    )
    .await;

    let req = test::TestRequest::post()
        .uri(&format!("/api/platform/organizations/{}/members", org_id))
        .insert_header(auth_header(&token))
        .set_json(serde_json::json!({
            "user_id": contractor_id,
            "role": "contractor"
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);

    let req = test::TestRequest::get()
        .uri(&format!("/api/platform/organizations/{}/members", org_id))
        .insert_header(auth_header(&token))
        .to_request();

    let body: serde_json::Value = test::call_and_read_body_json(&app, req).await;

    let members = body.as_array().unwrap();

    assert!(members.iter().any(|member| {
        member.get("email").and_then(|v| v.as_str()) == Some("contractor@example.com")
            && member.get("role").and_then(|v| v.as_str()) == Some("contractor")
    }));
}

#[actix_rt::test]
async fn assigning_same_user_updates_existing_membership_role() {
    let db = setup_db().await;
    let pool = db.pool.clone();
    let token = token_for(1, "superadmin@consultops.test", "super_admin");

    let org_id: i64 = sqlx::query_scalar(
        r#"
        INSERT INTO organizations (name, created_at, updated_at)
        VALUES ('Atlas Field Consulting', datetime('now'), datetime('now'))
        RETURNING id
        "#,
    )
    .fetch_one(db.pool.as_ref())
    .await
    .unwrap();

    let user_id = seed_user(
        db.pool.as_ref(),
        "ops@example.com",
        "Ops User",
        "operations_manager",
    )
    .await;

    let app = test::init_service(App::new().app_data(web::Data::new(db)).route(
        "/api/platform/organizations/{id}/members",
        web::post().to(assign_platform_user_to_organization),
    ))
    .await;

    for role in ["contractor", "finance_admin"] {
        let req = test::TestRequest::post()
            .uri(&format!("/api/platform/organizations/{}/members", org_id))
            .insert_header(auth_header(&token))
            .set_json(serde_json::json!({
                "user_id": user_id,
                "role": role
            }))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 200);
    }

    let count: i64 = sqlx::query_scalar::<_, i64>(
        r#"
    SELECT COUNT(*)
    FROM organization_members
    WHERE organization_id = ?
      AND user_id = ?
    "#,
    )
    .bind(org_id)
    .bind(user_id)
    .fetch_one(pool.as_ref())
    .await
    .unwrap();

    assert_eq!(count, 1);
}
