//! Event traits
//! An event trait is what the

/// The most basic event type, likely holds high level data for tracking
pub trait Event {
    /// returns the name of the event
    fn get_event_name(&self) -> String;
}
