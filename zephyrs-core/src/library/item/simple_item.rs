use std::path::Path;

use crate::library::item::traits::LibraryItem;

error_chain! {}

/// Simple Item is the most basic a library item can be
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct SimpleItem<'a> {
    full_path: &'a Path,
}

#[allow(dead_code)]
impl<'a> SimpleItem<'a> {
    fn new(provided_path: &'a str) -> Result<Self> {
        let path: &'a Path = Path::new(provided_path);
        Ok(Self { full_path: path })
    }
}

impl<'a> LibraryItem for SimpleItem<'a> {
    fn get_item_path(&self) -> &str {
        self.full_path.to_str().unwrap()
    }
    fn is_valid(&self) -> bool {
        // crate::library::item::simple_item::
        true
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use test_case::test_case;

    error_chain!();

    #[test_case("/tmp/library/simple/item.mp4"; "basic item")]
    fn test_simple_item_get_item_path(item_file_path: &str) {
        if let Ok(item) = SimpleItem::new(item_file_path) {
            assert_eq!(item.get_item_path(), item_file_path);
        }
    }

    #[test_case("/tmp/library/simple/item.mp4"; "basic item")]
    fn test_simple_item_is_valid(item_file_path: &str) -> Result<()> {
        if let Ok(item) = SimpleItem::new(item_file_path) {
            assert_eq!(item.is_valid(), true);
        };
        Ok(())
    }
}
