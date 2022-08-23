-- Your SQL goes here
CREATE TABLE users
(
    id         SERIAL       NOT NULL PRIMARY KEY,
    username   VARCHAR(32)  NOT NULL,
    email      VARCHAR(100) NOT NULL,
    password   VARCHAR(200) NOT NULL,
    role       INT          NOT NULL,
    is_deleted BOOLEAN      NOT NULL DEFAULT 'f',
    created_at TIMESTAMP    NOT NULL,
    deleted_at TIMESTAMP
);


INSERT INTO users (username, email, password, role, created_at)
VALUES ('root', 'admin@casbin.org', '$2y$12$ShXM69tDCu68A12bxb4uhOxytwzIErFCJygVgSkNIW4R4tAafd/iW', 0,
        '2020-07-15 14:37:40-07');