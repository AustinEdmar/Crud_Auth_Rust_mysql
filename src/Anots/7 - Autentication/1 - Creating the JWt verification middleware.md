1- agora vamos colocar as rotas envolva da autenticaçao,
vamos criar um middleware, que automaticamente pega o cabeçalho de auth, extrai o JWt
verifica e obtem o id do user, e passa as rotas autenticadas, e bloqueara requisiçoes as rotas protegidas


2 -vamos começar

criei /middleware/mod.rs e chamei main.rs
mod middleware;


<!-- use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::body::BoxBody;
use actix_web::error::{Error, ErrorUnauthorized};
use actix_web::http::header::HeaderValue;
use actix_web::middleware::Next;
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde_json::json;
use actix_web::HttpMessage;
use crate::controllers::auth::Claims;
use crate::AppState;

pub async fn verify_jwt(
    req: ServiceRequest,
    next: Next<BoxBody>,
) -> Result<ServiceResponse, Error> {

    // 1️⃣ Busca o header Authorization da requisição
    //    → porque o JWT deve vir nesse header
    let auth_header: &HeaderValue = req
        .headers()
        .get("Authorization")
        .ok_or_else(|| {
            // 2️⃣ Se não existir Authorization
            //    → significa que o usuário não está autenticado
            ErrorUnauthorized(json!({
                "status": "error",
                "message": "Unauthorized"
            }))
        })?;

    // 3️⃣ Converte o header para string
    //    → porque HeaderValue não é texto diretamente
    let auth_str = auth_header.to_str().map_err(|_| {
        // 4️⃣ Se falhar a conversão
        //    → significa que o header é inválido
        ErrorUnauthorized(json!({
            "status": "error",
            "message": "Authorization header is invalid"
        }))
    })?;

    // 5️⃣ Verifica se o header começa com "Bearer "
    //    → porque JWT padrão usa Bearer token
    if !auth_str.starts_with("Bearer ") {
        // 6️⃣ Se não for Bearer
        //    → significa formato de token inválido
        return Err(ErrorUnauthorized(json!({
            "status": "error",
            "message": "Invalid Bearer token"
        })));
    }

    // 7️⃣ Remove o prefixo "Bearer " e extrai apenas o JWT
    //    → o decode precisa somente do token puro
    let token = auth_str.strip_prefix("Bearer ").unwrap();

    // 8️⃣ Obtém o estado global da aplicação
    //    → onde está armazenado o segredo do JWT
    let state = req.app_data::<AppState>().unwrap();

    // 9️⃣ Cria a chave de decodificação usando o segredo
    //    → usada para validar a assinatura do token
    let key = DecodingKey::from_secret(state.json_web_token.as_bytes());

    // 🔟 Tenta decodificar e validar o JWT
    match decode::<Claims>(token, &key, &Validation::default()) {
        Ok(token_data) => {
            // 1️⃣1️⃣ Token válido
            //     → extrai o "sub" (normalmente o user_id)
            req.extensions_mut().insert(token_data.claims.sub);

            // 1️⃣2️⃣ Continua a execução da requisição
            //     → libera acesso ao controller
            Ok(next.call(req).await?)
        }
        Err(_) => {
            // 1️⃣3️⃣ Token inválido ou expirado
            //     → bloqueia a requisição
            Err(ErrorUnauthorized(json!({
                "status": "error",
                "message": "Invalid or expired token"
            })))
        }
    }
}
 -->