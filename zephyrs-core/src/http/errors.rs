//!
//! ZephyRS Media Core HTTP Errors

use crate::http::{client, server};

error_chain! {
    links {
        HTTPClientError(client::errors::Error,
            client::errors::ErrorKind) #[doc = "An internal error within the HTTP Client"];
        HTTPServerError(server::errors::Error,
            server::errors::ErrorKind) #[doc = "An internal error within the HTTP Server"];
    }
}
