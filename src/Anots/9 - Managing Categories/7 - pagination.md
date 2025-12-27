em db/category.rs
/* paginaçao */

<!-- pub async fn paginate_by_user(
    db: &MySqlPool,
    user_id: u64,
    limit: u64,
    cursor: Option<u64>, // último id recebido
) -> Result<Vec<Category>, sqlx::Error> {

    if let Some(last_id) = cursor {
        // Próxima página
        sqlx::query_as!(
            Category,
            r#"
            SELECT id, name, description,  created_at, updated_at
            FROM categories
            WHERE user_id = ? AND id < ?
            ORDER BY id DESC
            LIMIT ?
            "#,
            user_id,
            last_id,
            limit
        )
        .fetch_all(db)
        .await
    } else {
        // Primeira página
        sqlx::query_as!(
            Category,
            r#"
            SELECT id, name, description, created_at, updated_at
            FROM categories
            WHERE user_id = ?
            ORDER BY id DESC
            LIMIT ?
            "#,
            user_id,
            limit
        )
        .fetch_all(db)
        .await
    }
} -->


// em controllers/categories
<!-- #[get("/categories")]
pub async fn index(
    req: HttpRequest,
    state: web::Data<AppState>,
    query: web::Query<PaginationQuery>,
) -> impl Responder {

    let user_id = utils::get_user_id(&req);
    println!("USER_ID DO TOKEN: {}", user_id);
    let limit = query.limit.unwrap_or(6);

    match db::category::paginate_by_user(
        &state.db,
        user_id,
        limit,
        query.cursor,
    )
    .await {
        Ok(categories) => {
            let next_cursor = categories.last().map(|c| c.id);

            HttpResponse::Ok().json(serde_json::json!({
                "data": categories,
                "next_cursor": next_cursor
            }))
        }
        Err(e) => {
            eprintln!("Erro ao paginar categorias: {:?}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "status": "error",
                "message": "Erro ao listar categorias"
            }))
        }
    }
}

 -->