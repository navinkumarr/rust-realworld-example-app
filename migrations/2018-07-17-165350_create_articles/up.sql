CREATE TABLE articles (
    id INTEGER UNSIGNED PRIMARY KEY AUTO_INCREMENT,
    user_id INTEGER UNSIGNED NOT NULL,
    slug VARCHAR(255) NOT NULL,
    title VARCHAR(255) NOT NULL,
    description VARCHAR(255) NOT NULL,
    body TEXT NOT NULL,
    created_at bigint(20) unsigned NOT NULL DEFAULT 0,
    updated_at bigint(20) unsigned NOT NULL DEFAULT 0,
    UNIQUE KEY slug (slug),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
)