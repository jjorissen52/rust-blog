CREATE TABLE blogs (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title VARCHAR NOT NULL,
    text TEXT
);

INSERT INTO blogs (title, text) VALUES ("blog1", "some blog content");
INSERT INTO blogs (title, text) VALUES ("blog2", "some blog content again");
