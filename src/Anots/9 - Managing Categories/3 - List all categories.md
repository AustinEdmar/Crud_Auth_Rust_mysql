1 - vamos criar um controller para gerenciar as categorias

em db/category.rs


#[derive(Debug, Serialize)]
pub struct Category {
    pub id: u64,
    pub name: String,
    pub description: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>
}


pub async fn list_by_user(
    db: &MySqlPool,
    user_id: u64,
) -> Result<Vec<Category>, sqlx::Error> {

    let categories = sqlx::query_as!(
        Category,
        r#"
        SELECT id, name, description, created_at, updated_at
        FROM categories
        WHERE user_id = ?
        ORDER BY created_at DESC
        "#,
        user_id
    )
    .fetch_all(db)
    .await?;

    Ok(categories)
}


2 / controllers/categories.rs

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
