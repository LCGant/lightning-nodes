#![allow(clippy::unwrap_used, clippy::expect_used)]

use lightning_nodes::service::{fetch_and_store, HttpClient};
use lightning_nodes::repository::NodeRepository;
use lightning_nodes::models::{RemoteNode, Node};
use async_trait::async_trait;
use serde::de::DeserializeOwned;
use anyhow::{anyhow, Result};
use std::sync::{Arc, Mutex};

/// Spy repository capturing calls to `replace_all`.
#[derive(Clone)]
struct SpyRepo {
    called: Arc<Mutex<bool>>,
    last_nodes: Arc<Mutex<Vec<Node>>>,
}

impl SpyRepo {
    /// Creates a new spy with no calls recorded.
    fn new() -> Self {
        Self {
            called: Arc::new(Mutex::new(false)),
            last_nodes: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Returns true if `replace_all` was invoked.
    fn was_called(&self) -> Result<bool> {
        Ok(*self.called.lock().map_err(|_| anyhow!("mutex poisoned"))?)
    }

    /// Returns the nodes passed to the last `replace_all` call.
    fn nodes(&self) -> Result<Vec<Node>> {
        Ok(self.last_nodes.lock().map_err(|_| anyhow!("mutex poisoned"))?.clone())
    }
}

#[async_trait]
impl NodeRepository for SpyRepo {
    async fn list(&self) -> anyhow::Result<Vec<Node>> {
        Ok(Vec::new())
    }

    async fn replace_all(&self, nodes: Vec<Node>) -> anyhow::Result<()> {
        *self.called.lock().map_err(|_| anyhow!("mutex poisoned"))? = true;
        *self.last_nodes.lock().map_err(|_| anyhow!("mutex poisoned"))? = nodes;
        Ok(())
    }
}

/// Fake HTTP client that always returns a predefined list of `RemoteNode`.
#[derive(Clone)]
struct FakeClient {
    resp: Vec<RemoteNode>,
}

#[async_trait]
impl HttpClient for FakeClient {
    async fn get_json<T: DeserializeOwned>(&self, _url: &str) -> anyhow::Result<T> {
        let value = serde_json::to_value(&self.resp)?;
        Ok(serde_json::from_value(value)?)
    }
}

/// HTTP client that always returns an error.
struct ErrorClient;

#[async_trait]
impl HttpClient for ErrorClient {
    async fn get_json<T: DeserializeOwned>(&self, _url: &str) -> anyhow::Result<T> {
        Err(anyhow!("simulated error"))
    }
}

/// Verifies that a successful fetch calls the repository with correct node data.
#[tokio::test]
async fn fetch_and_store_success_calls_repo() -> Result<()> {
    let remote = RemoteNode {
        public_key: "pk1".to_string(),
        alias: Some("alias1".to_string()),
        capacity: 100_000_000,
        first_seen: 1_234_567_890,
    };
    let client = FakeClient { resp: vec![remote.clone()] };
    let repo = SpyRepo::new();

    fetch_and_store(&repo, &client).await?;

    assert!(repo.was_called()?);
    let nodes = repo.nodes()?;
    assert_eq!(nodes.len(), 1);
    let n = &nodes[0];
    assert_eq!(n.public_key, remote.public_key);
    assert_eq!(n.alias, remote.alias.unwrap_or_default());
    Ok(())
}

/// Ensures that errors from the HTTP client propagate and do not invoke the repository.
#[tokio::test]
async fn fetch_and_store_error_propagates() -> Result<()> {
    let client = ErrorClient;
    let repo = SpyRepo::new();

    let result = fetch_and_store(&repo, &client).await;
    assert!(result.is_err());
    assert!(!repo.was_called()?);
    Ok(())
}
