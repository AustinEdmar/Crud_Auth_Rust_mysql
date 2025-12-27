1 - em db/transactions.rs crie a funcao get_all_of_user

use serde::Serialize;
use sqlx::types::chrono::{DateTime, Utc};
use sqlx::MySqlPool;

#[derive(Debug, Serialize)]
pub struct Transaction {
    id: u64,
    user_id: u64,
    category_id: u64,
    r#type: String,
    amount: u64,
    memo: Option<String>,
    description: Option<String>,
    created_at: Option<DateTime<Utc>>,
    updated_at: Option<DateTime<Utc>>
} 


pub async fn get_all_of_user(
    db: &MySqlPool,
    user_id: u64,
) -> Result<Vec<Transaction>, sqlx::Error> {
    let transactions = sqlx::query_as!(
        Transaction,
        "SELECT * FROM transactions WHERE user_id = ? ORDER BY created_at DESC",
        user_id
    )
    .fetch_all(db)
    .await?;

    Ok(transactions)
}


2 - em controllers/transactions.rs crie a funcao index
<!-- 

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

 -->