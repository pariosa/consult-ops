use actix_web::{HttpResponse, Responder, web};

use crate::db::Db;
use crate::models::engagement_milestone::{
    CreateEngagementMilestone, CreateEngagementMilestoneRequest, EngagementMilestone,
    UpdateEngagementMilestoneRequest,
};

/// CREATE milestone for engagement
pub async fn create_engagement_milestone(
    db: web::Data<Db>,
    path: web::Path<i64>,
    payload: web::Json<CreateEngagementMilestoneRequest>,
) -> impl Responder {
    let engagement_id = path.into_inner();
    let input = payload.into_inner();

    let milestone = CreateEngagementMilestone {
        engagement_id,
        title: input.title,
        description: input.description,
        amount_cents: input.amount_cents.unwrap_or(0),
        due_date: input.due_date,
    };

    match EngagementMilestone::create(db.pool.as_ref(), milestone).await {
        Ok(milestone) => HttpResponse::Created().json(milestone),
        Err(err) => {
            eprintln!("create_engagement_milestone error: {:?}", err);
            HttpResponse::InternalServerError().body(err.to_string())
        }
    }
}

/// LIST milestones for engagement
pub async fn list_engagement_milestones(db: web::Data<Db>, path: web::Path<i64>) -> impl Responder {
    let engagement_id = path.into_inner();

    println!("Loading milestones for engagement_id: {}", engagement_id);

    match EngagementMilestone::for_engagement(db.pool.as_ref(), engagement_id).await {
        Ok(items) => HttpResponse::Ok().json(items),

        Err(err) => {
            eprintln!("list_engagement_milestones error: {:?}", err);

            HttpResponse::InternalServerError().body(err.to_string())
        }
    }
}

/// MARK milestone submitted
pub async fn submit_engagement_milestone(
    db: web::Data<Db>,
    path: web::Path<i64>,
) -> impl Responder {
    let milestone_id = path.into_inner();

    println!("Submitting milestone_id: {}", milestone_id);

    match EngagementMilestone::update_status(db.pool.as_ref(), milestone_id, "submitted").await {
        Ok(item) => HttpResponse::Ok().json(item),

        Err(err) => {
            eprintln!("submit_engagement_milestone error: {:?}", err);

            HttpResponse::InternalServerError().body(err.to_string())
        }
    }
}

/// MARK milestone approved
pub async fn approve_engagement_milestone(
    db: web::Data<Db>,
    path: web::Path<i64>,
) -> impl Responder {
    let milestone_id = path.into_inner();

    println!("Approving milestone_id: {}", milestone_id);

    match EngagementMilestone::update_status(db.pool.as_ref(), milestone_id, "approved").await {
        Ok(item) => HttpResponse::Ok().json(item),

        Err(err) => {
            eprintln!("approve_engagement_milestone error: {:?}", err);

            HttpResponse::InternalServerError().body(err.to_string())
        }
    }
}

/// MARK milestone paid
pub async fn mark_engagement_milestone_paid(
    db: web::Data<Db>,
    path: web::Path<i64>,
) -> impl Responder {
    let milestone_id = path.into_inner();

    println!("Marking milestone paid: {}", milestone_id);

    match EngagementMilestone::update_status(db.pool.as_ref(), milestone_id, "paid").await {
        Ok(item) => HttpResponse::Ok().json(item),

        Err(err) => {
            eprintln!("mark_engagement_milestone_paid error: {:?}", err);

            HttpResponse::InternalServerError().body(err.to_string())
        }
    }
}

/// UPDATE milestone details
pub async fn update_engagement_milestone(
    db: web::Data<Db>,
    path: web::Path<i64>,
    payload: web::Json<UpdateEngagementMilestoneRequest>,
) -> impl Responder {
    let milestone_id = path.into_inner();
    let input = payload.into_inner();

    println!("Updating milestone_id: {}", milestone_id);

    match EngagementMilestone::update(db.pool.as_ref(), milestone_id, input).await {
        Ok(item) => HttpResponse::Ok().json(item),

        Err(err) => {
            eprintln!("update_engagement_milestone error: {:?}", err);
            HttpResponse::InternalServerError().body(err.to_string())
        }
    }
}

/// REOPEN milestone
pub async fn reopen_engagement_milestone(
    db: web::Data<Db>,
    path: web::Path<i64>,
) -> impl Responder {
    let milestone_id = path.into_inner();

    println!("Reopening milestone_id: {}", milestone_id);

    match EngagementMilestone::reopen(db.pool.as_ref(), milestone_id).await {
        Ok(item) => HttpResponse::Ok().json(item),

        Err(err) => {
            eprintln!("reopen_engagement_milestone error: {:?}", err);
            HttpResponse::InternalServerError().body(err.to_string())
        }
    }
}
