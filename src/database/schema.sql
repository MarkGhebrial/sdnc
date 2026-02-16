CREATE TABLE IF NOT EXISTS Events (
    -- Same event and guild ids used by discord
    EventID NUMERIC PRIMARY KEY NOT NULL,
    GuildID NUMERIC NOT NULL,

    -- ISO 8601 dates
    StartTime TEXT NOT NULL,
    EndTime TEXT,

    EventName TEXT NOT NULL,
    EventDescription TEXT,
    EventLocation TEXT,
    Rsvps INTEGER NOT NULL DEFAULT 0
);