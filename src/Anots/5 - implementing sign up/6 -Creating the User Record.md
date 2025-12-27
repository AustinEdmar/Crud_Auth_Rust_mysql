1 - vamos criar a logica de criar user
em db/user.rs

add
use crate::controllers::auth::SignUpRequest;
cargo add bcrypt

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


2 - vamos no auth.rs


use actix_web::{ post,web, Responder};
use crate::AppState;
use crate::db::user;

<!-- deixamos publico -->
pub struct SignUpRequest {
    pub email: String,
    pub password: String,
    pub firstname: String,
    pub lastname: String,
}

parametrizar o envio no [post("/auth/sign-up

#[post("/auth/sign-up")]
pub async fn sign_up(state: web::Data<AppState>, data: web::Json<SignUpRequest>) -> impl Responder {
    let db = &state.db;

    if user::has_with_email(db, &data.email).await {
        return "User already exists".to_string();
    }

    let _ = user::create(db, &data).await;

    "User created".to_string()

}