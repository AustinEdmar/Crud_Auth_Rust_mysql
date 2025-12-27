use actix_web::{HttpRequest, HttpMessage};
use crate::db;


pub fn get_user_id(req: &HttpRequest) -> u64 {
    let ext = req.extensions();
    ext.get::<u64>().unwrap().to_owned()
}



pub async fn get_authenticad_user(req: &HttpRequest, db: &sqlx::MySqlPool) -> db::user::User{
      db::user::get_by_id(db, get_user_id(req))   
    .await
    .unwrap()
}