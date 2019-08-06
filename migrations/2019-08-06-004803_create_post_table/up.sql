CREATE TABLE posts (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    username VARCHAR NOT NULL,
    text TEXT
);

INSERT INTO posts (username, text) VALUES ("billy123", "First!");
INSERT INTO posts (username, text) VALUES ("clippy", "uWu I'm Second!");