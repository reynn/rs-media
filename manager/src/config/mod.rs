//! RS Media Management Configuration

/// Errors for general configuration
pub mod errors;

// This is `pub use errors::*;` So the types can be accessible
// from other modules (e.g., within a `links` section)
pub use errors::*;

/// Config struct for the media management service
#[derive(Debug, Clone)]
pub struct ManagerConfig {
    file_content: Vec<u8>,
}

impl ManagerConfig {
    ///
    /// ## Examples
    /// ```rust
    ///
    /// ```
    pub async fn new() -> Self {
        Self {
            file_content: Vec::new(),
        }
    }
    // async fn load_file(path: &str) -> Result<Vec<u8>> {
    //     Ok(Vec::new())
    //     // std::fs::read(std::path::Path::new(path))
    // }
}
