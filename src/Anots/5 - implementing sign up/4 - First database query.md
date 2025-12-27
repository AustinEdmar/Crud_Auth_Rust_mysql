1 - vamos criar uma consulta se  email ja existe
criei o db/mod.rs/
      pub mod user;

, no main mod db;

/db/user.rs
/* como o sqlx  assincrono precisamos definir as func assincronas, funca para 
verificar se existe um user com email especifico*/


pub async fn has_with_email(db: &sqlx::MySqlPool, email: &str) -> bool {
    sqlx::query!(
        "SELECT id FROM users WHERE email = ?",
        email
    )
    .fetch_optional(db)
    .await
    .unwrap()
    .is_some()
}


2 agora vamos no auth