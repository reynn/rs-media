//!
//! # RS Media Core Errors
//! Links various internal error types

error_chain! {
    links {
        // /// An internal auth error
        // CoreAuthError(crate::auth::errors::Error, crate::auth::errors::ErrorKind)
        // CoreHTTPError(http::Error, http::ErrorKind)
        // CoreLibraryError(library::Error, library::ErrorKind)
    }
    skip_msg_variant
}
