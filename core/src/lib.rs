#![deny(
    missing_docs,
    rust_2018_idioms,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    unused_import_braces,
    unused_allocation,
    trivial_numeric_casts
)]
#![cfg_attr(test, deny(warnings))]
//! # RS Media Core Modules

#[macro_use]
extern crate error_chain;

/// Core Authentication modules
pub mod auth;
/// Core Error modules
pub mod errors;
/// Core Event system
pub mod events;
/// Core HTTP modules
pub mod http;
/// Core Library modules
pub mod library;
/// Core Logging
pub mod logging;
/// Core Macros
pub mod macros;
/// Core Metadata
pub mod metadata;
/// Core Sorting
pub mod sorting;
