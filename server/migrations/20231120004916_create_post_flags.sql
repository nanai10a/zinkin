CREATE TABLE post_flags (
    id         INT8 NOT NULL,
    is_deleted BOOL NOT NULL DEFAULT FALSE,
    PRIMARY KEY (id)
);
