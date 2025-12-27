
use actix_web::{HttpRequest, HttpResponse, Responder, delete, get, post, put, web};

use serde::Deserialize;
use serde_json::json;

use crate::{AppState, utils, db};
use crate::db::category;



#[get("/categories")]
pub async fn index(
    req: HttpRequest,
    state: web::Data<AppState>,
) -> impl Responder {
 
    // 1️⃣ Pega o user_id do token
    let user_id = utils::get_user_id(&req);
    let db = &state.db;

    // 2️⃣ Busca no banco
    match db::category::list_by_user(db, user_id).await {
        Ok(categories) => HttpResponse::Ok().json(json!({
            "status": "success",
            "data": categories
        })),
        Err(e) => {
            eprintln!("Erro ao listar categorias: {:?}", e);
            HttpResponse::InternalServerError().json(json!({
                "status": "error",
                "message": "Erro ao listar categorias"
            }))
        } 
    }
}


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

 
#[get("/categories/{id}")]

pub async fn show(
    req: HttpRequest,
    state: web::Data<AppState>,
    path: web::Path<u64>,
) -> impl Responder { 

    // 1️⃣ user_id do token
    let user_id = utils::get_user_id(&req);

    // 2️⃣ id da URL
    let category_id = path.into_inner();

    let db = &state.db;

    // 3️⃣ Busca segura
    match category::show(db, category_id, user_id).await {
        Ok(Some(category)) => HttpResponse::Ok().json(json!({
            "status": "success",
            "data": category
        })),

        Ok(None) => HttpResponse::NotFound().json(json!({
            "status": "error",
            "message": "Categoria não encontrada"
        })),

        Err(e) => {
            eprintln!("Erro ao buscar categoria: {:?}", e);
            HttpResponse::InternalServerError().json(json!({
                "status": "error",
                "message": "Erro interno"
            }))
        }
    }
}


#[derive(Deserialize, Debug)]
pub struct UpdateCategoryRequest {
    pub name: String,
    pub description: Option<String>,
}

#[put("/categories/{id}")]
pub async fn update(
    req: HttpRequest,
    state: web::Data<AppState>,
    path: web::Path<u64>,
    data: web::Json<UpdateCategoryRequest>,
) -> impl Responder { 
    
     // 1️⃣ user_id do token
    let user_id = utils::get_user_id(&req);

    // 2️⃣ id da URL
    let category_id = path.into_inner();

    let db = &state.db;

    // 3️⃣ Atualiza com ownership
    match category::update_by_id_and_user(db, category_id, user_id, &data).await {
        Ok(affected) if affected > 0 => HttpResponse::Ok().json(json!({
            "status": "success",
            "message": "Categoria atualizada com sucesso"
        })),

        Ok(_) => HttpResponse::NotFound().json(json!({
            "status": "error",
            "message": "Categoria não encontrada"
        })),

        Err(e) => {
            eprintln!("Erro ao atualizar categoria: {:?}", e);
            HttpResponse::InternalServerError().json(json!({
                "status": "error",
                "message": "Erro interno"
            }))
        }
    }
    
}



#[delete("/categories/{id}")]
pub async fn delete(
    req: HttpRequest,
    state: web::Data<AppState>,
    path: web::Path<u64>,
) -> impl Responder {

    // 1️⃣ user_id do token
    let user_id = utils::get_user_id(&req);

    // 2️⃣ id da URL
    let category_id = path.into_inner();

    let db = &state.db;

    // 3️⃣ Delete com ownership
    match db::category::delete_by_id_and_user(db, category_id, user_id).await {
        Ok(affected) if affected > 0 => HttpResponse::Ok().json(json!({
            "status": "success",
            "message": "Categoria removida com sucesso"
        })),

        Ok(_) => HttpResponse::NotFound().json(json!({
            "status": "error",
            "message": "Categoria não encontrada"
        })),

        Err(e) => {
            eprintln!("Erro ao deletar categoria: {:?}", e);
            HttpResponse::InternalServerError().json(json!({
                "status": "error",
                "message": "Erro interno"
            }))
        }
    }
}