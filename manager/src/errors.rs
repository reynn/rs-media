//! RS Media Errors module

error_chain! {
    // links {
    //     /// Link in the chain
    //     ConfigError(config::errors::Error, config::errors::ErrorKind);
    // }
    errors {
        /// Unknown error
        UnknownError {
            description("An unknown error occured, please report the issue on GitHub")
        }
    }
}
