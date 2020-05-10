//! RS Media Sorting

/// Internal errors module
pub mod errors;

pub enum SortDirection {
    Ascending,
    Descending,
}

pub enum SortOption {
    AlphaNumerical(SortDirection),
    Year(SortDirection),
}

#[cfg(test)]
mod test {
    use super::*;
}
