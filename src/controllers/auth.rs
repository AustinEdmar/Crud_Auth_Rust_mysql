use actix_web::{ post,web, Responder, HttpResponse};
use crate::AppState;
use crate::db::user;
use serde_json::json;
use bcrypt;
use jsonwebtoken::{EncodingKey, Header};
use std::time::SystemTime;

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
        "message": "User criado"
    }))

}


#[derive(serde::Deserialize, Debug)]
pub struct SignInRequest {
    pub email: String,
    pub password: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Claims {
    pub sub: u64,
    pub role: String,
    pub exp: u64,
}


#[post("/auth/sign-in")]
pub async fn sign_in(state: web::Data<AppState>, data: web::Json<SignInRequest>) -> impl Responder {
    //pegamos o db e depois o user 
    let db = &state.db;

    // esta a vir do db/user
    let Some(user) = user::get_by_email(&db, &data.email).await else {
        return HttpResponse::Unauthorized().json(json!({
            "status": "error",
            "message": "Invalid credentials"
        }))
    };
   
    if !bcrypt::verify(&data.password, &user.password).unwrap() {
        return HttpResponse::Unauthorized().json(json!({
            "status": "error",
            "message": "Invalid credentials"
        }))
    }

    let claims = Claims {
        sub: user.id,
        role: "user".to_string(),
        exp: SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs()
        + 4 * 60 * 60,
    };

    let token = jsonwebtoken::encode(&Header::default(), 
    &claims, 
    &EncodingKey::from_secret(state.json_web_token.as_bytes()),
    ).unwrap(); // definimos uma chave secreta
    //encoding criara uma chave para assinar o token
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           
    HttpResponse::Ok().json(json!({
        "status": "sucess",
        "message": "User signed in",
        "token": token
    }))
}