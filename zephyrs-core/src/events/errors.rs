//!
//! ZephyRS Media Core Event Errors

// use std::sync::mpsc::SendError;

error_chain! {
    errors {
        /// Event names should adhere to specific naming conventions
        InvalidEventName(name: String) {
            description("invalid event name")
            display("invalid event name: '{}'", name)
        }
        /// When there is a failure when sending to the channel
        ChannelSendError(msg: String) {
            description("channel send failed")
            display("failed to send to channel: {}", msg)
        }
        /// When there is a failure when sending to the channel
        ChannelReceiveError(msg: String) {
            description("channel receive failed")
            display("failed to receive from channel: {}", msg)
        }
    }
    foreign_links {
        Fmt(::std::fmt::Error) #[doc = "standard library formatting error"];
        Io(::std::io::Error) #[doc = "standard library io error"] #[cfg(linux)];
    }
}
