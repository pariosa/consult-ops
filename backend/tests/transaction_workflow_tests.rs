use backend::services::transaction_workflow_service::TransactionWorkflowService;
use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};

async fn setup_transaction_test_db() -> SqlitePool {
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await
        .expect("failed to create sqlite memory db");

    sqlx::query(
        r#"
        CREATE TABLE agreement_payout_rules (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            agreement_id INTEGER NOT NULL,
            from_party_id INTEGER NOT NULL,
            to_party_id INTEGER NOT NULL,
            rule_type TEXT NOT NULL,
            percent INTEGER,
            amount_cents INTEGER,
            trigger_event TEXT NOT NULL,
            created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
        );
        "#,
    )
    .execute(&pool)
    .await
    .unwrap();

    sqlx::query(
        r#"
        CREATE TABLE operational_transactions (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            organization_id INTEGER NOT NULL,
            agreement_id INTEGER,
            engagement_id INTEGER,
            milestone_id INTEGER,
            from_party_id INTEGER NOT NULL,
            to_party_id INTEGER NOT NULL,
            transaction_type TEXT NOT NULL,
            amount_cents INTEGER NOT NULL,
            currency TEXT NOT NULL DEFAULT 'usd',
            status TEXT NOT NULL DEFAULT 'pending',
            trigger_event TEXT,
            created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
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
async fn milestone_approved_generates_percentage_transaction() {
    let pool = setup_transaction_test_db().await;

    sqlx::query(
        r#"
        INSERT INTO agreement_payout_rules (
            agreement_id,
            from_party_id,
            to_party_id,
            rule_type,
            percent,
            amount_cents,
            trigger_event
        )
        VALUES (1, 10, 20, 'contractor_payout', 100, NULL, 'MilestoneApproved')
        "#,
    )
    .execute(&pool)
    .await
    .unwrap();

    let created = TransactionWorkflowService::generate_transactions_for_trigger(
        &pool,
        1,
        1,
        Some(100),
        Some(200),
        "MilestoneApproved",
        250000,
    )
    .await
    .unwrap();

    assert_eq!(created.len(), 1);
    assert_eq!(created[0].amount_cents, 250000);
    assert_eq!(created[0].transaction_type, "contractor_payout");
    assert_eq!(created[0].from_party_id, 10);
    assert_eq!(created[0].to_party_id, 20);

    let count: i32 = sqlx::query_scalar(
        r#"
        SELECT COUNT(*)
        FROM operational_transactions
        WHERE engagement_id = 100
          AND milestone_id = 200
          AND trigger_event = 'MilestoneApproved'
        "#,
    )
    .fetch_one(&pool)
    .await
    .unwrap();

    assert_eq!(count, 1);
}

#[actix_rt::test]
async fn milestone_approved_generates_split_transaction() {
    let pool = setup_transaction_test_db().await;

    sqlx::query(
        r#"
        INSERT INTO agreement_payout_rules (
            agreement_id,
            from_party_id,
            to_party_id,
            rule_type,
            percent,
            amount_cents,
            trigger_event
        )
        VALUES (1, 10, 30, 'subcontractor_payout', 30, NULL, 'MilestoneApproved')
        "#,
    )
    .execute(&pool)
    .await
    .unwrap();

    let created = TransactionWorkflowService::generate_transactions_for_trigger(
        &pool,
        1,
        1,
        Some(100),
        Some(200),
        "MilestoneApproved",
        250000,
    )
    .await
    .unwrap();

    assert_eq!(created.len(), 1);
    assert_eq!(created[0].amount_cents, 75000);
    assert_eq!(created[0].transaction_type, "subcontractor_payout");
}

#[actix_rt::test]
async fn fixed_amount_rule_generates_fixed_transaction() {
    let pool = setup_transaction_test_db().await;

    sqlx::query(
        r#"
        INSERT INTO agreement_payout_rules (
            agreement_id,
            from_party_id,
            to_party_id,
            rule_type,
            percent,
            amount_cents,
            trigger_event
        )
        VALUES (1, 10, 40, 'dividend', NULL, 5000, 'MilestoneApproved')
        "#,
    )
    .execute(&pool)
    .await
    .unwrap();

    let created = TransactionWorkflowService::generate_transactions_for_trigger(
        &pool,
        1,
        1,
        Some(100),
        Some(200),
        "MilestoneApproved",
        250000,
    )
    .await
    .unwrap();

    assert_eq!(created.len(), 1);
    assert_eq!(created[0].amount_cents, 5000);
    assert_eq!(created[0].transaction_type, "dividend");
}

#[actix_rt::test]
async fn different_trigger_generates_no_transaction() {
    let pool = setup_transaction_test_db().await;

    sqlx::query(
        r#"
        INSERT INTO agreement_payout_rules (
            agreement_id,
            from_party_id,
            to_party_id,
            rule_type,
            percent,
            amount_cents,
            trigger_event
        )
        VALUES (1, 10, 20, 'contractor_payout', 100, NULL, 'MilestoneApproved')
        "#,
    )
    .execute(&pool)
    .await
    .unwrap();

    let created = TransactionWorkflowService::generate_transactions_for_trigger(
        &pool,
        1,
        1,
        Some(100),
        Some(200),
        "EngagementCompleted",
        250000,
    )
    .await
    .unwrap();

    assert_eq!(created.len(), 0);
}

#[actix_rt::test]
async fn transaction_generation_is_idempotent() {
    let pool = setup_transaction_test_db().await;

    sqlx::query(
        r#"
        INSERT INTO agreement_payout_rules (
            agreement_id,
            from_party_id,
            to_party_id,
            rule_type,
            percent,
            amount_cents,
            trigger_event
        )
        VALUES (1, 10, 20, 'contractor_payout', 100, NULL, 'MilestoneApproved')
        "#,
    )
    .execute(&pool)
    .await
    .unwrap();

    let first = TransactionWorkflowService::generate_transactions_for_trigger(
        &pool,
        1,
        1,
        Some(100),
        Some(200),
        "MilestoneApproved",
        250000,
    )
    .await
    .unwrap();

    let second = TransactionWorkflowService::generate_transactions_for_trigger(
        &pool,
        1,
        1,
        Some(100),
        Some(200),
        "MilestoneApproved",
        250000,
    )
    .await
    .unwrap();

    assert_eq!(first.len(), 1);
    assert_eq!(second.len(), 0);

    let count: i32 = sqlx::query_scalar(
        r#"
        SELECT COUNT(*)
        FROM operational_transactions
        WHERE agreement_id = 1
          AND engagement_id = 100
          AND milestone_id = 200
          AND transaction_type = 'contractor_payout'
          AND trigger_event = 'MilestoneApproved'
        "#,
    )
    .fetch_one(&pool)
    .await
    .unwrap();

    assert_eq!(count, 1);
}
