1- crie um controler transactions
use actix_web::{post, get, put, delete, Responder};

#[post("/transactions")]
pub async fn index() -> impl Responder {
    "Transactions List"
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

2 - registre as rotas no arquivo main.rs
.service(controllers::transactions::index)
                    .service(controllers::transactions::create)
                    .service(controllers::transactions::show)
                    .service(controllers::transactions::update)
                    .service(controllers::transactions::delete)