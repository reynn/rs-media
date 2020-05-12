//!
//! # RS Media Core Sorting
//! ## SortMethod
//! Usually passed to a SortOption variant, tells the option what
//! direction to sort the results, or to put a "shuffle" or "random"
//! ## SortOption
//! When returning results to a client application they should
//! be sorted before they are able to see them.

// Export Modules
/// Internal errors module
pub mod errors;

// import stdlib modules

// Import local crates
use errors::*;

/// How to order the sorting
#[derive(Debug, Copy, Clone)]
pub enum Order {
    /// Sort the Option in a standard -> direction
    Ascending,
    /// Sort the Option in a reverse <- direction
    Descending,
    /// Shuffle the results
    Random,
}

///
#[derive(Debug, Clone, Copy)]
pub enum SortOption {
    /// Sort By the item title alphabetically, specify the ordering
    TitleAlpha(Order),
    ///
    Year(Order),
}

impl SortOption {
    /// Returns a new sort option after parsing the provided option
    pub fn new(_parseable_option: &str) -> Result<Self> {
        Ok(Self::default())
    }
}

impl Default for SortOption {
    fn default() -> Self {
        Self::TitleAlpha(Order::Ascending)
    }
}

#[cfg(test)]
mod test {}
