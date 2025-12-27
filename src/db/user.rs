use serde::Serialize;
use sqlx::MySqlPool;
/* como o sqlx  assincrono precisamos definir as func assincronas, funca para 
verificar se existe um user com email especifico*/
use sqlx::types::chrono::{DateTime, Utc};
use crate::controllers::auth::SignUpRequest;
use crate::controllers::me::UpdateProfileRequest;
pub async fn has_with_email(
    db: &sqlx::MySqlPool, 
    email: &str) -> bool {
    sqlx::query!(
        "SELECT id FROM users WHERE email = ?",
        email
    )
    .fetch_optional(db)
    .await
    .unwrap()
    .is_some()
}


 pub async fn create(db: &sqlx::MySqlPool, user: &SignUpRequest) -> Result<(), sqlx::Error> {
    let hashed_password = bcrypt::hash(&user.password, bcrypt::DEFAULT_COST).unwrap();

    sqlx::query!(
        "INSERT INTO users (email, password, firstname, lastname) VALUES (?, ?, ?, ?)",
        &user.email,
        &hashed_password,
        &user.firstname,
        &user.lastname
    )
    .execute(db)
    .await?;

    Ok(())
}



#[derive(Serialize)]
pub struct User {
    pub id: u64,
    pub firstname: String,
   
    pub lastname: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub balance: u64,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,

}


pub async fn get_by_email(db: &sqlx::MySqlPool, email: &str) -> Option<User> {
    sqlx::query_as!(
    User,
    "SELECT * FROM users WHERE email = ?",
    email
)
.fetch_optional(db)
.await
.unwrap()

}




pub async fn get_by_id(db: &sqlx::MySqlPool, id: u64) -> Option<User> {
    sqlx::query_as!(
    User,
    "SELECT * FROM users WHERE id = ?",
    id
)
.fetch_optional(db)
.await
.unwrap()

}


pub async fn update(db: &sqlx::MySqlPool, id: u64, user: &UpdateProfileRequest){
   sqlx::query!(
    "UPDATE users SET firstname = ?, lastname = ?, email = ? WHERE id = ?",
    &user.firstname,
    &user.lastname,
    &user.email,
    id
   ).execute(db)
   .await
   .unwrap();
}



pub async fn list_all_users(
    
    db: &MySqlPool,
) -> Result<Vec<User>, sqlx::Error> {

    let users = sqlx::query_as!(
        User,
        r#"
        SELECT id, firstname, lastname, email, password, balance, created_at, updated_at
        FROM users
        ORDER BY created_at DESC
        "#,
       
    )
    .fetch_all(db)
    .await?;

    Ok(users)
}

