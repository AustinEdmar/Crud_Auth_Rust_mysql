vamos criar uma funcao para pegar o id do user da requisição


1 - criei utils.rs
chamei no main.rs

2- vamos de volta ao utils

criar uma fn que recebera o id como parametro e retornara o id do user como u64

use actix_web::{HttpRequest, HttpMessage};
fn get_user_id(req: &HttpRequest) -> u64 {
    let ext = req.extensions();
    ext.get::<u64>().unwrap().to_owned()
}


3 - agora vamos usar no me.rs

pub async fn get_profile(req: HttpRequest) -> impl Responder {
    let user_id = utils::get_user_id(&req);

    format!("User {}", user_id)
    
} e deixamos assim, 


4 - basicamente o que fizemos é criar uma funcao aparte para nao atrelar o fn do me ao req
