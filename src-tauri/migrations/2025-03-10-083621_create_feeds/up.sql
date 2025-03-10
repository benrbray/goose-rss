-- Your SQL goes here
CREATE TABLE `feeds`(
	`id` INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
	`title` TEXT NOT NULL,
	`url` TEXT NOT NULL,
	`status` TEXT NOT NULL DEFAULT "Subscribed",
	`checked_at` TIMESTAMP NOT NULL,
	`fetch_old_items` BOOL NOT NULL DEFAULT 0
);

