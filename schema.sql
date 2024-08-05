DROP TABLE IF EXISTS posts;

DROP TABLE IF EXISTS threads;

CREATE TABLE IF NOT EXISTS threads (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT,
    created_at TEXT
);

CREATE TABLE IF NOT EXISTS posts (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    thread_id INTEGER REFERENCES threads(id) ON DELETE CASCADE,
    content TEXT,
    author TEXT,
    created_at TEXT
);

INSERT INTO
    threads (id, title, created_at)
VALUES
    (
        1,
        'Breaking: The pig go',
        "1970-01-01T00:00:00Z"
    ),
    (
        2,
        'Breaking: Cat stuck in tree',
        "2024-08-05T12:00:00Z"
    );

INSERT INTO
    posts (thread_id, author, content, created_at)
VALUES
    (
        1,
        'PigSpotter',
        'The pig go. Pig is go.',
        "1970-01-01T00:00:00Z"
    ),
    (
        1,
        'FountainFan',
        'Go is to the fountain. The pig put foot. Grunt. Foot in what?',
        "1970-01-01T01:00:00Z"
    ),
    (
        1,
        'CIA',
        "## Suspicious substance found
Foot in ketchup. The fountain is cover with ketchup. The pig is escape.",
        "1970-01-01T02:00:00Z"
    ),
    (
        2,
        'Joe Bloggs, cat reporter',
        'A stray cat was found _stuck_ in a tree',
        "2024-08-04T12:00:00Z"
    ),
    (
        2,
        'Jane Doe, crime desk',
        'The cat was rescued by the fire department',
        "2024-08-04T12:30:00Z"
    ),
    (
        2,
        'Joe Bloggs, cat reporter',
        'The cat was adopted by a local family',
        "2024-08-04T13:00:00Z"
    ),
    (
        2,
        'Joe Bloggs, cat reporter',
        'The cat is now happy in its new home',
        "2024-08-04T14:00:00Z"
    ),
    (
        2,
        'Jane Doe, crime desk',
        "The cat is stuck in a tree ***again***![cat stuck](https://upload.wikimedia.org/wikipedia/commons/thumb/b/bb/Kittyply_edit1.jpg/1024px-Kittyply_edit1.jpg)",
        "2024-08-04T15:00:00Z"
    );

SELECT
    *
FROM
    posts;