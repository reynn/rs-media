//! Integration testing will ensure the interface between the core libs work properly

#[macro_use]
extern crate error_chain;
extern crate futures_await_test;

use futures_await_test::async_test;

use zephyrs_core::events::{manager::EventManager, types::BasicEvent};

error_chain! {
    links {
        ZephyrsMediaCoreEvents(
            zephyrs_core::events::errors::Error,
            zephyrs_core::events::errors::ErrorKind
        );
    }
}

#[async_test]
async fn integration_eventmanager_event_handling() -> Result<()> {
    let event_man = EventManager::new()?;
    let event_names = vec!["test-event-1"];
    for name in event_names.iter() {
        let ev = BasicEvent::new(name)?;
        event_man.send_event(ev).await?;
    }
    Ok(())
}
