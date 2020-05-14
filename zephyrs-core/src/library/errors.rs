//!
//! ZephyRS Media Core Library errors

use crate::library::item;

error_chain! {
    errors {
        /// occurs when a configuration doesn't pass validations
        InvalidCoreConfiguration(invalid_values: String) {
            display("The following values are invalid for the Managment service")
        }
    }
    links {
        LibraryItemError(item::errors::Error,
            item::errors::ErrorKind) #[doc = "Link the library item errors"];
    }
}
