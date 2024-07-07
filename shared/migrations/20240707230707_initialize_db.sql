CREATE TABLE
    IF NOT EXISTS rss_items (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        title TEXT NOT NULL,
        link TEXT NOT NULL,
        description TEXT,
        pub_date TEXT,
        source TEXT
    );

CREATE TABLE
    IF NOT EXISTS feed_urls (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        url TEXT NOT NULL,
        name TEXT NOT NULL
    );

CREATE TABLE
    IF NOT EXISTS news_items (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        title TEXT NOT NULL,
        author TEXT,
        body TEXT,
        url TEXT NOT NULL
    );