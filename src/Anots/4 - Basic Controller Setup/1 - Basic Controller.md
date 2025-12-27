deixe o main assim 

use actix_web::{ App, HttpServer};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new())
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}


2 - agora vamos criar o controllers

em src/controllers
    mod.rs:
        pub mod auth;
        pub mod me;
    auth.rs:
    
        #[post("/auth/sign-up")]
        pub async fn sign_up() -> impl Responder {
            "Sign Up"
        }


        #[post("/auth/sign-in")]
        pub async fn sign_in() -> impl Responder {
            "Sign In"
        }

    me.rs: 
      
      use actix_web::{ get, post,web, Responder};
        #[get("/me")]
        pub async fn get_profile() -> impl Responder {
            "Profile"
        }


        #[post("/me")]
        pub async fn update_profile() -> impl Responder {
            "Update profile"
        } 



3 - no main chamo  e adicionei o me

use actix_web::{ App, HttpServer};

mod controllers;

use actix_web::{ App, HttpServer};

mod controllers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new()
    .service(controllers::auth::sign_up)
    .service(controllers::auth::sign_in)
    .service(controllers::me::get_profile)
    .service(controllers::me::update_profile))

    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
        
