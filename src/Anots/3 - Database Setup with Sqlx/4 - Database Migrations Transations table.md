sqlx migrate add create_transactions_table

-- Add migration script here

CREATE TABLE `transactions` (
    `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
    `user_id` BIGINT UNSIGNED NOT NULL,
    `category_id` BIGINT UNSIGNED NOT NULL,
    `type` VARCHAR(255) NOT NULL,
    `amount` BIGINT UNSIGNED NOT NULL,
    `memo` VARCHAR(255) NOT NULL, 
    `description` TEXT, 
    `created_at` TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    `updated_at` TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    PRIMARY KEY (`id`),
    FOREIGN KEY (`user_id`) REFERENCES `users` (`id`),
    FOREIGN KEY (`category_id`) REFERENCES `categories` (`id`)
);

 