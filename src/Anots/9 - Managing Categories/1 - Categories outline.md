1 - criar controller para as categorias

use actix_web::{ get, post,put,delete, Responder};

#[get("/categories")]
pub async fn index() -> impl Responder {
    "Lista Categorias"
    
}

#[post("/categories")]
pub async fn create() -> impl Responder {
    "Cria Categoria"
}


#[get("/categories/{id}")]
pub async fn show() -> impl Responder {
    "Cria Categoria"
}

#[put("/categories/{id}")]
pub async fn update() -> impl Responder {
    "Cria Categoria"
}

#[delete("/categories/{id}")]
pub async fn delete() -> impl Responder {
    "Cria Categoria"
}

2 - coloco no mod
pub mod categories;


3 - no main.rs registrar-los

 .service(controllers::categories::index)
                .service(controllers::categories::create)
                .service(controllers::categories::show)
                .service(controllers::categories::update)
                .service(controllers::categories::delete)