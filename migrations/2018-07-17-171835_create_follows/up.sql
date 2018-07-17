CREATE TABLE follows (
    follower_id INTEGER UNSIGNED NOT NULL,
    followed_id INTEGER UNSIGNED NOT NULL,
    created_at bigint(20) unsigned NOT NULL DEFAULT 0,
    updated_at bigint(20) unsigned NOT NULL DEFAULT 0,
    PRIMARY KEY (follower_id, followed_id),
    FOREIGN KEY (follower_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (followed_id) REFERENCES users(id) ON DELETE CASCADE
)