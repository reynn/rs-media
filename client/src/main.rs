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
//! RS Media Client interface
//! Will include Rust bindings for the public APIs, a basic rust wrapper for an client

#[tokio::main]
async fn main() {
    println!("Welcome to the RS Media Client");
    std::thread::sleep(std::time::Duration::new(2, 0));
    println!("RS Media Client is shutting down");
}
