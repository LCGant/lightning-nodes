use crate::models::Node;
use sqlx::SqlitePool;
use async_trait::async_trait;

/// Repository interface for node storage operations.
#[async_trait]
pub trait NodeRepository: Clone + Send + Sync + 'static {
    /// Retrieves all nodes from storage.
    ///
    /// # Errors
    ///
    /// Returns an error if the database query fails.
    async fn list(&self) -> anyhow::Result<Vec<Node>>;

    /// Replaces all stored nodes with the given list.
    ///
    /// # Errors
    ///
    /// Returns an error if deletion or insertion fails.
    async fn replace_all(&self, nodes: Vec<Node>) -> anyhow::Result<()>;
}

/// SQLite implementation of the `NodeRepository` trait.
#[derive(Clone)]
pub struct SqliteNodeRepo {
    pool: SqlitePool,
}

impl SqliteNodeRepo {
    /// Constructs a new repository backed by the given SQLite pool.
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl NodeRepository for SqliteNodeRepo {
    async fn list(&self) -> anyhow::Result<Vec<Node>> {
        let rows = sqlx::query_as::<_, Node>(
            "SELECT public_key, alias, capacity, first_seen FROM nodes",
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(rows)
    }

    async fn replace_all(&self, nodes: Vec<Node>) -> anyhow::Result<()> {
        // Clear existing records
        sqlx::query("DELETE FROM nodes")
            .execute(&self.pool)
            .await?;

        // Insert new batch
        for n in nodes {
            sqlx::query(
                "INSERT INTO nodes (public_key, alias, capacity, first_seen) VALUES (?, ?, ?, ?)"
            )
            .bind(n.public_key)
            .bind(n.alias)
            .bind(n.capacity)
            .bind(n.first_seen)
            .execute(&self.pool)
            .await?;
        }

        Ok(())
    }
}
