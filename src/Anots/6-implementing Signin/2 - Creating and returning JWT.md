1 - proximo passo criar e retornar um token web

para isso precisamos adicionar o crate jwt jsonwebtoken
cargo add jsonwebtoken 


2 - agora vamos criar uma struct que contem as informacoes do usuario e token em auth.rs
<!-- 
#[derive(serde::Deserialize, Debug)]
pub struct SignInRequest {
    pub email: String,
    pub password: String,
}

#[derive(serde::Serialize, Deserialize, Debug)]
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
        exp: (chrono::Utc::now() + chrono::Duration::days(1)).timestamp() as u64,
    };

    let token = jsonwebtoken::encode(&Header::default(), 
    &claims, 
    &EncodingKey::from_secret("".as_bytes())); // definimos uma chave secreta
    
} -->


3 - JWT_SECRET=secret no env

4 - vamos armazenar o estado da aplicacao no main como pool do db

struct AppState {
    db: sqlx::MySqlPool,
    jwt_secret: String,
}

async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    let pool = sqlx::MySqlPool::connect(&std::env::var("DATABASE_URL").unwrap())
    .await
    .unwrap();

    let state = web::Data::new(AppState { 
        db: pool,
        jwt_secret: std::env::var("JWT_SECRET").unwrap(),
    });


    5 - Agora ja esta disponivel no handlers de requisicao, agora volto no auth no asbytes

    <!-- 
    
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
    &EncodingKey::from_secret(state.json_web_token.as_bytes()),).unwrap(); // definimos uma chave secreta
    //encoding criara uma chave para assinar o token
                                                                    
    HttpResponse::Ok().json(json!({
        "status": "sucess",
        "message": "User signed in",
        "token": token
    }))
}
     -->