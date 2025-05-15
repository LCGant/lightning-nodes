use sqlx::{SqlitePool, sqlite::{SqliteConnectOptions, SqliteJournalMode}};
use std::str::FromStr;

/// Initializes a SQLite connection pool and ensures the `nodes` table exists.
///
/// # Arguments
///
/// * `db_url` - A database URL string, e.g. `sqlite://nodes.db` or `sqlite::memory:`.
///
/// # Errors
///
/// Returns an error if the connection options are invalid, the pool cannot connect,
/// or the table creation query fails.
pub async fn init_pool(db_url: &str) -> anyhow::Result<SqlitePool> {
    // Parse connection options and enable WAL for better concurrency
    let opts = SqliteConnectOptions::from_str(db_url)?
        .create_if_missing(true)
        .journal_mode(SqliteJournalMode::Wal);

    // Establish the connection pool
    let pool = SqlitePool::connect_with(opts).await?;

    // Create the `nodes` table if it does not exist
    sqlx::query(
        r#"CREATE TABLE IF NOT EXISTS nodes (
            public_key TEXT PRIMARY KEY,
            alias      TEXT,
            capacity   TEXT,
            first_seen TEXT
        )"#,
    )
    .execute(&pool)
    .await?;

    Ok(pool)
}
