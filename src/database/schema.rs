// @generated automatically by Diesel CLI.

diesel::table! {
    events (event_id) {
        event_id -> Double,
        guild_id -> Double,
        start_time -> TimestamptzSqlite,
        end_time -> Nullable<TimestamptzSqlite>,
        event_name -> Text,
        event_description -> Nullable<Text>,
        event_location -> Nullable<Text>,
        rsvps -> Integer,
    }
}
