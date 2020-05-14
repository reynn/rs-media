//!
//! ZephyRS Media Core Event Types

use super::errors::*;
use super::traits::Event;

use std::fmt::Display;

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

impl<'a> Display for BasicEvent<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BasicEvent<{}>", self.get_event_id())
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

#[cfg(test)]
mod basic_event_test {
    use super::*;
    use test_case::test_case;

    #[test_case("event-test-1"; "Basic test")]
    fn test_basic_event_get_name(n: &str) -> Result<()> {
        let be = BasicEvent::new(n)?;
        assert_eq!(be.get_event_name(), n);
        assert_eq!(be.get_event_id(), "573a043c-fc71-41c7-a710-d8aa5966b67b");
        Ok(())
    }
}
