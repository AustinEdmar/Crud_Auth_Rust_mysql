1- get user profile concluido vamos implementar update profile

2- primeiro em db/user.rs criar fn update

pub async fn update(db: &sqlx::MySqlPool, id: u64, user: &UpdateProfileRequest){
   sqlx::query!(
    "UPDATE users SET firstname = ?, lastname = ? WHERE id = ?",
    &user.firstname,
    &user.lastname,
    id
   ).execute(db)
   .await
   .unwrap();
}

3 - volto em controllers/me.rs



/* vamos precisar de state para integir com bd */
#[post("/me")]
pub async fn update_profile(
    state: web::Data<AppState>,
    req: HttpRequest,
    data: web::Json<UpdateProfileRequest>,
) -> impl Responder {
     let db = &state.db;
    let user = db::user::get_by_id(&db, utils::get_user_id(&req))
    .await
    .unwrap();

   /* vamos em bd/user criar fn para update */  
   
   db::user::update(&db, user.id, &data).await;

   let user = db::user::get_by_id(&db, utils::get_user_id(&req))
    .await
    .unwrap();
 
    HttpResponse::Ok().json(user)
} 


