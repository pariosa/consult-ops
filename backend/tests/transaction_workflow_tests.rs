mod common;

use backend::services::transaction_workflow_service::TransactionWorkflowService;
use common::setup_test_db;
use serial_test::serial;

async fn seed_payout_rule(
    db: &backend::db::Db,
    rule_type: &str,
    percent: Option<i64>,
    amount_cents: Option<i64>,
    to_party_id: i64,
    trigger_event: &str,
) {
    sqlx::query(
        r#"
        INSERT INTO agreement_payout_rules (
            agreement_id,
            from_party_id,
            to_party_id,
            rule_type,
            percent,
            amount_cents,
            trigger_event,
            created_at
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, CURRENT_TIMESTAMP)
        "#,
    )
    .bind(1_i64)
    .bind(10_i64)
    .bind(to_party_id)
    .bind(rule_type)
    .bind(percent)
    .bind(amount_cents)
    .bind(trigger_event)
    .execute(db.pool.as_ref())
    .await
    .unwrap();
}

#[actix_rt::test]
#[serial]
async fn milestone_approved_generates_percentage_transaction() {
    let db = setup_test_db().await;

    seed_payout_rule(
        &db,
        "contractor_payout",
        Some(100),
        None,
        20,
        "MilestoneApproved",
    )
    .await;

    let created = TransactionWorkflowService::generate_transactions_for_trigger(
        db.pool.as_ref(),
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

    let count: i64 = sqlx::query_scalar(
        r#"
        SELECT COUNT(*)::BIGINT
        FROM operational_transactions
        WHERE engagement_id = $1
          AND milestone_id = $2
          AND trigger_event = $3
        "#,
    )
    .bind(100_i64)
    .bind(200_i64)
    .bind("MilestoneApproved")
    .fetch_one(db.pool.as_ref())
    .await
    .unwrap();

    assert_eq!(count, 1);
}

#[actix_rt::test]
#[serial]
async fn milestone_approved_generates_split_transaction() {
    let db = setup_test_db().await;

    seed_payout_rule(
        &db,
        "subcontractor_payout",
        Some(30),
        None,
        30,
        "MilestoneApproved",
    )
    .await;

    let created = TransactionWorkflowService::generate_transactions_for_trigger(
        db.pool.as_ref(),
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
#[serial]
async fn fixed_amount_rule_generates_fixed_transaction() {
    let db = setup_test_db().await;

    seed_payout_rule(&db, "dividend", None, Some(5000), 40, "MilestoneApproved").await;

    let created = TransactionWorkflowService::generate_transactions_for_trigger(
        db.pool.as_ref(),
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
#[serial]
async fn different_trigger_generates_no_transaction() {
    let db = setup_test_db().await;

    seed_payout_rule(
        &db,
        "contractor_payout",
        Some(100),
        None,
        20,
        "MilestoneApproved",
    )
    .await;

    let created = TransactionWorkflowService::generate_transactions_for_trigger(
        db.pool.as_ref(),
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
#[serial]
async fn transaction_generation_is_idempotent() {
    let db = setup_test_db().await;

    seed_payout_rule(
        &db,
        "contractor_payout",
        Some(100),
        None,
        20,
        "MilestoneApproved",
    )
    .await;

    let first = TransactionWorkflowService::generate_transactions_for_trigger(
        db.pool.as_ref(),
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
        db.pool.as_ref(),
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

    let count: i64 = sqlx::query_scalar(
        r#"
        SELECT COUNT(*)::BIGINT
        FROM operational_transactions
        WHERE agreement_id = $1
          AND engagement_id = $2
          AND milestone_id = $3
          AND transaction_type = $4
          AND trigger_event = $5
        "#,
    )
    .bind(1_i64)
    .bind(100_i64)
    .bind(200_i64)
    .bind("contractor_payout")
    .bind("MilestoneApproved")
    .fetch_one(db.pool.as_ref())
    .await
    .unwrap();

    assert_eq!(count, 1);
}
