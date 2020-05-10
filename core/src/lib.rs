#![deny(
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    unused_import_braces,
    unused_allocation,
    trivial_numeric_casts
)]
#![cfg_attr(test, deny(warnings))]
//! # RS Media Core Modules

#[cfg(test)]
extern crate test_case;

#[macro_use]
extern crate error_chain;
extern crate futures;

/// # Core Auth modules
pub mod auth;
/// # Core Error modules
pub mod errors;
/// # Core HTTP modules
pub mod http;
/// # Core library modules
pub mod library;

// tests/integration_test.rs will include tests between the core modules
