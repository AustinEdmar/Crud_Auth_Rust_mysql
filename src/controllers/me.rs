use actix_web::{ get, post, Responder};
use actix_web::HttpRequest;
use actix_web::HttpResponse;
use actix_web::web;
use crate::AppState;
use crate::utils;
use crate::db;


#[get("/me")]
pub async fn get_profile(state: web::Data<AppState>, req: HttpRequest) -> impl Responder {
    let db = &state.db;
   // let user_id = utils::get_user_id(&req);

    let user = db::user::get_by_id(&db, utils::get_user_id(&req)).await;

    HttpResponse::Ok().json(user)
    
}

/* deserializa */
#[derive(serde::Deserialize, Debug)]
pub struct UpdateProfileRequest {
    pub firstname: String,
    pub lastname: String,
    pub email: String,

}

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

