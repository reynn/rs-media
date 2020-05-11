//! Integration testing will ensure the interface between the core libs work properly

#[macro_use]
extern crate error_chain;

use core::events::{manager::EventManager, types::BasicEvent};

error_chain! {
    links {
        RSMediaCoreEvents(core::events::errors::Error, core::events::errors::ErrorKind);
    }
}

#[test]
fn integration_eventmanager_send_event() -> Result<()> {
    let event_man = EventManager::new()?;
    let event = BasicEvent::new("test-event-1")?;
    Ok(event_man.send_event(event)?)
}
