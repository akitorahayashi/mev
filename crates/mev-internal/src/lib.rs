//! `mev-internal` — latency-sensitive internal command runtime for `mev`.
//!
//! This crate provides the `git` and `gh` command domains
//! invoked by `mev internal ...` through the Rust CLI boundary.

pub mod app;
mod gh;
mod git;

pub use app::cli::run as cli;
