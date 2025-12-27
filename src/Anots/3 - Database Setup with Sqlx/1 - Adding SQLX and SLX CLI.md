cargo install sqlx-cli

2 - no cargo.toml
sqlx = { version = "0.8", features = [ "runtime-tokio", "mysql","tls-native-tls", "chrono" ] }
 

 <!-- ou 
 
 Se você instalar o SQLx CLI com suporte a dotenv, ele vai carregar o .env sozinho.

Instala assim:

cargo install sqlx-cli --features mysql,runtime-tokio-native-tls,dotenv


Depois você pode rodar:

sqlx database create


E ele vai pegar automaticamente o DATABASE_URL do .env.

 -->