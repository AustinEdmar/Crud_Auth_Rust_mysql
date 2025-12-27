1 - no env

DATABASE_URL=mysql:root:923.Eddy@localhost/moneyflow

2 - sqlx database --help

3 - sqlx migrate --help

4 - sqlx migrate add --help


5 - no powershell
 C:\Users\AustinEdmar\Documents\crud_mysql_rust> $env:DATABASE_URL="mysql://root:923.Eddy@localhost:3306/moneyflow"
PS C:\Users\AustinEdmar\Documents\crud_mysql_rust> sqlx database create


6 - sqlx migrate add create_user_table

<!-- 
-- Add migration script here

CREATE TABLE `users` (
    `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
    `firstname` VARCHAR(255) NOT NULL,
    `lastname` VARCHAR(255) NOT NULL,
    `email` VARCHAR(255) NOT NULL UNIQUE,
    `password` VARCHAR(255) NOT NULL,
    `balance` BIGINT UNSIGNED NOT NULL DEFAULT 0,
    `created_at` TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    `updated_at` TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    PRIMARY KEY (`id`)
);

 -->

 7 - sqlx migrate run  so ta funcionar no powershell



 