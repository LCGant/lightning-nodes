//! HTTP handlers organized by resource modules.
//!
//! This module re-exports route initializer functions for easy integration.

/// Defines node-related routes (`GET /nodes`).
pub mod nodes;

/// Exposes the `init_routes` function from the `nodes` module.
///
/// # Example
///
/// ```rust
/// use lightning_nodes::handlers::init_routes;
/// ```
pub use nodes::init_routes;
