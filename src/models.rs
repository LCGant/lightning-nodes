use serde::{Deserialize, Serialize};
use chrono::{SecondsFormat, TimeZone, Utc};

/// Representation of a node as received from the remote API.
#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoteNode {
    /// Node's public key identifier.
    pub public_key: String,
    /// Optional alias (may be missing).
    pub alias: Option<String>,
    /// Capacity in satoshis.
    pub capacity: u64,
    /// UNIX timestamp in seconds when the node was first seen.
    pub first_seen: i64,
}

/// Local storage model for a node.
#[derive(Clone, Serialize, sqlx::FromRow)]
pub struct Node {
    /// Node's public key.
    pub public_key: String,
    /// Resolved alias or empty string if none.
    pub alias: String,
    /// Capacity formatted as BTC string with 8 decimals.
    pub capacity: String,
    /// RFC 3339 timestamp or fallback to raw seconds string.
    pub first_seen: String,
}

impl From<RemoteNode> for Node {
    fn from(r: RemoteNode) -> Self {
        // Convert satoshis to BTC and format
        let btc = (r.capacity as f64) / 100_000_000.0;

        // Attempt RFC 3339 with 'Z'; fallback to raw seconds string
        let first_seen = Utc
            .timestamp_opt(r.first_seen, 0)
            .single()
            .map(|dt| dt.to_rfc3339_opts(SecondsFormat::Secs, true))
            .unwrap_or_else(|| r.first_seen.to_string());

        Node {
            public_key: r.public_key,
            alias: r.alias.unwrap_or_default(),
            capacity: format!("{:.8}", btc),
            first_seen,
        }
    }
}