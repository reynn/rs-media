//!
//! # RS Media Core Event traits
//! An event trait is what allows any struct to be able to pass
//! through the event system

/// The most basic event type, likely holds high level data for tracking
pub trait Event {
    /// returns the name of the event
    fn get_event_name(&self) -> String;
    /// Get the event ID, should be a UUID generally
    fn get_event_id(&self) -> String;
}

/// What is an event handler?
pub trait EventHandler {
    /// Called when a valid event for the handler is sent along
    fn execute(&self);

    /// Get handleable events
    fn get_events(&self) -> Vec<String>;
}
