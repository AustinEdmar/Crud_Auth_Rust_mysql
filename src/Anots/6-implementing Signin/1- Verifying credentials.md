1- vamos fazer a implementacao do login apos cadstro concluido
aceitaremos no corpo da requisicao contendo email e senha 
2- precisamos verificar se o email existe no banco de dados
3- apos verificar se o email existe, precisamos verificar se a senha e a senha do banco de dados sao correspondentes, precisamos verificar se o hash da senha e o hash da senha do banco de dados sao correspondentes, faremos isso uso o crate bcrypt que usamos para criar a senha 
4- apos sucesso, criaremos um JWT com dos dados do usuario e retornaremos a resposta
o user pode usar este JWT para autenticar as requisicoes, com actualizar perfil, gerenciar categorias e transactions


5 - famos criar o struct de requisicao de login, que contem os dados de requisicao 

<!-- implementacao -->

em auth.rs no signin
#[derive(serde::Deserialize, Debug)]
pub struct SignInRequest {
    pub email: String,
    pub password: String,
}

#[post("/auth/sign-in")]
pub async fn sign_in(state: web::Data<AppState>, data: web::Json<SignInRequest>) -> impl Responder {
    "Signin"
    
}

2- <!-- vamos se o user tem este email -->



em db/user.rs

pub struct User {
    pub id: u64,
    pub firstname: String,
    pub lastname: String,
    pub email: String,
    pub password: String,
    pub balance: u64,
    pub created_at: Option<DateTime<Utc>>, // 
    pub updated_at: Option<DateTime<Utc>>,

}


pub async fn get_by_email(db: &sqlx::MySqlPool, email: &str) -> Option<User> {
    sqlx::query_as!(
    User,
    "SELECT * FROM users WHERE email = ?",
    email
)
.fetch_optional(db)
.await
.unwrap()

}


<!-- 3 agora voltamos no auth user a funcao -->

<!-- 
#[derive(serde::Deserialize, Debug)]
pub struct SignInRequest {
    pub email: String,
    pub password: String,
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


    
} -->