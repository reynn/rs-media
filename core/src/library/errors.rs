//!
//! RS Media Core Library errors

error_chain! {
    errors {
        /// occurs when a configuration doesn't pass validations
        InvalidCoreConfiguration(invalid_values: String) {
            display("The following values are invalid for the Managment service")
        }
    }
    // links {
    //     /// Link the library item errors
    //     LibraryItem(library::item::errors::Error, library::item::errors::ErrorKind);
    // }
}
