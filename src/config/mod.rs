use serde::Deserialize;
use std::env;

/// Application settings loaded from environment variables or `.env`.
#[derive(Deserialize, Clone)]
pub struct Settings {
    /// URI for the SQLite database. Defaults to `sqlite://nodes.db`.
    #[serde(default = "default_db_url")]
    pub database_url: String,

    /// Interval, in seconds, between import tasks. Defaults to 60.
    #[serde(default = "default_poll_interval")]
    pub poll_interval_secs: u64,
}

/// Default database URL when `DATABASE_URL` is not set.
fn default_db_url() -> String {
    env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite://nodes.db".into())
}

/// Default polling interval of 60 seconds.
fn default_poll_interval() -> u64 {
    60
}

impl Settings {
    /// Reads environment (`.env` file + process vars) into a `Settings` instance.
    ///
    /// # Errors
    ///
    /// Returns an error if required values cannot be deserialized.
    pub fn from_env() -> anyhow::Result<Self> {
        // Load `.env` silently if present
        dotenvy::dotenv().ok();

        // Deserialize into Settings, applying defaults
        let s = envy::from_env::<Settings>()?;
        Ok(s)
    }
}
