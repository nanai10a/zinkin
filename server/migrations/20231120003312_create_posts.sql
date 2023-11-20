CREATE TABLE posts (
    id         INT8      NOT NULL,
    content    TEXT      NOT NULL,
    posted_at  TIMESTAMP NOT NULL,
    created_at TIMESTAMP NOT NULL,
    PRIMARY KEY (id, created_at)
);
