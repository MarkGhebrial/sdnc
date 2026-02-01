pub mod models;
pub mod schema;

use diesel::prelude::*;

use crate::config::CONFIG;

/// Get a connection to the database.
pub fn connect_to_database() -> SqliteConnection {
    println!("Connecting to {}", CONFIG.database_url);
    SqliteConnection::establish(&CONFIG.database_url).unwrap()
}
