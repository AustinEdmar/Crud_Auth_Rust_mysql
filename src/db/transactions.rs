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
