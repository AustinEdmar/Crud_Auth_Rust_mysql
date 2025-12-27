use serde::Serialize;
use sqlx::MySqlPool;
use crate::controllers::categories::CreateCategoryRequest;
use sqlx::types::chrono::{DateTime, Utc};
use crate::controllers::categories::UpdateCategoryRequest;

/* create */
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


#[derive(Debug, Serialize)]
pub struct Category {
    pub id: u64,
    pub name: String,
    pub description: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>
}



/* index */


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


 /* update */


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

/* paginaçao index */


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

