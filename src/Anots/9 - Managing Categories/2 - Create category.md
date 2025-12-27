1 - vamos criar um controller para gerenciar as categorias


//controllers/categories.rs

use actix_web::{HttpRequest, HttpResponse, Responder, delete, get, post, put, web};

use serde::Deserialize;
use serde_json::json;

use crate::{AppState, utils, db};


#[get("/categories")]
pub async fn index() -> impl Responder {
    "Lista Categorias"
    
}
<!-- 1 -->
#[derive(Deserialize, Debug)]
pub struct CreateCategoryRequest {
    pub name: String,
    pub description: Option<String>,
}



#[post("/categories")]
pub async fn create(
    state: web::Data<AppState>,
    req: HttpRequest,
    data: web::Json<CreateCategoryRequest>,  // Changed from UpdateProfileRequest
) -> impl Responder {
    // 1️⃣ Pega o user_id vindo do middleware JWT
    let user_id = utils::get_user_id(&req);

    let db = &state.db;

    // 2️⃣ Chama o DB
    let _ = db::category::create(&db, user_id, &data).await;
    
    HttpResponse::Ok().json(json!({
        "status": "success",
        "message": "Categoria criada com sucesso"
    }))
}


/* #[post("/categories")]
pub async fn create() -> impl Responder {
    "Cria Categoria"
} */


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

<!-- 2 -->
2 - no db/category

use sqlx::MySqlPool;
use crate::controllers::categories::CreateCategoryRequest;

pub async fn create(
    db: &MySqlPool,
    user_id: u64,
    category: &CreateCategoryRequest,
) -> Result<(), sqlx::Error> {

    sqlx::query!(
        "INSERT INTO categories (user_id, name, description)
         VALUES (?, ?, ?)",
        user_id,
        category.name,
        category.description
    )
    .execute(db)
    .await?;

    Ok(())
}
