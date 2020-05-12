//!
//! # RS Media Core Errors
//! Links various internal error types

error_chain! {
    links {
        CoreAuthError(crate::auth::errors::Error,
            crate::auth::errors::ErrorKind) #[doc="An internal auth error"];
        CoreHTTPError(crate::http::errors::Error,
            crate::http::errors::ErrorKind) #[doc="An internal HTTP error"];
        CoreLibraryError(crate::library::errors::Error,
            crate::library::errors::ErrorKind) #[doc="An internal Library error"];
    }
}
