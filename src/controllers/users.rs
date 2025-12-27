use actix_web::{ HttpResponse, Responder, get,  web};


use serde_json::json;

use crate::{AppState, db};

#[get("/users")]
pub async fn index(
    //req: HttpRequest,
    state: web::Data<AppState>,
) -> impl Responder {

    // 1️⃣ Pega o user_id do token
   // let user_id = utils::get_user_id(&req);
    let db = &state.db;

    // 2️⃣ Busca no banco
    match db::user::list_all_users(db).await {
        Ok(users) => HttpResponse::Ok().json(json!({
            "status": "success",
            "data": users
        })),
        Err(e) => {
            eprintln!("Erro ao listar users: {:?}", e);
            HttpResponse::InternalServerError().json(json!({
                "status": "error",
                "message": "Erro ao listar users"
            }))
        }
    }
}
