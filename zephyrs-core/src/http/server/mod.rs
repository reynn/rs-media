//!
//! # RS Media Core HTTP Server

/// Wrap the warp library for now, further abstraction should be done so warp can be easily replaced
pub use warp;

/// Internal errors module
pub mod errors;

pub use errors::*;
