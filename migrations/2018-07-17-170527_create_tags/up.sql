CREATE TABLE tags (
    id INTEGER UNSIGNED PRIMARY KEY AUTO_INCREMENT,
    name VARCHAR(255) NOT NULL,
    created_at bigint(20) unsigned NOT NULL DEFAULT 0,
    updated_at bigint(20) unsigned NOT NULL DEFAULT 0,
    UNIQUE KEY name (name)
);

CREATE TABLE article_tag (
    article_id INTEGER UNSIGNED NOT NULL,
    tag_id INTEGER UNSIGNED NOT NULL,
    PRIMARY KEY (article_id, tag_id),
    FOREIGN KEY (article_id) REFERENCES articles(id) ON DELETE CASCADE,
    FOREIGN KEY (tag_id) REFERENCES tags(id) ON DELETE CASCADE
);