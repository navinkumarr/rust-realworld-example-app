CREATE TABLE comments (
    id INTEGER UNSIGNED PRIMARY KEY AUTO_INCREMENT,
    user_id INTEGER UNSIGNED NOT NULL,
    article_id INTEGER UNSIGNED NOT NULL,
    body TEXT NOT NULL,
    created_at bigint(20) unsigned NOT NULL DEFAULT 0,
    updated_at bigint(20) unsigned NOT NULL DEFAULT 0,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (article_id) REFERENCES articles(id) ON DELETE CASCADE
)