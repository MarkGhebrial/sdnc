CREATE TABLE IF NOT EXISTS Events
(
    eid INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    start_time DATE NOT NULL,
    end_time DATE,
    description TEXT,
    location TEXT,
    rsvps INTEGER NOT NULL DEFAULT 0,
    discord_link TEXT NOT NULL
);