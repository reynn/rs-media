//! Traits for Library Items

/// What it means to be a library item
pub trait LibraryItem {
    /// Should return a full path to the library item for the host
    fn get_item_path(&self) -> &str;
    /// Returns if the library item is still valid
    fn is_valid(&self) -> bool;
}
