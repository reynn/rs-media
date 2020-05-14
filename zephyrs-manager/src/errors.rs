pub use zephyrs_core;

error_chain! {
    links {
        Config(crate::config::Error, crate::config::ErrorKind) #[doc = "Configuration Error"];
    }
    foreign_links {
        ZephyrsCore(zephyrs_core::errors::Error) #[doc = "Error in Core libraries"];
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    error_chain! {}

    #[test]
    pub fn test_core_errors() -> Result<()> {
        Ok(())
    }
}
