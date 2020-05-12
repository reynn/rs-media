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
//! ZephyRS Media Manager
//!
//! provides very simple wrappers for the main binary in case it needs to split up further

// Simple and robust error handling with error-chain!
// from https://github.com/rust-lang-nursery/error-chain/blob/master/examples/quickstart.rs

#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;

// import any necessary internal manager modules
/// Configuration of the management services
pub mod config;
/// High level application errors
pub mod errors;

// import any necessary internal core modules

use errors::*;

#[tokio::main]
async fn main() {
    if let Err(ref e) = run().await {
        use error_chain::ChainedError;
        use std::io::Write; // trait which holds `display_chain`
        let stderr = &mut ::std::io::stderr();
        let errmsg = "Error writing to stderr";

        writeln!(stderr, "{}", e.display_chain()).expect(errmsg);
        ::std::process::exit(1);
    }
}

// Use this macro to auto-generate the main above. You may want to
// set the `RUST_BACKTRACE` env variable to see a backtrace.
// quick_main!(run);

// Most functions will return the `Result` type, imported from the
// `errors` module. It is a typedef of the standard `Result` type
// for which the error type is always our own `Error`.
async fn run() -> Result<()> {
    println!("Welcome to the RS Media Manager");
    // let manager_config = config::
    std::thread::sleep(std::time::Duration::new(2, 0));
    println!("RS Media Manager is shutting down");
    Ok(())
}
