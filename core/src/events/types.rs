//!
//! RS Media Core Event Types

use super::errors::*;
use super::traits::Event;

/// A very basic event that contains little data
#[derive(Debug, Copy, Clone)]
pub struct BasicEvent<'a> {
    name: &'a str,
}

impl<'a> BasicEvent<'a> {
    /// New basic event
    /// Ok will be a valid event
    /// Err could be because of invalid naming
    pub fn new(name: &'a str) -> Result<Self> {
        Ok(Self { name })
    }
}

impl<'a> Event for BasicEvent<'a> {
    fn get_event_name(&self) -> String {
        self.name.to_string()
    }
}
