#[tokio::test]
async fn milestone_approval_does_not_generate_transaction_without_agreement() {
    // create org/project/engagement/milestone
    // approve milestone
    // assert operational_transactions count == 0
    // assert response/timeline says agreement_required or payout_blocked
}

#[tokio::test]
async fn milestone_approval_does_not_generate_transaction_without_payout_rules() {
    // create agreement but no payout rules
    // approve milestone
    // assert operational_transactions count == 0
}

#[tokio::test]
async fn milestone_approval_generates_transaction_when_agreement_and_rules_exist() {
    // create agreement
    // add payout rule
    // lock/activate agreement if your model requires it
    // approve milestone
    // assert operational_transactions count == 1
    // assert amount/status/trigger_event are correct
}
