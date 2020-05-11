//!
//! RS Media Core Events Manager
//! This will be responsible for listening for events and dispatching any relevant handlers

use std::sync::mpsc::{channel, Receiver, Sender};

use super::errors::*;
use super::traits::Event;

/// Main struct for an event manager
#[derive(Debug)]
pub struct EventManager<E>
where
    E: Event,
{
    sending_channel: Sender<E>,
    receiving_channel: Receiver<E>,
}

impl<E> EventManager<E>
where
    E: Event,
{
    /// Returns a new version of an event manager
    pub fn new() -> Result<Self> {
        let channel = channel();
        Ok(Self {
            sending_channel: channel.0,
            receiving_channel: channel.1,
        })
    }

    /// Send a new event to its relevant handlers
    pub fn send_event(&self, event: E) -> Result<()> {
        self.sending_channel
            .send(event)
            .map_err(|_| ErrorKind::ChannelSendError.into())
        // Ok(())
    }
}
