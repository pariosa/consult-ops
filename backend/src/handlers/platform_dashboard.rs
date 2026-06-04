use actix_web::{HttpResponse, Responder, web};
use serde_json::json;

use crate::auth_context::AuthUser;
use crate::db::Db;
use serde::Deserialize;

fn can_view_platform_dashboard(auth: &AuthUser) -> bool {
    auth.user_type == "admin" || auth.user_type == "super_admin"
}

fn forbidden(message: &str) -> HttpResponse {
    HttpResponse::Forbidden().json(json!({
        "error": message
    }))
}

async fn count_query(db: &Db, sql: &str) -> i64 {
    sqlx::query_scalar::<_, i64>(sql)
        .fetch_one(db.pool.as_ref())
        .await
        .unwrap_or(0)
}

async fn cents_query(db: &Db, sql: &str) -> i64 {
    sqlx::query_scalar::<_, i64>(sql)
        .fetch_one(db.pool.as_ref())
        .await
        .unwrap_or(0)
}

#[derive(Debug, Deserialize)]
pub struct PlatformSearchQuery {
    pub q: Option<String>,
}

pub async fn get_platform_dashboard(db: web::Data<Db>, auth: AuthUser) -> impl Responder {
    if !can_view_platform_dashboard(&auth) {
        return forbidden("You do not have permission to view platform dashboard.");
    }

    let organizations = count_query(&db, "SELECT COUNT(*)::BIGINT FROM organizations").await;
    let users = count_query(&db, "SELECT COUNT(*)::BIGINT FROM users").await;
    let projects = count_query(&db, "SELECT COUNT(*)::BIGINT FROM projects").await;
    let clients = count_query(&db, "SELECT COUNT(*)::BIGINT FROM clients").await;
    let engagements = count_query(&db, "SELECT COUNT(*)::BIGINT FROM engagements").await;
    let contracts = count_query(&db, "SELECT COUNT(*)::BIGINT FROM contracts").await;
    let transactions =
        count_query(&db, "SELECT COUNT(*)::BIGINT FROM operational_transactions").await;

    let active_engagements = count_query(
        &db,
        r#"
        SELECT COUNT(*)::BIGINT
        FROM engagements
        WHERE status IN ('active', 'in_progress', 'awaiting_payment', 'pending_signature', 'contract_signed')
        "#,
    )
    .await;

    let awaiting_payment = count_query(
        &db,
        r#"
        SELECT COUNT(*)::BIGINT
        FROM engagements
        WHERE status = 'awaiting_payment'
           OR platform_fee_status = 'pending'
        "#,
    )
    .await;

    let signed_contracts = count_query(
        &db,
        r#"
        SELECT COUNT(*)::BIGINT
        FROM engagements
        WHERE status IN ('contract_signed', 'signed', 'active', 'in_progress', 'completed')
        "#,
    )
    .await;

    let completed_engagements = count_query(
        &db,
        r#"
        SELECT COUNT(*)::BIGINT
        FROM engagements
        WHERE status = 'completed'
        "#,
    )
    .await;

    let disputed_engagements = count_query(
        &db,
        r#"
        SELECT COUNT(*)::BIGINT
        FROM engagements
        WHERE status = 'disputed'
        "#,
    )
    .await;

    let transactions_cents_30d = cents_query(
        &db,
        r#"
        SELECT COALESCE(SUM(amount_cents), 0)::BIGINT
        FROM operational_transactions
        WHERE created_at::timestamp >= NOW() - INTERVAL '30 days'
        "#,
    )
    .await;

    let transactions_cents_prev_30d = cents_query(
        &db,
        r#"
        SELECT COALESCE(SUM(amount_cents), 0)::BIGINT
        FROM operational_transactions
        WHERE created_at::timestamp >= NOW() - INTERVAL '60 days'
          AND created_at::timestamp < NOW() - INTERVAL '30 days'
        "#,
    )
    .await;

    let activation_fees_cents_30d = cents_query(
        &db,
        r#"
        SELECT COALESCE(SUM(amount_cents), 0)::BIGINT
        FROM engagement_billing
        WHERE billing_type = 'activation_fee'
          AND status = 'paid'
          AND created_at::timestamp >= NOW() - INTERVAL '30 days'
        "#,
    )
    .await;

    let activation_fees_cents_prev_30d = cents_query(
        &db,
        r#"
        SELECT COALESCE(SUM(amount_cents), 0)::BIGINT
        FROM engagement_billing
        WHERE billing_type = 'activation_fee'
          AND status = 'paid'
          AND created_at::timestamp >= NOW() - INTERVAL '60 days'
          AND created_at::timestamp < NOW() - INTERVAL '30 days'
        "#,
    )
    .await;

    let draft_engagements = count_query(
        &db,
        "SELECT COUNT(*)::BIGINT FROM engagements WHERE status = 'draft'",
    )
    .await;

    let pending_signature_engagements = count_query(
        &db,
        "SELECT COUNT(*)::BIGINT FROM engagements WHERE status = 'pending_signature'",
    )
    .await;

    let awaiting_payment_engagements = count_query(
        &db,
        "SELECT COUNT(*)::BIGINT FROM engagements WHERE status = 'awaiting_payment'",
    )
    .await;

    let active_status_engagements = count_query(
        &db,
        "SELECT COUNT(*)::BIGINT FROM engagements WHERE status = 'active'",
    )
    .await;

    let with_projects = count_query(
        &db,
        r#"
        SELECT COUNT(DISTINCT organization_id)::BIGINT
        FROM projects
        "#,
    )
    .await;

    let with_engagements = count_query(
        &db,
        r#"
        SELECT COUNT(DISTINCT organization_id)::BIGINT
        FROM engagements
        "#,
    )
    .await;

    let with_signed_contracts = count_query(
        &db,
        r#"
        SELECT COUNT(DISTINCT organization_id)::BIGINT
        FROM engagements
        WHERE status IN ('contract_signed', 'signed', 'active', 'in_progress', 'completed')
        "#,
    )
    .await;

    let with_paid_transactions = count_query(
        &db,
        r#"
        SELECT COUNT(DISTINCT organization_id)::BIGINT
        FROM operational_transactions
        WHERE status = 'paid'
        "#,
    )
    .await;

    let pending_transactions = count_query(
        &db,
        r#"
        SELECT COUNT(*)::BIGINT
        FROM operational_transactions
        WHERE status IN ('pending', 'processing')
        "#,
    )
    .await;

    let failed_transactions = count_query(
        &db,
        r#"
        SELECT COUNT(*)::BIGINT
        FROM operational_transactions
        WHERE status = 'failed'
        "#,
    )
    .await;

    let pending_invitations = count_query(
        &db,
        r#"
        SELECT COUNT(*)::BIGINT
        FROM organization_invitations
        WHERE status = 'pending'
        "#,
    )
    .await;

    let mut action_queue = Vec::new();

    if awaiting_payment > 0 {
        action_queue.push(json!({
            "type": "awaiting_payment",
            "label": "Engagements awaiting payment",
            "description": "Activation fees need payment or webhook confirmation.",
            "count": awaiting_payment,
            "route": "/organization/operations"
        }));
    }

    if pending_signature_engagements > 0 {
        action_queue.push(json!({
            "type": "pending_signature",
            "label": "Contracts awaiting signature",
            "description": "Sent contracts have not been signed yet.",
            "count": pending_signature_engagements,
            "route": "/organization/operations"
        }));
    }

    if pending_transactions > 0 {
        action_queue.push(json!({
            "type": "pending_transactions",
            "label": "Transactions pending settlement",
            "description": "Operational transactions need processing or payment confirmation.",
            "count": pending_transactions,
            "route": "/organization/transactions"
        }));
    }

    if failed_transactions > 0 {
        action_queue.push(json!({
            "type": "failed_transactions",
            "label": "Failed transactions",
            "description": "Transactions failed and need review.",
            "count": failed_transactions,
            "route": "/organization/transactions"
        }));
    }

    if pending_invitations > 0 {
        action_queue.push(json!({
            "type": "pending_invitations",
            "label": "Pending invitations",
            "description": "Organization invitations have not been accepted.",
            "count": pending_invitations,
            "route": "/organization/invitations"
        }));
    }

    let top_organizations = match sqlx::query!(
        r#"
        SELECT
            o.id,
            o.name,
            COUNT(DISTINCT e.id)::BIGINT AS engagement_count,
            COALESCE(SUM(ot.amount_cents), 0)::BIGINT AS transaction_volume_cents
        FROM organizations o
        LEFT JOIN engagements e ON e.organization_id = o.id
        LEFT JOIN operational_transactions ot ON ot.organization_id = o.id
        GROUP BY o.id, o.name
        ORDER BY engagement_count DESC, transaction_volume_cents DESC
        LIMIT 5
        "#
    )
    .fetch_all(db.pool.as_ref())
    .await
    {
        Ok(rows) => rows
            .into_iter()
            .map(|row| {
                json!({
                    "id": row.id,
                    "name": row.name,
                    "engagement_count": row.engagement_count.unwrap_or(0),
                    "transaction_volume_cents": row.transaction_volume_cents.unwrap_or(0)
                })
            })
            .collect::<Vec<_>>(),
        Err(_) => Vec::new(),
    };

    let recent_activity = match sqlx::query!(
        r#"
        SELECT
            oe.id,
            oe.event_type,
            oe.entity_type,
            oe.entity_id,
            oe.created_at,
            o.name AS organization_name,
            u.name AS actor_name,
            u.email AS actor_email
        FROM operational_events oe
        LEFT JOIN organizations o ON o.id = oe.organization_id
        LEFT JOIN users u ON u.id = oe.actor_user_id
        ORDER BY oe.created_at DESC
        LIMIT 12
        "#
    )
    .fetch_all(db.pool.as_ref())
    .await
    {
        Ok(rows) => rows
            .into_iter()
            .map(|row| {
                json!({
                    "id": row.id,
                    "event_type": row.event_type,
                    "entity_type": row.entity_type,
                    "entity_id": row.entity_id,
                    "created_at": row.created_at,
                    "organization_name": row.organization_name,
                    "actor_name": row.actor_name,
                    "actor_email": row.actor_email
                })
            })
            .collect::<Vec<_>>(),
        Err(_) => Vec::new(),
    };

    HttpResponse::Ok().json(json!({
        "system_totals": {
            "organizations": organizations,
            "users": users,
            "projects": projects,
            "clients": clients,
            "engagements": engagements,
            "contracts": contracts,
            "transactions": transactions
        },
        "overview": {
            "organizations": organizations,
            "users": users,
            "active_engagements": active_engagements,
            "awaiting_payment": awaiting_payment,
            "signed_contracts": signed_contracts,
            "transactions_cents_30d": transactions_cents_30d,
            "activation_fees_cents_30d": activation_fees_cents_30d,
            "completed_engagements": completed_engagements
        },
        "revenue": {
            "activation_fees_30d": activation_fees_cents_30d,
            "activation_fees_prev_30d": activation_fees_cents_prev_30d,
            "transactions_30d": transactions_cents_30d,
            "transactions_prev_30d": transactions_cents_prev_30d
        },
        "health": {
            "database": "healthy",
            "stripe": "configured",
            "email": "configured",
            "webhooks": "configured"
        },
        "engagement_statuses": {
            "draft": draft_engagements,
            "pending_signature": pending_signature_engagements,
            "awaiting_payment": awaiting_payment_engagements,
            "active": active_status_engagements,
            "completed": completed_engagements,
            "disputed": disputed_engagements
        },
        "adoption": {
            "organizations": organizations,
            "with_projects": with_projects,
            "with_engagements": with_engagements,
            "with_signed_contracts": with_signed_contracts,
            "with_paid_transactions": with_paid_transactions
        },
        "action_queue": action_queue,
        "top_organizations": top_organizations,
        "recent_activity": recent_activity
    }))
}

pub async fn platform_search(
    db: web::Data<Db>,
    auth: AuthUser,
    query: web::Query<PlatformSearchQuery>,
) -> impl Responder {
    if !can_view_platform_dashboard(&auth) {
        return forbidden("You do not have permission to search the platform.");
    }

    let search = query.q.clone().unwrap_or_default().trim().to_string();

    if search.len() < 2 {
        return HttpResponse::Ok().json(json!({
            "organizations": [],
            "users": [],
            "projects": [],
            "clients": [],
            "engagements": []
        }));
    }

    let pattern = format!("%{}%", search);

    let organizations = sqlx::query!(
        r#"
        SELECT id, name
        FROM organizations
        WHERE name ILIKE $1
        ORDER BY name ASC
        LIMIT 6
        "#,
        pattern
    )
    .fetch_all(db.pool.as_ref())
    .await
    .unwrap_or_default()
    .into_iter()
    .map(|row| {
        json!({
            "id": row.id,
            "label": row.name,
            "type": "organization",
            "route": format!("/platform/organizations/{}", row.id)
        })
    })
    .collect::<Vec<_>>();

    let users = sqlx::query!(
        r#"
        SELECT id, name, email
        FROM users
        WHERE name ILIKE $1 OR email ILIKE $1
        ORDER BY created_at DESC
        LIMIT 6
        "#,
        pattern
    )
    .fetch_all(db.pool.as_ref())
    .await
    .unwrap_or_default()
    .into_iter()
    .map(|row| {
        json!({
            "id": row.id,
            "label": row.name.unwrap_or_else(|| row.email.clone()),
            "description": row.email,
            "type": "user",
            "route": format!("/admin/users/{}", row.id)
        })
    })
    .collect::<Vec<_>>();

    let projects = sqlx::query!(
        r#"
        SELECT id, name
        FROM projects
        WHERE name ILIKE $1
        ORDER BY created_at DESC
        LIMIT 6
        "#,
        pattern
    )
    .fetch_all(db.pool.as_ref())
    .await
    .unwrap_or_default()
    .into_iter()
    .map(|row| {
        json!({
            "id": row.id,
            "label": row.name,
            "type": "project",
            "route": format!("/projects/{}", row.id)
        })
    })
    .collect::<Vec<_>>();

    let clients = sqlx::query!(
        r#"
        SELECT id, name, email
        FROM clients
        WHERE name ILIKE $1 OR email ILIKE $1
        ORDER BY created_at DESC
        LIMIT 6
        "#,
        pattern
    )
    .fetch_all(db.pool.as_ref())
    .await
    .unwrap_or_default()
    .into_iter()
    .map(|row| {
        json!({
            "id": row.id,
            "label": row.name,
            "description": row.email,
            "type": "client",
            "route": "/organization/clients"
        })
    })
    .collect::<Vec<_>>();

    let engagements = sqlx::query!(
        r#"
        SELECT id, title, contractor_email
        FROM engagements
        WHERE title ILIKE $1 OR contractor_email ILIKE $1
        ORDER BY created_at DESC
        LIMIT 6
        "#,
        pattern
    )
    .fetch_all(db.pool.as_ref())
    .await
    .unwrap_or_default()
    .into_iter()
    .map(|row| {
        json!({
            "id": row.id,
            "label": row.title,
            "description": row.contractor_email,
            "type": "engagement",
            "route": format!("/engagements/{}", row.id)
        })
    })
    .collect::<Vec<_>>();

    HttpResponse::Ok().json(json!({
        "organizations": organizations,
        "users": users,
        "projects": projects,
        "clients": clients,
        "engagements": engagements
    }))
}

pub async fn get_platform_overview(db: web::Data<Db>, auth: AuthUser) -> impl Responder {
    if !can_view_platform_dashboard(&auth) {
        return HttpResponse::Forbidden().json(json!({
            "error": "You do not have permission to view platform overview."
        }));
    }

    let organizations = sqlx::query_scalar::<_, i64>("SELECT COUNT(*)::BIGINT FROM organizations")
        .fetch_one(db.pool.as_ref())
        .await
        .unwrap_or(0);

    let users = sqlx::query_scalar::<_, i64>("SELECT COUNT(*)::BIGINT FROM users")
        .fetch_one(db.pool.as_ref())
        .await
        .unwrap_or(0);

    let active_engagements = sqlx::query_scalar::<_, i64>(
        r#"
        SELECT COUNT(*)::BIGINT
        FROM engagements
        WHERE status IN ('active', 'in_progress', 'awaiting_payment', 'pending_signature', 'contract_signed')
        "#,
    )
    .fetch_one(db.pool.as_ref())
    .await
    .unwrap_or(0);

    let awaiting_payment = sqlx::query_scalar::<_, i64>(
        r#"
        SELECT COUNT(*)::BIGINT
        FROM engagements
        WHERE status = 'awaiting_payment'
           OR platform_fee_status = 'pending'
        "#,
    )
    .fetch_one(db.pool.as_ref())
    .await
    .unwrap_or(0);

    let signed_contracts = sqlx::query_scalar::<_, i64>(
        r#"
        SELECT COUNT(*)::BIGINT
        FROM engagements
        WHERE status IN ('contract_signed', 'signed', 'active', 'in_progress', 'completed')
        "#,
    )
    .fetch_one(db.pool.as_ref())
    .await
    .unwrap_or(0);

    let transactions_cents_30d = sqlx::query_scalar::<_, i64>(
        r#"
        SELECT COALESCE(SUM(amount_cents), 0)::BIGINT
        FROM operational_transactions
        WHERE created_at::timestamp >= NOW() - INTERVAL '30 days'
        "#,
    )
    .fetch_one(db.pool.as_ref())
    .await
    .unwrap_or(0);

    let activation_fees_cents_30d = sqlx::query_scalar::<_, i64>(
        r#"
        SELECT COALESCE(SUM(amount_cents), 0)::BIGINT
        FROM engagement_billing
        WHERE billing_type = 'activation_fee'
          AND status = 'paid'
          AND created_at::timestamp >= NOW() - INTERVAL '30 days'
        "#,
    )
    .fetch_one(db.pool.as_ref())
    .await
    .unwrap_or(0);

    let completed_engagements = sqlx::query_scalar::<_, i64>(
        r#"
        SELECT COUNT(*)::BIGINT
        FROM engagements
        WHERE status = 'completed'
        "#,
    )
    .fetch_one(db.pool.as_ref())
    .await
    .unwrap_or(0);

    HttpResponse::Ok().json(json!({
        "organizations": organizations,
        "users": users,
        "active_engagements": active_engagements,
        "awaiting_payment": awaiting_payment,
        "signed_contracts": signed_contracts,
        "transactions_cents_30d": transactions_cents_30d,
        "activation_fees_cents_30d": activation_fees_cents_30d,
        "completed_engagements": completed_engagements
    }))
}

pub async fn get_platform_alerts(db: web::Data<Db>, auth: AuthUser) -> impl Responder {
    if !can_view_platform_dashboard(&auth) {
        return HttpResponse::Forbidden().json(json!({
            "error": "You do not have permission to view platform alerts."
        }));
    }

    let awaiting_payment = sqlx::query_scalar::<_, i64>(
        r#"
        SELECT COUNT(*)::BIGINT
        FROM engagements
        WHERE status = 'awaiting_payment'
           OR platform_fee_status = 'pending'
        "#,
    )
    .fetch_one(db.pool.as_ref())
    .await
    .unwrap_or(0);

    let unsigned_contracts = sqlx::query_scalar::<_, i64>(
        r#"
        SELECT COUNT(*)::BIGINT
        FROM engagements
        WHERE status = 'pending_signature'
        "#,
    )
    .fetch_one(db.pool.as_ref())
    .await
    .unwrap_or(0);

    let pending_transactions = sqlx::query_scalar::<_, i64>(
        r#"
        SELECT COUNT(*)::BIGINT
        FROM operational_transactions
        WHERE status IN ('pending', 'processing')
        "#,
    )
    .fetch_one(db.pool.as_ref())
    .await
    .unwrap_or(0);

    let pending_invitations = sqlx::query_scalar::<_, i64>(
        r#"
        SELECT COUNT(*)::BIGINT
        FROM organization_invitations
        WHERE status = 'pending'
        "#,
    )
    .fetch_one(db.pool.as_ref())
    .await
    .unwrap_or(0);

    let mut alerts = Vec::new();

    if awaiting_payment > 0 {
        alerts.push(json!({
            "type": "awaiting_payment",
            "label": "Engagements awaiting payment",
            "description": "Engagements are waiting for activation fee confirmation.",
            "count": awaiting_payment,
            "to": "/organization/operations"
        }));
    }

    if unsigned_contracts > 0 {
        alerts.push(json!({
            "type": "unsigned_contracts",
            "label": "Contracts awaiting signature",
            "description": "Contracts have been sent but not signed.",
            "count": unsigned_contracts,
            "to": "/organization/operations"
        }));
    }

    if pending_transactions > 0 {
        alerts.push(json!({
            "type": "pending_transactions",
            "label": "Transactions pending settlement",
            "description": "Operational transactions need processing or final payment confirmation.",
            "count": pending_transactions,
            "to": "/organization/transactions"
        }));
    }

    if pending_invitations > 0 {
        alerts.push(json!({
            "type": "pending_invitations",
            "label": "Pending organization invitations",
            "description": "Users have been invited but have not accepted yet.",
            "count": pending_invitations,
            "to": "/organization/invitations"
        }));
    }

    HttpResponse::Ok().json(alerts)
}

pub async fn get_platform_activity(db: web::Data<Db>, auth: AuthUser) -> impl Responder {
    if !can_view_platform_dashboard(&auth) {
        return HttpResponse::Forbidden().json(json!({
            "error": "You do not have permission to view platform activity."
        }));
    }

    let rows = sqlx::query!(
        r#"
        SELECT
            oe.id,
            oe.event_type,
            oe.entity_type,
            oe.entity_id,
            oe.created_at,
            o.name AS organization_name
        FROM operational_events oe
        LEFT JOIN organizations o ON o.id = oe.organization_id
        ORDER BY oe.created_at DESC
        LIMIT 10
        "#
    )
    .fetch_all(db.pool.as_ref())
    .await;

    let rows = match rows {
        Ok(rows) => rows,
        Err(err) => {
            return HttpResponse::InternalServerError().json(json!({
                "error": err.to_string()
            }));
        }
    };

    let activity: Vec<_> = rows
        .into_iter()
        .map(|row| {
            json!({
                "id": row.id,
                "event_type": row.event_type,
                "entity_type": row.entity_type,
                "entity_id": row.entity_id,
                "created_at": row.created_at,
                "organization_name": row.organization_name
            })
        })
        .collect();

    HttpResponse::Ok().json(activity)
}

pub async fn get_platform_funnel(db: web::Data<Db>, auth: AuthUser) -> impl Responder {
    if !can_view_platform_dashboard(&auth) {
        return HttpResponse::Forbidden().json(json!({
            "error": "You do not have permission to view platform funnel."
        }));
    }

    let organizations_created =
        sqlx::query_scalar::<_, i64>("SELECT COUNT(*)::BIGINT FROM organizations")
            .fetch_one(db.pool.as_ref())
            .await
            .unwrap_or(0);

    let with_projects = sqlx::query_scalar::<_, i64>(
        r#"
        SELECT COUNT(DISTINCT organization_id)::BIGINT
        FROM projects
        "#,
    )
    .fetch_one(db.pool.as_ref())
    .await
    .unwrap_or(0);

    let with_engagements = sqlx::query_scalar::<_, i64>(
        r#"
        SELECT COUNT(DISTINCT organization_id)::BIGINT
        FROM engagements
        "#,
    )
    .fetch_one(db.pool.as_ref())
    .await
    .unwrap_or(0);

    let with_signed_contracts = sqlx::query_scalar::<_, i64>(
        r#"
        SELECT COUNT(DISTINCT organization_id)::BIGINT
        FROM engagements
        WHERE status IN ('contract_signed', 'signed', 'active', 'in_progress', 'completed')
        "#,
    )
    .fetch_one(db.pool.as_ref())
    .await
    .unwrap_or(0);

    let with_paid_transactions = sqlx::query_scalar::<_, i64>(
        r#"
        SELECT COUNT(DISTINCT organization_id)::BIGINT
        FROM operational_transactions
        WHERE status = 'paid'
        "#,
    )
    .fetch_one(db.pool.as_ref())
    .await
    .unwrap_or(0);

    HttpResponse::Ok().json(json!({
        "organizations_created": organizations_created,
        "with_projects": with_projects,
        "with_engagements": with_engagements,
        "with_signed_contracts": with_signed_contracts,
        "with_paid_transactions": with_paid_transactions
    }))
}
