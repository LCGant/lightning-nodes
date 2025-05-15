use crate::{config::Settings, models::{RemoteNode, Node}, repository::NodeRepository};
use serde::de::DeserializeOwned;
use async_trait::async_trait;
use tracing::{error, debug};
use tokio::time::{sleep, Duration};

/// HTTP client abstraction for fetching JSON payloads.
#[async_trait]
pub trait HttpClient {
    /// Sends a GET request to the specified URL and deserializes the JSON response into type `T`.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or if JSON deserialization fails.
    async fn get_json<T: DeserializeOwned>(&self, url: &str) -> anyhow::Result<T>;
}

/// Real HTTP client implementation using `reqwest`.
pub struct ReqwestClient;

#[async_trait]
impl HttpClient for ReqwestClient {
    async fn get_json<T: DeserializeOwned>(&self, url: &str) -> anyhow::Result<T> {
        let resp = reqwest::get(url).await?;
        let data = resp.json::<T>().await?;
        Ok(data)
    }
}

/// Remote ranking endpoint for top Lightning Network nodes.
const NODES_URL: &str =
    "https://mempool.space/api/v1/lightning/nodes/rankings/connectivity";

/// Periodically fetches node data and updates the repository.
///
/// Runs indefinitely, waiting `poll_interval_secs` between iterations.
pub async fn import_task<R, C>(repo: R, settings: Settings, client: C)
where
    R: NodeRepository,
    C: HttpClient,
{
    loop {
        match fetch_and_store(&repo, &client).await {
            Ok(_) => debug!("finished import cycle"),
            Err(e) => error!(%e, "import cycle error"),
        }
        sleep(Duration::from_secs(settings.poll_interval_secs)).await;
    }
}

/// Fetches node rankings and replaces all entries in the repository.
///
/// # Errors
///
/// Returns an error if fetching, parsing, or repository update fails.
pub async fn fetch_and_store<R, C>(repo: &R, client: &C) -> anyhow::Result<()> 
where
    R: NodeRepository,
    C: HttpClient,
{
    let resp = client.get_json::<Vec<RemoteNode>>(NODES_URL).await?;
    let nodes: Vec<Node> = resp.into_iter().map(Node::from).collect();
    repo.replace_all(nodes).await?;
    Ok(())
}
