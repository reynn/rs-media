//! Errors for the Library listeners

error_chain! {
    errors {
        /// A configuration is invalid and we are unable to continue loading the manager
        InvalidListenerConfiguration(cause: String) {
            description("The provided configuration is invalid")
            display("The provided configuration is invalid: {}", cause)
        }
    }
}
