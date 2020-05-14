//!
//! ZephyRS Media Core Events Manager
//! This will be responsible for listening for events and dispatching any relevant handlers

use std::fmt::{Debug, Display};
use std::sync::mpsc::{channel, Receiver, Sender};

use crate::events::errors::*;
use crate::events::traits::{Event, EventHandler};

#[allow(dead_code)]

/// Main struct for an event manager
pub struct EventManager {
    sending_channel: Sender<Box<dyn Event>>,
    receiving_channel: Receiver<Box<dyn Event>>,
    handlers: Vec<Box<dyn EventHandler>>,
}

impl Debug for EventManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Event manager is ready to handle events")
    }
}

impl Display for EventManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Event manager is ready to handle events")
    }
}

impl EventManager {
    /// Returns a new version of an event manager
    pub fn new() -> Result<Self> {
        let (send, receive) = channel();
        let s = Self {
            sending_channel: send,
            receiving_channel: receive,
            handlers: Vec::new(),
        };
        Ok(s)
    }

    // TODO: See what a better way to format this is associated types?
    /// Send a new event to its relevant handlers
    pub async fn send_event<T: Event + 'static + std::fmt::Debug + std::fmt::Display>(
        &self,
        event: T,
    ) -> Result<()> {
        println!("{}", &event);
        let b: Box<dyn Event> = Box::new(event);
        self.sending_channel
            .send(b)
            .map_err(|e| ErrorKind::ChannelSendError(e.to_string()).into())
        // self.sending_channel
        //     .send(event)
        //     .map_err(|e| ErrorKind::ChannelSendError(e.to_string()).into())
    }

    /// gets an event out of the recieving channel
    async fn get_event(&self) -> Result<Box<dyn Event>> {
        self.receiving_channel
            .recv()
            .map_err(|e| ErrorKind::ChannelReceiveError(e.to_string()).into())
    }

    /// Starts an event listener, handlers will be dispatched
    pub async fn start_event_listener(&self) -> Result<()> {
        loop {
            let event = self.get_event().await?;
            println!("Event name: {}", event.get_event_name());
        }
    }

    /// Register an event handler
    pub fn register_handler(&mut self, _handler: Box<dyn EventHandler>) -> Result<()> {
        // self.handlers.push(handler);
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use test_case::test_case;

    #[test_case("a"; "Test the resulting dsply output")]
    fn test_event_manager_dsply(_: &str) -> Result<()> {
        let event_man = EventManager::new()?;
        let event_man_dsply = event_man.to_string();
        assert_eq!(event_man_dsply, "Event manager is ready to handle events");
        Ok(())
    }
}
