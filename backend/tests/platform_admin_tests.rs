mod common;

use actix_web::test;
use backend::auth::hash_password;
use chrono::Utc;
use common::{setup_test_db, test_app};
use serial_test::serial;

async fn seed_verified_user(
    db: &backend::db::Db,
    email: Option<&str>,
    password: &str,
    user_type: &str,
) -> (i64, String) {
    let now = chrono::Utc::now().to_rfc3339();

    let email = email.map(|s| s.to_string()).unwrap_or_else(|| {
        format!(
            "verified-{}@example.com",
            chrono::Utc::now().timestamp_nanos_opt().unwrap_or_default()
        )
    });

    let password_hash = backend::auth::hash_password(password).unwrap();

    let user_id = sqlx::query_scalar::<_, i64>(
        r#"
        INSERT INTO users
        (
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
    .bind(&email)
    .bind(password_hash)
    .bind(&email)
    .bind(user_type)
    .bind(&now)
    .bind(&now)
    .bind(&now)
    .fetch_one(db.pool.as_ref())
    .await
    .unwrap();

    (user_id, email)
}

async fn seed_organization(db: &backend::db::Db, name: &str) -> i64 {
    let now = Utc::now().to_rfc3339();
    let slug = format!(
        "{}-{}",
        name.to_lowercase().replace(' ', "-"),
        Utc::now().timestamp_nanos_opt().unwrap_or_default()
    );

    sqlx::query_scalar::<_, i64>(
        r#"
        INSERT INTO organizations (name, slug, created_at, updated_at)
        VALUES ($1, $2, $3, $4)
        RETURNING id
        "#,
    )
    .bind(name)
    .bind(slug)
    .bind(&now)
    .bind(&now)
    .fetch_one(db.pool.as_ref())
    .await
    .unwrap()
}
macro_rules! login_and_get_token {
    ($app:expr, $email:expr) => {{
        let login_req = test::TestRequest::post()
            .uri("/api/auth/login")
            .set_json(serde_json::json!({
                "email": $email,
                "password": "DemoPass123!"
            }))
            .to_request();

        let login_resp: serde_json::Value =
            test::call_and_read_body_json(&$app, login_req).await;

        login_resp["token"].as_str().unwrap().to_string()
    }};
}
#[actix_web::test]
#[serial]
async fn super_admin_can_create_and_list_platform_organizations() {
    let db = setup_test_db().await;

    seed_verified_user(&db, Some("root@atlas.test"), "DemoPass123!", "super_admin").await;

    let app = test::init_service(test_app(db.clone())).await;
    let token = login_and_get_token!(&app, "root@atlas.test");

    let create_req = test::TestRequest::post()
        .uri("/api/platform/organizations")
        .insert_header(("Authorization", format!("Bearer {}", token.clone())))
        .set_json(serde_json::json!({
            "name": "Atlas Operations"
        }))
        .to_request();

    let create_resp = test::call_service(&app, create_req).await;
    assert_eq!(create_resp.status().as_u16(), 201);

    let list_req = test::TestRequest::get()
        .uri("/api/platform/organizations")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();

    let orgs: serde_json::Value = test::call_and_read_body_json(&app, list_req).await;

    assert!(
        orgs.as_array()
            .unwrap()
            .iter()
            .any(|org| { org["name"] == "Atlas Operations" })
    );
}

#[actix_web::test]
#[serial]
async fn non_super_admin_cannot_create_platform_organization() {
    let db = setup_test_db().await;

    seed_verified_user(&db, Some("admin@atlas.test"), "DemoPass123!", "admin").await;

    let app = test::init_service(test_app(db.clone())).await;
    let token = login_and_get_token!(&app, "admin@atlas.test");

    let create_req = test::TestRequest::post()
        .uri("/api/platform/organizations")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(serde_json::json!({
            "name": "Blocked Org"
        }))
        .to_request();

    let resp = test::call_service(&app, create_req).await;
    assert_eq!(resp.status().as_u16(), 403);
}

#[actix_web::test]
#[serial]
async fn super_admin_can_create_and_list_platform_users() {
    let db = setup_test_db().await;

    seed_verified_user(
        &db,
        Some("root-users@atlas.test"),
        "DemoPass123!",
        "super_admin",
    )
    .await;

    let app = test::init_service(test_app(db.clone())).await;
    let token = login_and_get_token!(&app, "root-users@atlas.test");

    let create_req = test::TestRequest::post()
        .uri("/api/platform/users")
        .insert_header(("Authorization", format!("Bearer {}", token.clone())))
        .set_json(serde_json::json!({
            "email": "new-user@atlas.test",
            "name": "New User",
            "user_type": "consultant",
            "password": "DemoPass123!"
        }))
        .to_request();

    let create_resp = test::call_service(&app, create_req).await;
    assert_eq!(create_resp.status().as_u16(), 201);

    let list_req = test::TestRequest::get()
        .uri("/api/platform/users")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();

    let users: serde_json::Value = test::call_and_read_body_json(&app, list_req).await;

    assert!(
        users
            .as_array()
            .unwrap()
            .iter()
            .any(|user| { user["email"] == "new-user@atlas.test" })
    );
}

#[actix_web::test]
#[serial]
async fn super_admin_can_assign_user_to_organization() {
    let db = setup_test_db().await;

    seed_verified_user(
        &db,
        Some("root-assign@atlas.test"),
        "DemoPass123!",
        "super_admin",
    )
    .await;

    let (user_id, _) =
        seed_verified_user(&db, Some("member@atlas.test"), "DemoPass123!", "consultant").await;

    let organization_id = seed_organization(&db, "Atlas Client Org").await;

    let app = test::init_service(test_app(db.clone())).await;
    let token = login_and_get_token!(&app, "root-assign@atlas.test");

    let assign_req = test::TestRequest::post()
        .uri(&format!(
            "/api/platform/organizations/{}/members",
            organization_id
        ))
        .insert_header(("Authorization", format!("Bearer {}", token.clone())))
        .set_json(serde_json::json!({
            "user_id": user_id,
            "role": "operations_manager"
        }))
        .to_request();

    let assign_resp = test::call_service(&app, assign_req).await;
    assert_eq!(assign_resp.status().as_u16(), 200);

    let list_req = test::TestRequest::get()
        .uri(&format!(
            "/api/platform/organizations/{}/members",
            organization_id
        ))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();

    let members: serde_json::Value = test::call_and_read_body_json(&app, list_req).await;

    assert_eq!(members.as_array().unwrap().len(), 1);
    assert_eq!(members[0]["user_id"].as_i64(), Some(user_id));
    assert_eq!(members[0]["role"], "operations_manager");
    assert_eq!(members[0]["status"], "active");
}

#[actix_web::test]
#[serial]
async fn assigning_same_user_updates_existing_membership_role() {
    let db = setup_test_db().await;

    seed_verified_user(
        &db,
        Some("root-update@atlas.test"),
        "DemoPass123!",
        "super_admin",
    )
    .await;

    let (user_id, _) = seed_verified_user(
        &db,
        Some("member-update@atlas.test"),
        "DemoPass123!",
        "consultant",
    )
    .await;

    let organization_id = seed_organization(&db, "Atlas Client Org Update").await;

    let app = test::init_service(test_app(db.clone())).await;
    let token = login_and_get_token!(&app, "root-update@atlas.test");

    for role in ["operations_manager", "finance_admin"] {
        let req = test::TestRequest::post()
            .uri(&format!(
                "/api/platform/organizations/{}/members",
                organization_id
            ))
            .insert_header(("Authorization", format!("Bearer {}", token.clone())))
            .set_json(serde_json::json!({
                "user_id": user_id,
                "role": role
            }))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status().as_u16(), 200);
    }

    let list_req = test::TestRequest::get()
        .uri(&format!(
            "/api/platform/organizations/{}/members",
            organization_id
        ))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();

    let members: serde_json::Value = test::call_and_read_body_json(&app, list_req).await;

    assert_eq!(members.as_array().unwrap().len(), 1);
    assert_eq!(members[0]["user_id"].as_i64(), Some(user_id));
    assert_eq!(members[0]["role"], "finance_admin");
    assert_eq!(members[0]["status"], "active");
}
