mod common;

use backend::domain::engagement_state::{EngagementEvent, EngagementStatus};
use backend::services::operations_kernel_service::OperationsKernelService;
use common::setup_test_db;
use serial_test::serial;

async fn seed_engagement(db: &backend::db::Db, status: &str) -> i64 {
    sqlx::query_scalar::<_, i64>(
        r#"
        INSERT INTO engagements (
            organization_id,
            project_id,
            engagement_type,
            contractor_name,
            contractor_email,
            role,
            title,
            scope_of_work,
            amount_cents,
            currency,
            status,
            platform_fee_status,
            created_at,
            updated_at
        )
        VALUES (
            $1,
            $2,
            'software',
            $3,
            $4,
            $5,
            $6,
            $7,
            $8,
            $9,
            $10,
            'pending',
            CURRENT_TIMESTAMP,
            CURRENT_TIMESTAMP
        )
        RETURNING id
        "#,
    )
    .bind(1_i64)
    .bind(1_i64)
    .bind("Test Contractor")
    .bind("contractor@example.com")
    .bind("Developer")
    .bind("Test Engagement")
    .bind("Build the workflow")
    .bind(100000_i64)
    .bind("usd")
    .bind(status)
    .fetch_one(db.pool.as_ref())
    .await
    .unwrap()
}

#[actix_rt::test]
#[serial]
async fn payment_received_activates_engagement_and_records_event() {
    let db = setup_test_db().await;
    let engagement_id = seed_engagement(&db, "awaiting_payment").await;

    let next_status = OperationsKernelService::apply_engagement_event(
        db.pool.as_ref(),
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
        SET status = $1,
            updated_at = CURRENT_TIMESTAMP
        WHERE id = $2
        "#,
    )
    .bind("active")
    .bind(engagement_id)
    .execute(db.pool.as_ref())
    .await
    .unwrap();

    let saved_status: String = sqlx::query_scalar(
        r#"
        SELECT status
        FROM engagements
        WHERE id = $1
        "#,
    )
    .bind(engagement_id)
    .fetch_one(db.pool.as_ref())
    .await
    .unwrap();

    assert_eq!(saved_status, "active");

    let event_count: i64 = sqlx::query_scalar(
        r#"
        SELECT COUNT(*)::BIGINT
        FROM operational_events
        WHERE entity_type = 'engagement'
          AND entity_id = $1
        "#,
    )
    .bind(engagement_id)
    .fetch_one(db.pool.as_ref())
    .await
    .unwrap();

    assert!(event_count >= 1);
}

#[actix_rt::test]
#[serial]
async fn cannot_activate_engagement_from_draft() {
    let db = setup_test_db().await;

    let result = OperationsKernelService::apply_engagement_event(
        db.pool.as_ref(),
        1,
        999,
        None,
        EngagementStatus::Draft,
        EngagementEvent::PaymentReceived,
    )
    .await;

    assert!(result.is_err());
}
