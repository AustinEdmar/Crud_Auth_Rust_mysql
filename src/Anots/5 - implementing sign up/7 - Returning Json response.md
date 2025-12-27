vamos retornar as respnses com http add
 HttpResponse};

use serde_json::json;
  return HttpResponse::UnprocessableEntity().json(json!({
            "status": "error",
            "message": "User already exists"
        }))


 HttpResponse::Ok().json(json!({
        "status": "success",
        "message": "User created"
    }))



<!-- final  -->

use actix_web::{ post,web, Responder, HttpResponse};
use crate::AppState;
use crate::db::user;
use serde_json::json;

#[derive(serde::Deserialize, Debug)]


pub struct SignUpRequest {
    pub email: String,
    pub password: String,
    pub firstname: String,
    pub lastname: String,
}



#[post("/auth/sign-up")]
pub async fn sign_up(state: web::Data<AppState>, data: web::Json<SignUpRequest>) -> impl Responder {
    let db = &state.db;

    if user::has_with_email(db, &data.email).await {
        return HttpResponse::UnprocessableEntity().json(json!({
            "status": "error",
            "message": "User already exists"
        }))
    }
     // funcao create do db/user.rs
    let _ = user::create(db, &data).await;

    
    HttpResponse::Ok().json(json!({
        "status": "success",
        "message": "User created"
    }))

}