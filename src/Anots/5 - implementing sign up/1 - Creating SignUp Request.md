cargo add serde e cargo add serde_json
 para serializar e deserializar
add serde = { version = "1.0.228", features = [ "derive" ] }
 
em Auth.rs
use actix_web::{ post,web, Responder};

#[derive(serde::Deserialize, Debug)] <!-- add -->
struct SignUpRequest {
    email: String,
    password: String,
    firstname: String,
    lastname: String,
}

#[post("/auth/sign-up")]      <!-- addd -->
pub async fn sign_up(data: web::Json<SignUpRequest>) -> impl Responder {
    format!(" Sign up: {:?}", data)
}


#[post("/auth/sign-in")]
pub async fn sign_in() -> impl Responder {
    "Sign In"
}





<!-- aula 2 teste no insomia -->
http://127.0.0.1:8080/auth/sign-up

{
  "email": "austin@gmail.com",
	"password": "123456",
	"firstname": "Austin",
	"lastname": "Edmar"
}