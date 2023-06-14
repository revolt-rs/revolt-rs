//! # revolt-rs
//! one day we'll have crate-level docs for the whole project, but not today!
//! This crate simply re-exports all the other crates, for convenience
//!

/// Revolt API JSON models
pub use revolt_rs_model as model;

/// Revolt gateway event wrapper
pub use revolt_rs_gateway as gateway;

/// Revolt API HTTP wrapper
pub use revolt_rs_http as http;
