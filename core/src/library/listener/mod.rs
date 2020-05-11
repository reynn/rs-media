//!
//! # Library Listeners
//! Listeners will be responsible for listening to events and changes to library listeners
//! As an example a listener for file system events

error_chain! {}

/// Internal errors module
pub mod errors;
/// traits for listeners
pub mod traits;
