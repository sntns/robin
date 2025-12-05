// lib.rs
mod commands;
mod error;
mod netlink;

pub mod client;
pub mod model;

pub use client::RobinClient;
pub use model::*;
