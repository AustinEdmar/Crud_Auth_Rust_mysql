<!-- 1 - em db/category.rs -->


/* delete */
pub async fn delete_by_id_and_user(
    db: &MySqlPool,
    category_id: u64,
    user_id: u64,
) -> Result<u64, sqlx::Error> {

    let result = sqlx::query!(
        r#"
        DELETE FROM categories
        WHERE id = ? AND user_id = ?
        "#,
        category_id,
        user_id
    )
    .execute(db)
    .await?;

    Ok(result.rows_affected())
}



<!-- 2 - em controllers/categories.rs -->

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