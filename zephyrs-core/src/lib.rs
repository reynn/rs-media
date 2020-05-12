#![deny(
    missing_docs,
    rust_2018_idioms,
    missing_debug_implementations,
    trivial_casts,
    unused_import_braces,
    unused_allocation,
    trivial_numeric_casts
)]
#![cfg_attr(test, deny(warnings))]
//! # RS Media Core Modules

#[macro_use]
extern crate error_chain;

/// Core modules: Authentication
pub mod auth;
/// Core modules: Error
pub mod errors;
/// Core modules: Event
pub mod events;
/// Core modules: HTTP
pub mod http;
/// Core modules: Library
pub mod library;
/// Core modules: Logging
pub mod logging;
/// Core modules: Macros
pub mod macros;
/// Core modules: Metadata
pub mod metadata;
/// Core modules: Sorting
pub mod sorting;
