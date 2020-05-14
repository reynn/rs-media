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
//! ZephyRS Media Server
//! Allow client connection for streaming media services
//! This will likely stay a self hosted solution so users will be setting this up like Jellyfin/Kodi

fn main() {
    println!("Welcome to the RS Media Server");
    std::thread::sleep(std::time::Duration::new(2, 0));
    println!("RS Media Server is shutting down");
}
