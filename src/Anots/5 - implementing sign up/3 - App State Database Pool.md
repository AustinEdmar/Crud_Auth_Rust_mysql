cargo add tokio
tokio = {version = "1.48.0", features = [ "full" ]}
cargo add dotenvy

1 / 
no main
use tokio::sync::Mutex;

struct AppState {
    db: Mutex< sqlx::MySqlPool>,
}


2 / 
async fn main() -> std::io::Result<()> {
    let state: Data<AppState> = web::Data::new(AppState {
        db: Mutex::new(sqlx::MySqlPool::connect("mysql://root:root@localhost:3306/crud")
        .await.unwrap()
    ),
    });



3 -
      .app_data(state.clone()) 


4 - dotenv

no main
    dotenvy::dotenv().ok();


    <!-- app final
    
use actix_web::{ App, HttpServer, web};
mod controllers;
mod db;

struct AppState {
    db: sqlx::MySqlPool,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    let pool = sqlx::MySqlPool::connect(&std::env::var("DATABASE_URL").unwrap())
    .await
    .unwrap();

    let state = web::Data::new(AppState { db: pool });
    HttpServer::new(move || App::new() //mover o estado para dentro dele
    .app_data(state.clone())
    .service(controllers::auth::sign_up)
    .service(controllers::auth::sign_in)
    .service(controllers::me::get_profile)
    .service(controllers::me::update_profile))

    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}



5 - verify the database connection

no auth.rs

use actix_web::{ post,web, Responder};
use crate::AppState;
use crate::db::user;

 // adicionar a pool do bancostate: web::Data<AppState> 

#[post("/auth/sign-up")]
pub async fn sign_up(state: web::Data<AppState>, data: web::Json<SignUpRequest>) -> impl Responder {
    let db = &state.db;

    if user::has_with_email(db, &data.email).await {
        return "User already exists".to_string();
    }

    format!("Sign up: {:?}", data)
} 