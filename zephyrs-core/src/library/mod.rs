//!
//! # RS Media Core Library Modules

/// Internal errors module
pub mod errors;
/// Item is an item that would be served to a client
/// This means it contains all metadata related to a movie, tv show/episode or other video type
pub mod item;
/// Listeners are what are responsible for finding changes in file system structure
pub mod listener;
