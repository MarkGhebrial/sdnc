CREATE TABLE events (
    -- Same event and guild ids used by discord
    event_id NUMERIC PRIMARY KEY NOT NULL,
    guild_id NUMERIC NOT NULL,

    -- ISO 8601 dates
    start_time TEXT NOT NULL,
    end_time TEXT,

    event_name TEXT NOT NULL,
    event_description TEXT,
    event_location TEXT,
    rsvps INTEGER NOT NULL DEFAULT 0
);