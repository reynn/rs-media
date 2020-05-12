//! ZephyRS Media Configuration

/// Errors for the configuration service
pub mod errors;
pub use errors::*;

/// Config struct for the media management service
#[derive(Debug, Clone)]
pub struct ManagerConfig {
    file_content: Vec<u8>,
}

impl ManagerConfig {
    /// # ZephyRS Media Manager configuration
    /// Various configuration formats will be available including TOML/YAML/JSON
    /// Configuration items will include library paths, database configuration,
    /// proxy settings, file filters, metadata configuration, etc.
    ///
    /// # Examples
    ///
    ///```
    ///# #[async_std::test]
    ///# async fn my_test() -> std::io::Result<()> {
    ///     let config = ManagerConfig::new().await?;
    ///# }
    ///```
    pub async fn new() -> Result<Self> {
        Ok(Self {
            file_content: Vec::new(),
        })
    }
    // async fn load_file(path: &str) -> Result<Vec<u8>> {
    //     Ok(Vec::new())
    //     // std::fs::read(std::path::Path::new(path))
    // }
}
