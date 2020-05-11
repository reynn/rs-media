//! Errors for the event system

// use std::sync::mpsc::SendError;

error_chain! {
    errors {
        /// Event names should adhere to specific naming conventions
        InvalidEventName(name: String) {
            description("invalid event name")
            display("invalid event name: '{}'", name)
        }
        /// When there is a failure when sending to the channel
        ChannelSendError
    }
    // foreign_links {
    //     /// standard library formatting error
    //     Fmt(std::fmt::Error);
    //     ///standard library io error
    //     Io(std::io::Error) #[cfg(unix)];
    // }
}
