CREATE TABLE posts (
    id         INTEGER NOT NULL,
    content    TEXT    NOT NULL,
    posted_at  TEXT    NOT NULL,
    created_at TEXT    NOT NULL,
    PRIMARY KEY (id, created_at)
) STRICT;
