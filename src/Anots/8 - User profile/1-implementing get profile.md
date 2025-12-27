agora que temos o id do user, vamos implementar o get profile, para trazer o user do bd


2- 
em db/user.rs
pub async fn get_by_id(db: &sqlx::MySqlPool, id: u64) -> Option<User> {
    sqlx::query_as!(
    User,
    "SELECT * FROM users WHERE id = ?",
    id
)
.fetch_optional(db)
.await
.unwrap()

}


3 - agora volto no /bd/user adiciono
pub async fn get_by_id(db: &sqlx::MySqlPool, id: u64) -> Option<User> {
    sqlx::query_as!(
    User,
    "SELECT * FROM users WHERE id = ?",
    id
)
.fetch_optional(db)
.await
.unwrap()

}



o user tem que estar serialiado  
#[derive(Serialize)]
pub struct User {
    pub id: u64,
    pub firstname: String,
    pub lastname: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub balance: u64,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,

}


4 - volto no /me.rs
#[get("/me")]
pub async fn get_profile(state: web::Data<AppState>, req: HttpRequest) -> impl Responder {
    let db = &state.db;
   // let user_id = utils::get_user_id(&req);

    let user = db::user::get_by_id(&db, utils::get_user_id(&req)).await;

    HttpResponse::Ok().json(user)
    
}

