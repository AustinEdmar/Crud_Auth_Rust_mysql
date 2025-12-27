use crate::db::category;


/* show */
 pub async fn show(
    db: &MySqlPool,
    category_id: u64,
    user_id: u64,
) -> Result<Option<Category>, sqlx::Error> {

    let category = sqlx::query_as!(
        Category,
        r#"
        SELECT id, name, description, created_at, updated_at
        FROM categories
        WHERE id = ? AND user_id = ?
        LIMIT 1
        "#,
        category_id,
        user_id
    )
    .fetch_optional(db)
    .await?;

    Ok(category)
}


 2 - em controllers/categories.rs


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




