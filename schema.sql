DROP TABLE IF EXISTS posts;
CREATE TABLE IF NOT EXISTS posts (id INTEGER PRIMARY KEY AUTOINCREMENT, title TEXT, content TEXT, created_at TEXT);
INSERT INTO posts (id, title, content, created_at) VALUES (1, 'hello', 'world', "1970-01-01"), (4, 'goodbye', 'world', "1970-01-02"), (11, 'still alive', 'maybe', "1970-01-03");
