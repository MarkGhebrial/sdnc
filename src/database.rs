// use sqlx::Connection;
// use sqlx::sqlite::SqliteConnection;

use sqlx::{Pool, Sqlite, sqlite::SqlitePool, Row};

pub async fn initialize_database() -> Pool<Sqlite> {
    // Connect to the database
    let pool = SqlitePool::connect("sqlite:sdnc.db").await.unwrap();

    let id = sqlx::query_file!("src/sql/create_tables.sql");

    match id.fetch_one(&pool).await {
        Ok(row) => {
            let x: String = row.get(0);

            println!("Query row 0: {}", x);
        }
        Err(e) => println!("Error fetching row from create tables query: {}", e)
    };

    pool
}
