use backend::domain::engagement_state::{EngagementEvent, EngagementStatus};
use backend::services::event_service::EventService;
use backend::services::operations_kernel_service::OperationsKernelService;
use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};

async fn setup_test_db() -> SqlitePool {
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await
        .expect("failed to create sqlite memory db");

    sqlx::query(
        r#"
        CREATE TABLE engagements (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            organization_id INTEGER NOT NULL,
            project_id INTEGER NOT NULL,
            contractor_name TEXT NOT NULL,
            contractor_email TEXT NOT NULL,
            role TEXT NOT NULL,
            title TEXT NOT NULL,
            scope_of_work TEXT NOT NULL,
            deliverables TEXT,
            repo_url TEXT,
            amount_cents INTEGER NOT NULL,
            currency TEXT NOT NULL DEFAULT 'usd',
            due_date TEXT,
            status TEXT NOT NULL DEFAULT 'draft',
            platform_fee_status TEXT NOT NULL DEFAULT 'pending',
            contract_id INTEGER,
            invoice_id INTEGER,
            payment_id INTEGER,
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
        CREATE TABLE engagement_billing (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            engagement_id INTEGER NOT NULL,
            organization_id INTEGER NOT NULL,
            billing_type TEXT NOT NULL,
            amount_cents INTEGER NOT NULL,
            currency TEXT NOT NULL DEFAULT 'usd',
            status TEXT NOT NULL DEFAULT 'pending',
            stripe_checkout_session_id TEXT,
            stripe_payment_intent_id TEXT,
            paid_at TEXT,
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
        CREATE TABLE operational_events (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            organization_id INTEGER NOT NULL,
            actor_user_id INTEGER NULL,
            entity_type TEXT NOT NULL,
            entity_id INTEGER NOT NULL,
            event_type TEXT NOT NULL,
            from_status TEXT NULL,
            to_status TEXT NULL,
            metadata TEXT NOT NULL DEFAULT '{}',
            created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
        );
        "#,
    )
    .execute(&pool)
    .await
    .unwrap();

    pool
}

#[actix_rt::test]
async fn payment_received_activates_engagement_and_records_event() {
    let pool = setup_test_db().await;

    let engagement_id: i64 = sqlx::query_scalar(
        r#"
        INSERT INTO engagements (
            organization_id,
            project_id,
            contractor_name,
            contractor_email,
            role,
            title,
            scope_of_work,
            amount_cents,
            currency,
            status
        )
        VALUES (
            1,
            1,
            'Test Contractor',
            'contractor@example.com',
            'Developer',
            'Test Engagement',
            'Build the workflow',
            100000,
            'usd',
            'awaiting_payment'
        )
        RETURNING id
        "#,
    )
    .fetch_one(&pool)
    .await
    .unwrap();

    let next_status = OperationsKernelService::apply_engagement_event(
        &pool,
        1,
        engagement_id,
        None,
        EngagementStatus::AwaitingPayment,
        EngagementEvent::PaymentReceived,
    )
    .await
    .unwrap();

    assert_eq!(next_status, EngagementStatus::Active);

    sqlx::query(
        r#"
        UPDATE engagements
        SET status = ?
        WHERE id = ?
        "#,
    )
    .bind("active")
    .bind(engagement_id)
    .execute(&pool)
    .await
    .unwrap();

    let saved_status: String = sqlx::query_scalar(
        r#"
        SELECT status
        FROM engagements
        WHERE id = ?
        "#,
    )
    .bind(engagement_id)
    .fetch_one(&pool)
    .await
    .unwrap();

    assert_eq!(saved_status, "active");

    let event_count: i64 = sqlx::query_scalar(
        r#"
        SELECT COUNT(*)
        FROM operational_events
        WHERE entity_type = 'engagement'
          AND entity_id = ?
        "#,
    )
    .bind(engagement_id)
    .fetch_one(&pool)
    .await
    .unwrap();

    assert!(event_count >= 1);
}

#[actix_rt::test]
async fn cannot_activate_engagement_from_draft() {
    let pool = setup_test_db().await;

    let result = OperationsKernelService::apply_engagement_event(
        &pool,
        1,
        999,
        None,
        EngagementStatus::Draft,
        EngagementEvent::PaymentReceived,
    )
    .await;

    assert!(result.is_err());
}
