//! Integration testing will ensure the interface between the core libs work properly

#[macro_use]
extern crate error_chain;

use core::events;

error_chain! {
    links {
        RSMediaCore(core::errors::Error, core::errors::ErrorKind);
        RSMediaCoreEvents(core::events::errors::Error, core::events::errors::ErrorKind);
    }
}

#[test]
fn send_event_test() -> Result<()> {
    let event_man = events::manager::EventManager::new()?;
    let event = events::types::BasicEvent::new("test-event-1")?;
    Ok(event_man.send_event(event)?)
}
