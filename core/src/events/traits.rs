//!
//! # RS Media Core Event traits
//! An event trait is what allows any struct to be able to pass
//! through the event system

/// The most basic event type, likely holds high level data for tracking
pub trait Event {
    /// returns the name of the event
    fn get_event_name(&self) -> String;
}
