use crate::db::Db;
use crate::models::Contract;
use actix_web::{HttpResponse, Responder, web};

/// Get all contracts
pub async fn get_contracts(db: web::Data<Db>) -> impl Responder {
    match Contract::all(&db).await {
        Ok(contracts) => HttpResponse::Ok().json(contracts),
        Err(e) => {
            eprintln!("DB error fetching contracts: {}", e);
            HttpResponse::InternalServerError().body("Failed to fetch contracts")
        }
    }
}

/// Create a new contract
pub async fn create_contract(db: web::Data<Db>, info: web::Json<Contract>) -> impl Responder {
    let mut contract = info.into_inner();

    // Ensure required fields have defaults if needed
    if contract.status.is_empty() {
        contract.status = "draft".to_string();
    }

    // Set created_at if not provided
    if contract.created_at.is_empty() {
        contract.created_at = chrono::Utc::now().to_rfc3339();
    }

    match Contract::create(&db, contract).await {
        Ok(contract) => HttpResponse::Ok().json(contract),
        Err(e) => {
            eprintln!("DB error creating contract: {}", e);
            HttpResponse::InternalServerError().body("Failed to create contract")
        }
    }
}
