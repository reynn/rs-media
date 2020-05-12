//!
//! ZephyRS Media Core Event Types

use super::errors::*;
use super::traits::Event;

/// A very basic event that contains little data
#[derive(Debug, Copy, Clone)]
pub struct BasicEvent<'a> {
    name: &'a str,
    id: &'a str,
}

impl<'a> BasicEvent<'a> {
    /// New basic event
    /// Ok will be a valid event
    /// Err could be because of invalid naming
    pub fn new(name: &'a str) -> Result<Self> {
        Ok(Self {
            name,
            id: "573a043c-fc71-41c7-a710-d8aa5966b67b",
        })
    }
}

impl<'a> Event for BasicEvent<'a> {
    fn get_event_name(&self) -> String {
        self.name.to_string()
    }
    fn get_event_id(&self) -> String {
        self.id.into()
    }
}
