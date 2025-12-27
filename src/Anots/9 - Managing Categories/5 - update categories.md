1 - controllers/categories.rs

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


2 - db/category.rs

pub async fn update_by_id_and_user(
    db: &MySqlPool,
    category_id: u64,
    user_id: u64,
    data: &UpdateCategoryRequest, 
) -> Result<u64, sqlx::Error> {

    let result = sqlx::query!(
        r#"
        UPDATE categories SET name = ?, description = ? WHERE id = ? AND user_id = ? "#,
        data.name,
        data.description,
        category_id,
        user_id
    )
    .execute(db)
    .await?;

    Ok(result.rows_affected())
}



 