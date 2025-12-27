use actix_web::{HttpResponse, Responder, delete, get, post, put, HttpRequest, web};
use crate::{AppState, utils};
use crate::db;


#[get("/transactions")]
pub async fn index(
    req: HttpRequest,
    state: web::Data<AppState>,
) -> impl Responder {
    // Get user_id from token
    let user_id = utils::get_user_id(&req);
    let db = &state.db;

    match db::transactions::get_all_of_user(db, user_id).await {
        Ok(transactions) => HttpResponse::Ok().json(transactions),
        Err(e) => {
            // Log the error for debugging
            eprintln!("Failed to fetch transactions: {}", e);
            // Return a 500 Internal Server Error
            HttpResponse::InternalServerError().json("Failed to fetch transactions")
        }
    }
}




#[get("/transactions/{id}")]
pub async fn show() -> impl Responder {
    "Transactions Show"
} 


#[post("/transactions")]
pub async fn create() -> impl Responder {
    "Transactions Create"
}

#[put("/transactions/{id}")]
pub async fn update() -> impl Responder {
    "Transactions Update"
}

#[delete("/transactions/{id}")]
pub async fn delete() -> impl Responder {
    "Transactions Delete"
}