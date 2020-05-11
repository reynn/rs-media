//!
//! # RS Media Core Event System

/// Event system error types
pub mod errors;
/// Handle events, essentially there is a many to many relationship between handlers and types
pub mod handlers;
/// Manager for handling incoming events and dispatching the handlers
pub mod manager;
/// Event traits, event types should adhere to these traits so they can pass through the system
pub mod traits;
/// Event types
pub mod types;
