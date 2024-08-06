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
    ),
    (
        3,
        'Breaking: Social media on fire',
        "2024-08-06T12:00:00Z"
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
    ),
    (
        3,
        'Joe Bloggs, cat business reporter',
        '<blockquote class="twitter-tweet"><p lang="en" dir="ltr">$8.5 trillion wiped out from global stock markets but are recession fears premature? <a href="https://t.co/jovg9Dmzyn">https://t.co/jovg9Dmzyn</a></p>&mdash; ST Business Desk (@stbusinessdesk) <a href="https://twitter.com/stbusinessdesk/status/1820607996801778015?ref_src=twsrc%5Etfw">August 5, 2024</a></blockquote> <script async src="https://platform.twitter.com/widgets.js" charset="utf-8"></script>',
        "2024-08-05T12:00:00Z"
    ),
    (
        3,
        'Joe Bloggs, cat transport reporter',
        '<iframe width="560" height="315" src="https://www.youtube.com/embed/6mGPaEGlYjc?si=RYtjZv91WRFXAOuj" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerpolicy="strict-origin-when-cross-origin" allowfullscreen></iframe>',
        "2024-08-05T12:10:00Z"
    ),
    (
        3,
        'Joe Bloggs, cat finance reporter',
        '<blockquote class="tiktok-embed" cite="https://www.tiktok.com/@straitstimes/video/7399472905391770897" data-video-id="7399472905391770897" style="max-width: 605px;min-width: 325px;" > <section> <a target="_blank" title="@straitstimes" href="https://www.tiktok.com/@straitstimes?refer=embed">@straitstimes</a> Plant enthusiasts! 🗣️🪴 From a whimsical tree house to a kaleidoscopic community garden, take a tour through the mesmerising displays at the Singapore Garden Festival. It runs from Aug 3 to 11 at Suntec Singapore. 🌿 🌳 <a title="sgnews" target="_blank" href="https://www.tiktok.com/tag/sgnews?refer=embed">#sgnews</a> <a title="exploresg" target="_blank" href="https://www.tiktok.com/tag/exploresg?refer=embed">#exploresg</a> <a title="thingstodo" target="_blank" href="https://www.tiktok.com/tag/thingstodo?refer=embed">#thingstodo</a> <a title="plants" target="_blank" href="https://www.tiktok.com/tag/plants?refer=embed">#plants</a> <a title="garden" target="_blank" href="https://www.tiktok.com/tag/garden?refer=embed">#garden</a> <a title="plantlover" target="_blank" href="https://www.tiktok.com/tag/plantlover?refer=embed">#plantlover</a> <a target="_blank" title="♬ original sound  - The Straits Times" href="https://www.tiktok.com/music/original-sound-The-Straits-Times-7399473288684129040?refer=embed">♬ original sound  - The Straits Times</a> </section> </blockquote> <script async src="https://www.tiktok.com/embed.js"></script>',
        "2024-08-05T13:30:00Z"
    )
;

SELECT
    *
FROM
    posts;