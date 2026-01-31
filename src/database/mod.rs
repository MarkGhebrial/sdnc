use rusqlite::Connection;

use crate::config::CONFIG;

/// Create the database if it does not already exist. All this does is run the
/// commands in `schema.sql`.
pub fn create_database() -> rusqlite::Result<()> {
    let connection = connect_to_database()?;

    let schema_sql = include_str!("schema.sql");
    connection.execute_batch(schema_sql)?;

    Ok(())
}

/// Get a connection to the database.
pub fn connect_to_database() -> rusqlite::Result<Connection> {
    Connection::open(&CONFIG.database_path)
}
