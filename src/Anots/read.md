$env:DATABASE_URL="mysql://root:923.Eddy@localhost:3306/moneyflow"
cargo run
cargo watch -c -x run 


sqlx database drop; sqlx database create; sqlx migrate run