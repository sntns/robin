//! Internal netlink utilities for batman-adv.
//!
//! This module exposes low-level netlink helpers for building messages, attributes, and sockets.
//! These are **internal** and only used within the crate (`pub(crate)`).

mod attribute_builder;
mod message;
mod socket;

pub(crate) use attribute_builder::*;
pub(crate) use message::*;
pub(crate) use socket::*;
