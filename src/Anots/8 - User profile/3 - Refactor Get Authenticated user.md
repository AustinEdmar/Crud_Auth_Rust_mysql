vamos refactorar tirar o codigo duplicado

vamos colocar a fn de obter o id do user em utils

pub async fn get_authenticad_user(req: &HttpRequest, db: &sqlx::MySqlPool) -> db::user::User{
      db::user::get_by_id(db, get_user_id(&req))   
    .await
    .unwrap()
}


2 - volto no /me.rs
/* vamos precisar de state para integir com bd */
#[post("/me")]
pub async fn update_profile(
    state: web::Data<AppState>,
    req: HttpRequest,
    data: web::Json<UpdateProfileRequest>,
) -> impl Responder {
     let db = &state.db;
   

  /* ao obter o user podemos fazer get_authenticad_user e passo a requisiçao e o bd  */

  let user = utils::get_authenticad_user(&req, &db).await;
   
   db::user::update(&db, user.id, &data).await;

  let user = utils::get_authenticad_user(&req, &db).await;
 
    HttpResponse::Ok().json(user)
} 