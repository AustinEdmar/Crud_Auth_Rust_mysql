use actix_web::{App, HttpServer, web};
use actix_web::middleware::from_fn;
mod controllers;
mod db;
mod middleware;
mod utils;


struct AppState {
    db: sqlx::MySqlPool,
    json_web_token: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();

    let pool = sqlx::MySqlPool::connect(
        &std::env::var("DATABASE_URL").expect("DATABASE_URL missing")
    )
    .await
    .unwrap();

    let token_jwt = std::env::var("JWT_SECRET").expect("missing JWT_SECRET");
    //let token_jwt = std::env::var("JWT_SECRET").unwrap_or_else(|_| "default_secret_key".to_string());

    let state = web::Data::new(AppState { 
        db: pool,
        json_web_token: token_jwt,
    });


   

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(controllers::auth::sign_up)
            .service(controllers::auth::sign_in)
            .service(
                web::scope("/api")
                .wrap(from_fn(middleware::auth::verify_jwt))
                .service(controllers::me::get_profile)
                .service(controllers::me::update_profile)
                    .service(controllers::categories::index)
                    .service(controllers::categories::create)
                    .service(controllers::categories::show)
                    .service(controllers::categories::update)
                    .service(controllers::categories::delete)
                    .service(controllers::transactions::index)
                    .service(controllers::transactions::create)
                    .service(controllers::transactions::show)
                    .service(controllers::transactions::update)
                    .service(controllers::transactions::delete)
                    .service(controllers::users::index)
            )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
